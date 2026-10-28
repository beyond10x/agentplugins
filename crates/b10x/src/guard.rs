//! `b10x guard bash`: a Claude Code `PreToolUse` hook that refuses the shell commands an operator's
//! written rules forbid, naming the replacement.
//!
//! [`verdict`] is pure: it parses the command with the Bash grammar and checks each simple command
//! against the groups the [`Config`] switches on. [`run`] is the hook around it: one event on stdin,
//! exit 0 to allow, exit 2 with one stderr line to refuse, and exit 0 on anything it cannot read.
//!
//! The configuration is `$B10X_GUARD`, else `$XDG_CONFIG_HOME/b10x/guard.toml`, else
//! `~/.config/b10x/guard.toml`:
//!
//! ```toml
//! [gh_writes]
//! enabled = true
//! orgs = ["example-org"]
//! workspace_roots = ["~/work"]
//!
//! [tmp_writes]
//! enabled = true
//! ```
//!
//! Without a file, `read_dev_zero`, `pgrep_loops` and `heredoc_stdin` are on and the other groups
//! are off.

use std::ffi::OsString;
use std::io::Read;
use std::path::{Component, Path, PathBuf};
use std::process::ExitCode;

use serde::Deserialize;
use tree_sitter::{Node, Parser};

/// A rule group: one rule from the operator's instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Group {
    /// GitHub writes with the personal `gh` login, in a configured organization.
    GhWrites,
    /// Writes under `/tmp` or `/dev/shm`.
    TmpWrites,
    /// `read -t` fed from `/dev/zero`.
    ReadDevZero,
    /// `pkill -f`, and `pgrep -f` in a loop.
    PgrepLoops,
    /// A heredoc or here-string into a stdin reader; `python -c "…"` holding a backtick.
    HeredocStdin,
    /// `git commit -m`.
    CommitMessageFlag,
}

impl Group {
    /// Every group, in the order the configuration lists them.
    pub const ALL: [Group; 6] = [
        Group::GhWrites,
        Group::TmpWrites,
        Group::ReadDevZero,
        Group::PgrepLoops,
        Group::HeredocStdin,
        Group::CommitMessageFlag,
    ];

    /// The group's name in the configuration and in a refusal.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Group::GhWrites => "gh_writes",
            Group::TmpWrites => "tmp_writes",
            Group::ReadDevZero => "read_dev_zero",
            Group::PgrepLoops => "pgrep_loops",
            Group::HeredocStdin => "heredoc_stdin",
            Group::CommitMessageFlag => "commit_message_flag",
        }
    }

    /// The refusal, naming what to do instead.
    #[must_use]
    pub fn message(self) -> &'static str {
        match self {
            Group::GhWrites => "GitHub writes go through the bot: `b10x-gates api --method … --path … --input f --output f --repository <owner>/<repo> --policy <policy>`",
            Group::TmpWrites => "do not write under /tmp or /dev/shm (per-user quota): use $TMPDIR or a directory under ~/.cache",
            Group::ReadDevZero => "`read -t` on /dev/zero never times out in zsh and dumps core: wait with `until grep -q '^EXIT=' <file>; do timeout 5 tail -f /dev/null; done`",
            Group::PgrepLoops => "`pgrep -f`/`pkill -f` match the waiting shell itself: wait on the PID (`wait $!`, `kill -0 $PID`) or on an output file",
            Group::HeredocStdin => "a heredoc into a stdin reader has hung a session for 22 h: write the script with the Write tool and run it by path",
            Group::CommitMessageFlag => "write the message to a `mktemp` file and `git commit -F <file>`",
        }
    }

    /// Whether the group is on when no configuration file exists: the ones that only prevent
    /// hangs and crashes.
    #[must_use]
    pub fn on_by_default(self) -> bool {
        matches!(
            self,
            Group::ReadDevZero | Group::PgrepLoops | Group::HeredocStdin
        )
    }
}

/// What the guard decided about one command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// Run it.
    Allow,
    /// Refuse it, for this group's rule.
    Refuse(Group),
}

/// The switched-on groups and what `gh_writes` applies to.
#[derive(Debug, Clone, Default)]
pub struct Config {
    enabled: [bool; 6],
    /// Organizations whose repositories take writes only through the bot.
    pub orgs: Vec<String>,
    /// A `gh` command run below one of these directories counts as one of `orgs`' repositories.
    pub workspace_roots: Vec<PathBuf>,
    /// `$HOME`, for `cd ~`; unknown leaves the directory unknown.
    pub home: Option<PathBuf>,
}

impl Config {
    /// The configuration with no file: only the hang and crash guards.
    #[must_use]
    pub fn defaults(home: Option<PathBuf>) -> Self {
        let mut config = Config {
            home,
            ..Config::default()
        };
        for group in Group::ALL {
            config.set(group, group.on_by_default());
        }
        config
    }

    /// Every group on.
    #[cfg(test)]
    #[must_use]
    pub fn all(orgs: Vec<String>, workspace_roots: Vec<PathBuf>, home: Option<PathBuf>) -> Self {
        Config {
            enabled: [true; 6],
            orgs,
            workspace_roots,
            home,
        }
    }

    /// Whether a group is on.
    #[must_use]
    pub fn on(&self, group: Group) -> bool {
        self.enabled[group as usize]
    }

    fn set(&mut self, group: Group, on: bool) {
        self.enabled[group as usize] = on;
    }

    /// Parse a configuration file's text; `~/` in a workspace root is `home`.
    pub fn parse(text: &str, home: Option<PathBuf>) -> Result<Self, String> {
        let file: File = toml::from_str(text).map_err(|error| error.message().to_owned())?;
        let mut config = Config::defaults(home);
        let toggles = [
            (Group::TmpWrites, file.tmp_writes),
            (Group::ReadDevZero, file.read_dev_zero),
            (Group::PgrepLoops, file.pgrep_loops),
            (Group::HeredocStdin, file.heredoc_stdin),
            (Group::CommitMessageFlag, file.commit_message_flag),
        ];
        for (group, toggle) in toggles {
            if let Some(enabled) = toggle.and_then(|toggle| toggle.enabled) {
                config.set(group, enabled);
            }
        }
        if let Some(gh) = file.gh_writes {
            if let Some(enabled) = gh.enabled {
                config.set(Group::GhWrites, enabled);
            }
            config.orgs = gh.orgs;
            config.workspace_roots = gh
                .workspace_roots
                .iter()
                .map(|root| expand_home(root, config.home.as_deref()))
                .collect();
        }
        Ok(config)
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct File {
    gh_writes: Option<GhSection>,
    tmp_writes: Option<Toggle>,
    read_dev_zero: Option<Toggle>,
    pgrep_loops: Option<Toggle>,
    heredoc_stdin: Option<Toggle>,
    commit_message_flag: Option<Toggle>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Toggle {
    enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GhSection {
    enabled: Option<bool>,
    #[serde(default)]
    orgs: Vec<String>,
    #[serde(default)]
    workspace_roots: Vec<String>,
}

fn expand_home(path: &str, home: Option<&Path>) -> PathBuf {
    match (path.strip_prefix("~/"), home) {
        (Some(rest), Some(home)) => home.join(rest),
        _ if path == "~" => home.map_or_else(|| PathBuf::from(path), Path::to_path_buf),
        _ => PathBuf::from(path),
    }
}

/// The configuration file: `b10x_guard` (`$B10X_GUARD`) when set, else under `xdg`
/// (`$XDG_CONFIG_HOME`), else under `home/.config`.
#[must_use]
pub fn config_path(home: &Path, b10x_guard: Option<OsString>, xdg: Option<OsString>) -> PathBuf {
    if let Some(named) = b10x_guard.filter(|value| !value.is_empty()) {
        return PathBuf::from(named);
    }
    match xdg.filter(|value| !value.is_empty()) {
        Some(config) => PathBuf::from(config).join("b10x/guard.toml"),
        None => home.join(".config/b10x/guard.toml"),
    }
}

/// Read the configuration. An absent default file is the defaults; a file named by `$B10X_GUARD`
/// that is absent, or any file that cannot be read or parsed, is an error.
fn load(home: Option<PathBuf>) -> Result<Config, String> {
    let named = std::env::var_os("B10X_GUARD").filter(|value| !value.is_empty());
    let explicit = named.is_some();
    let base = home.clone().unwrap_or_default();
    let path = config_path(&base, named, std::env::var_os("XDG_CONFIG_HOME"));
    match std::fs::read_to_string(&path) {
        Ok(text) => Config::parse(&text, home)
            .map_err(|error| format!("{}: {}", path.display(), error.lines().next().unwrap_or(""))),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && !explicit => {
            Ok(Config::defaults(home))
        }
        Err(error) => Err(format!("{}: {error}", path.display())),
    }
}

/// The hook: read one `PreToolUse` event on stdin and answer it. Never fails closed.
pub fn run() -> ExitCode {
    let mut input = String::new();
    if std::io::stdin().read_to_string(&mut input).is_err() {
        return ExitCode::SUCCESS;
    }
    let Ok(event) = serde_json::from_str::<serde_json::Value>(&input) else {
        return ExitCode::SUCCESS;
    };
    if event.get("tool_name").and_then(serde_json::Value::as_str) != Some("Bash") {
        return ExitCode::SUCCESS;
    }
    let Some(command) = event
        .pointer("/tool_input/command")
        .and_then(serde_json::Value::as_str)
    else {
        return ExitCode::SUCCESS;
    };
    let cwd = event
        .get("cwd")
        .and_then(serde_json::Value::as_str)
        .map(PathBuf::from)
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_default();
    let home = std::env::var_os("HOME").map(PathBuf::from);
    let config = match load(home) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("b10x guard: {error}; nothing is guarded until it is fixed");
            return ExitCode::SUCCESS;
        }
    };
    std::panic::set_hook(Box::new(|_| {}));
    match std::panic::catch_unwind(|| verdict(command, &cwd, &config)) {
        Ok(Verdict::Refuse(group)) => {
            eprintln!("b10x guard ({}): {}", group.name(), group.message());
            ExitCode::from(2)
        }
        Ok(Verdict::Allow) | Err(_) => ExitCode::SUCCESS,
    }
}

/// Decide one command run in `cwd`.
#[must_use]
pub fn verdict(command: &str, cwd: &Path, config: &Config) -> Verdict {
    if !Group::ALL.iter().any(|group| config.on(*group)) {
        return Verdict::Allow;
    }
    let mut walker = Walker {
        config,
        cwd: Some(cwd.to_path_buf()),
        found: None,
        mentions_dev_zero: command.contains("/dev/zero"),
        pending: None,
    };
    walker.script(command, 0);
    walker.found.map_or(Verdict::Allow, Verdict::Refuse)
}

/// How deep `bash -c '…'` is followed.
const MAX_DEPTH: usize = 3;

struct Walker<'c> {
    config: &'c Config,
    /// The directory the next command runs in, as far as `cd` can be followed.
    cwd: Option<PathBuf>,
    found: Option<Group>,
    mentions_dev_zero: bool,
    /// The command that receives the redirections of the statement being visited.
    pending: Option<(usize, Extra)>,
}

#[derive(Debug, Clone, Copy, Default)]
struct Ctx {
    in_loop: bool,
    depth: usize,
}

/// A redirection: its operator and its target, unquoted.
#[derive(Debug, Clone)]
struct Redirect {
    op: String,
    target: String,
}

impl Redirect {
    fn writes(&self) -> bool {
        !self.op.starts_with('<') || self.op == "<>"
    }
}

/// What feeds a command's stdin from the command text itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Feed {
    Heredoc,
    HereString,
}

#[derive(Debug, Default)]
struct Extra {
    redirects: Vec<Redirect>,
    feed: Option<Feed>,
    words: Vec<String>,
}

/// One simple command as it will run: its words with wrappers (`env`, `timeout`, …) removed.
struct Simple<'s, 't> {
    words: Vec<(String, Option<Node<'t>>)>,
    source: &'s str,
    redirects: &'s [Redirect],
    feed: Option<Feed>,
    in_loop: bool,
    /// Run by `b10x-gates` as the organization bot.
    via_bot: bool,
}

impl Simple<'_, '_> {
    fn word(&self, index: usize) -> Option<&str> {
        self.words.get(index).map(|(word, _)| word.as_str())
    }

    fn program(&self) -> &str {
        self.word(0).map_or("", |word| {
            Path::new(word)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(word)
        })
    }

    fn args(&self) -> impl Iterator<Item = &str> {
        self.words.iter().skip(1).map(|(word, _)| word.as_str())
    }
}

impl Walker<'_> {
    fn script(&mut self, text: &str, depth: usize) {
        let mut parser = Parser::new();
        if parser
            .set_language(&tree_sitter_bash::LANGUAGE.into())
            .is_err()
        {
            return;
        }
        let Some(tree) = parser.parse(text, None) else {
            return;
        };
        self.visit(
            tree.root_node(),
            text,
            Ctx {
                in_loop: false,
                depth,
            },
        );
    }

    fn visit(&mut self, node: Node<'_>, src: &str, ctx: Ctx) {
        if self.found.is_some() {
            return;
        }
        match node.kind() {
            "while_statement" => {
                self.children(
                    node,
                    src,
                    Ctx {
                        in_loop: true,
                        ..ctx
                    },
                );
            }
            // A `for` list runs once; only its body repeats.
            "for_statement" | "c_style_for_statement" => {
                let mut cursor = node.walk();
                for child in node.named_children(&mut cursor) {
                    let in_loop = ctx.in_loop || child.kind() == "do_group";
                    self.visit(child, src, Ctx { in_loop, ..ctx });
                }
            }
            "redirected_statement" => self.redirected(node, src, ctx),
            "command" => {
                let extra = match self.pending.take() {
                    Some((id, extra)) if id == node.id() => extra,
                    other => {
                        self.pending = other;
                        Extra::default()
                    }
                };
                self.command(node, src, ctx, extra);
            }
            "file_redirect" => {
                let redirect = redirect(node, src);
                self.check_redirect(&redirect);
                self.children(node, src, ctx);
            }
            _ => self.children(node, src, ctx),
        }
    }

    fn children(&mut self, node: Node<'_>, src: &str, ctx: Ctx) {
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            self.visit(child, src, ctx);
        }
    }

    fn refuse(&mut self, group: Group) {
        if self.found.is_none() && self.config.on(group) {
            self.found = Some(group);
        }
    }

    fn check_redirect(&mut self, redirect: &Redirect) {
        if redirect.writes() && is_tmp(&redirect.target) {
            self.refuse(Group::TmpWrites);
        }
    }

    fn redirected(&mut self, node: Node<'_>, src: &str, ctx: Ctx) {
        let body = node.child_by_field_name("body");
        let mut extra = Extra::default();
        let mut rest = Vec::new();
        let mut first_redirect = None;
        let mut cursor = node.walk();
        for child in node.children_by_field_name("redirect", &mut cursor) {
            first_redirect.get_or_insert(child.start_byte());
            match child.kind() {
                "file_redirect" => extra.redirects.push(redirect(child, src)),
                "heredoc_redirect" => {
                    extra.feed = Some(Feed::Heredoc);
                    let mut inner = child.walk();
                    for part in child.named_children(&mut inner) {
                        match part.kind() {
                            "file_redirect" => extra.redirects.push(redirect(part, src)),
                            "heredoc_start" | "heredoc_end" => {}
                            _ => rest.push(part),
                        }
                    }
                }
                "herestring_redirect" => {
                    extra.feed = Some(Feed::HereString);
                    rest.push(child);
                }
                _ => rest.push(child),
            }
        }
        for redirect in &extra.redirects {
            self.check_redirect(redirect);
        }
        let mut cat_feeds_pipe = false;
        let outer = self.pending.take();
        // The grammar hangs `a && b <<EOF` on the list; the shell gives the redirection to `b`.
        if let Some(body) = body {
            if let Some(target) = last_command(body) {
                // The grammar drops a word between the command and the first redirection
                // (`python3 - <<EOF` parses as `python3`); read it back from the text.
                if let Some(end) = first_redirect.filter(|end| *end >= target.end_byte()) {
                    extra.words = split(src.get(target.end_byte()..end).unwrap_or(""));
                }
                cat_feeds_pipe = extra.feed.is_some()
                    && extra.redirects.iter().all(|redirect| !redirect.writes())
                    && command_name(target, src).as_deref() == Some("cat");
                self.pending = Some((target.id(), extra));
            }
            self.visit(body, src, ctx);
            self.pending = outer;
        }
        for part in rest {
            if cat_feeds_pipe && part.kind() == "pipeline" {
                self.piped(part, src, ctx);
            } else {
                self.visit(part, src, ctx);
            }
        }
    }

    /// The pipeline after `cat <<EOF`: its first command reads the heredoc.
    fn piped(&mut self, pipeline: Node<'_>, src: &str, ctx: Ctx) {
        let mut cursor = pipeline.walk();
        let mut first = true;
        for child in pipeline.named_children(&mut cursor) {
            if first && child.kind() == "command" {
                first = false;
                let extra = Extra {
                    feed: Some(Feed::Heredoc),
                    ..Extra::default()
                };
                self.command(child, src, ctx, extra);
            } else {
                first = false;
                self.visit(child, src, ctx);
            }
        }
    }

    fn command(&mut self, node: Node<'_>, src: &str, ctx: Ctx, mut extra: Extra) {
        let mut words: Vec<(String, Option<Node<'_>>)> = Vec::new();
        if let Some(name) = node.child_by_field_name("name") {
            words.push((unquote(text(name, src)), Some(name)));
        }
        let mut cursor = node.walk();
        for argument in node.children_by_field_name("argument", &mut cursor) {
            words.push((unquote(text(argument, src)), Some(argument)));
        }
        let mut cursor = node.walk();
        for child in node.children_by_field_name("redirect", &mut cursor) {
            match child.kind() {
                "file_redirect" => {
                    let redirect = redirect(child, src);
                    self.check_redirect(&redirect);
                    extra.redirects.push(redirect);
                }
                "herestring_redirect" => extra.feed = Some(Feed::HereString),
                "heredoc_redirect" => extra.feed = Some(Feed::Heredoc),
                _ => {}
            }
        }
        words.extend(extra.words.drain(..).map(|word| (word, None)));
        let (words, via_bot) = unwrap(words);
        let simple = Simple {
            words,
            source: src,
            redirects: &extra.redirects,
            feed: extra.feed,
            in_loop: ctx.in_loop,
            via_bot,
        };
        self.check(&simple, ctx);
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            if child.kind() != "file_redirect" {
                self.visit(child, src, ctx);
            }
        }
    }

    fn check(&mut self, simple: &Simple<'_, '_>, ctx: Ctx) {
        let program = simple.program().to_owned();
        if let Some(script) = shell_script(simple) {
            if ctx.depth < MAX_DEPTH {
                let saved = self.cwd.clone();
                self.script(&script, ctx.depth + 1);
                self.cwd = saved;
            }
        }
        if program == "cd" || program == "pushd" {
            let target = simple.args().find(|arg| !arg.starts_with('-'));
            if target.is_some_and(is_tmp) {
                self.refuse(Group::TmpWrites);
            }
            self.cwd = self.change_directory(target);
            return;
        }
        if program == "gh" && !simple.via_bot && self.gh_applies(simple) && gh_writes(simple) {
            self.refuse(Group::GhWrites);
        }
        if tmp_write(simple) {
            self.refuse(Group::TmpWrites);
        }
        if program == "read" && self.mentions_dev_zero && read_with_timeout(simple) {
            self.refuse(Group::ReadDevZero);
        }
        if program == "pkill" && full_match(simple) {
            self.refuse(Group::PgrepLoops);
        }
        if program == "pgrep" && simple.in_loop && full_match(simple) {
            self.refuse(Group::PgrepLoops);
        }
        if (simple.feed.is_some() && reads_stdin(simple)) || python_backtick(simple) {
            self.refuse(Group::HeredocStdin);
        }
        if program == "git" && commit_message(simple) {
            self.refuse(Group::CommitMessageFlag);
        }
    }

    fn change_directory(&self, target: Option<&str>) -> Option<PathBuf> {
        let home = self.config.home.clone();
        let next = match target {
            None | Some("~") => home?,
            Some(path) if path.starts_with("~/") => expand_home(path, home.as_deref()),
            Some(path) if path.starts_with('/') => PathBuf::from(path),
            Some(path) if path.contains('$') || path == "-" => return None,
            Some(path) => self.cwd.as_ref()?.join(path),
        };
        Some(normalize(&next))
    }

    /// Whether `gh_writes` covers this `gh` command: its `--repo` or API path names a configured
    /// organization, or it names none and runs below a configured workspace root.
    fn gh_applies(&self, simple: &Simple<'_, '_>) -> bool {
        let config = self.config;
        if config.orgs.is_empty() {
            return false;
        }
        if let Some(org) = gh_org(simple) {
            return config
                .orgs
                .iter()
                .any(|known| known.eq_ignore_ascii_case(&org));
        }
        self.cwd.as_ref().is_some_and(|cwd| {
            config
                .workspace_roots
                .iter()
                .any(|root| cwd.starts_with(normalize(root)))
        })
    }
}

fn text<'s>(node: Node<'_>, src: &'s str) -> &'s str {
    src.get(node.byte_range()).unwrap_or("")
}

/// A word as the shell passes it, when that is knowable from the text; the text itself otherwise.
fn unquote(raw: &str) -> String {
    match shlex::split(raw) {
        Some(mut words) if words.len() == 1 => words.remove(0),
        _ => raw.to_owned(),
    }
}

fn split(raw: &str) -> Vec<String> {
    shlex::split(raw).unwrap_or_default()
}

/// The simple command a statement's trailing redirections belong to: the statement itself, or
/// the last command of a list or pipeline.
fn last_command(node: Node<'_>) -> Option<Node<'_>> {
    match node.kind() {
        "command" => Some(node),
        "list" | "pipeline" => {
            let count = node.named_child_count();
            node.named_child(count.checked_sub(1)?)
                .and_then(last_command)
        }
        _ => None,
    }
}

fn command_name(node: Node<'_>, src: &str) -> Option<String> {
    node.child_by_field_name("name")
        .map(|name| unquote(text(name, src)))
}

fn redirect(node: Node<'_>, src: &str) -> Redirect {
    let mut cursor = node.walk();
    let op = node
        .children(&mut cursor)
        .find(|child| !child.is_named())
        .map(|child| text(child, src).to_owned())
        .unwrap_or_default();
    let target = node
        .child_by_field_name("destination")
        .map(|target| unquote(text(target, src)))
        .unwrap_or_default();
    Redirect { op, target }
}

fn normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::ParentDir => {
                out.pop();
            }
            Component::CurDir => {}
            other => out.push(other),
        }
    }
    out
}

fn is_tmp(path: &str) -> bool {
    ["/tmp", "/dev/shm"].iter().any(|root| {
        path.strip_prefix(root)
            .is_some_and(|rest| rest.is_empty() || rest.starts_with('/'))
    })
}

/// Remove the wrappers that run another command unchanged, and say whether the organization bot
/// runs it (`b10x-gates bot … -- <command>`).
fn unwrap(mut words: Vec<(String, Option<Node<'_>>)>) -> (Vec<(String, Option<Node<'_>>)>, bool) {
    let mut via_bot = false;
    for _ in 0..8 {
        let Some((first, _)) = words.first() else {
            break;
        };
        let program = Path::new(first)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(first)
            .to_owned();
        let skip = match program.as_str() {
            "env" => {
                1 + words[1..]
                    .iter()
                    .take_while(|(word, _)| word.starts_with('-') || word.contains('='))
                    .count()
            }
            "nohup" | "command" | "exec" | "time" | "builtin" | "noglob" => 1,
            "sudo" | "stdbuf" | "nice" | "ionice" => {
                1 + words[1..]
                    .iter()
                    .take_while(|(word, _)| word.starts_with('-'))
                    .count()
            }
            "timeout" => {
                let flags = words[1..]
                    .iter()
                    .take_while(|(word, _)| word.starts_with('-'))
                    .count();
                2 + flags
            }
            "b10x-gates" => match words.iter().position(|(word, _)| word == "--") {
                Some(dash) => {
                    via_bot = true;
                    dash + 1
                }
                None => break,
            },
            _ => break,
        };
        if skip >= words.len() {
            return (Vec::new(), via_bot);
        }
        words.drain(..skip);
    }
    (words, via_bot)
}

/// The script of `bash -c '<script>'` (or `sh`, `zsh`, `eval`), to check as well.
fn shell_script(simple: &Simple<'_, '_>) -> Option<String> {
    let program = simple.program();
    if program == "eval" {
        return Some(simple.args().collect::<Vec<_>>().join(" "));
    }
    if !matches!(program, "bash" | "sh" | "zsh" | "dash") {
        return None;
    }
    let mut args = simple.args();
    while let Some(arg) = args.next() {
        if !arg.starts_with('-') || arg.starts_with("--") {
            return None;
        }
        if arg.contains('c') {
            return args.next().map(str::to_owned);
        }
    }
    None
}

fn gh_org(simple: &Simple<'_, '_>) -> Option<String> {
    let args: Vec<&str> = simple.args().collect();
    let mut index = 0;
    while let Some(arg) = args.get(index) {
        let repo = match *arg {
            "-R" | "--repo" => args.get(index + 1).copied(),
            other => other
                .strip_prefix("--repo=")
                .or_else(|| other.strip_prefix("-R").filter(|value| !value.is_empty())),
        };
        if let Some(repo) = repo {
            let parts: Vec<&str> = repo.split('/').collect();
            let org = if parts.len() >= 3 { parts[1] } else { parts[0] };
            return Some(org.to_owned());
        }
        index += 1;
    }
    let words = positionals(&args, GH_VALUE_FLAGS);
    match words.as_slice() {
        ["api", path, ..] => {
            let mut segments = path.trim_start_matches('/').split('/');
            if matches!(segments.next(), Some("repos" | "orgs")) {
                return segments.next().map(str::to_owned);
            }
            None
        }
        ["repo", _, name, ..] if name.contains('/') => name.split('/').next().map(str::to_owned),
        _ => None,
    }
}

/// `gh` flags (global, and `gh api`'s) whose value is the next word.
const GH_VALUE_FLAGS: &[&str] = &[
    "-R",
    "--repo",
    "-X",
    "--method",
    "-H",
    "--header",
    "-f",
    "--raw-field",
    "-F",
    "--field",
    "--input",
    "-q",
    "--jq",
    "-t",
    "--template",
    "--hostname",
    "--cache",
    "-p",
    "--preview",
];

/// The words that are not flags or a flag's value, up to the first `--title`-style flag whose
/// value is unknown: enough to find `gh`'s subcommand and the API path.
fn positionals<'a>(args: &[&'a str], value_flags: &[&str]) -> Vec<&'a str> {
    let mut out = Vec::new();
    let mut index = 0;
    while let Some(arg) = args.get(index) {
        if value_flags.contains(arg) {
            index += 2;
            continue;
        }
        if !arg.starts_with('-') {
            out.push(*arg);
        }
        index += 1;
    }
    out
}

fn gh_writes(simple: &Simple<'_, '_>) -> bool {
    let args: Vec<&str> = simple.args().collect();
    let words = positionals(&args, GH_VALUE_FLAGS);
    let (Some(area), verb) = (words.first().copied(), words.get(1).copied()) else {
        return false;
    };
    let verb = verb.unwrap_or("");
    match area {
        "api" => gh_api_writes(&args, verb),
        "pr" => matches!(
            verb,
            "create" | "comment" | "edit" | "merge" | "close" | "review" | "ready" | "reopen"
        ),
        "issue" => matches!(
            verb,
            "create" | "comment" | "edit" | "close" | "delete" | "reopen" | "transfer"
        ),
        "release" => matches!(
            verb,
            "create" | "edit" | "delete" | "upload" | "delete-asset"
        ),
        "run" => matches!(verb, "rerun" | "cancel" | "delete"),
        "workflow" => matches!(verb, "run" | "enable" | "disable"),
        "secret" | "variable" => matches!(verb, "set" | "delete"),
        "repo" => matches!(
            verb,
            "create" | "delete" | "edit" | "archive" | "unarchive" | "rename"
        ),
        "label" => matches!(verb, "create" | "edit" | "delete" | "clone"),
        _ => false,
    }
}

/// `gh api` writes with a method other than GET, or without one when it sends fields or a body
/// (which makes it a POST); a GraphQL call writes when it sends a mutation.
fn gh_api_writes(args: &[&str], path: &str) -> bool {
    let mut method = None;
    let mut body = false;
    let mut index = 0;
    while let Some(arg) = args.get(index) {
        match *arg {
            "-X" | "--method" => method = args.get(index + 1).map(|m| m.to_ascii_uppercase()),
            "-f" | "-F" | "--field" | "--raw-field" | "--input" => body = true,
            other => {
                if let Some(value) = other
                    .strip_prefix("--method=")
                    .or_else(|| other.strip_prefix("-X").filter(|v| !v.is_empty()))
                {
                    method = Some(value.to_ascii_uppercase());
                }
                if ["--field=", "--raw-field=", "--input="]
                    .iter()
                    .any(|prefix| other.starts_with(prefix))
                    || ((other.starts_with("-f") || other.starts_with("-F")) && other.len() > 2)
                {
                    body = true;
                }
            }
        }
        index += 1;
    }
    if path.trim_start_matches('/') == "graphql" {
        return args.iter().any(|arg| arg.contains("mutation")) || args.contains(&"--input");
    }
    match method.as_deref() {
        Some("GET" | "HEAD") => false,
        Some(_) => true,
        None => body,
    }
}

fn tmp_write(simple: &Simple<'_, '_>) -> bool {
    let args: Vec<&str> = simple.args().collect();
    let flagged = args.iter().enumerate().any(|(index, arg)| {
        let value = match *arg {
            "-o" | "--output" | "--out" => args.get(index + 1).copied(),
            other => other
                .strip_prefix("--output=")
                .or_else(|| other.strip_prefix("--out="))
                .or_else(|| other.strip_prefix("-o").filter(|v| v.starts_with('/'))),
        };
        value.is_some_and(is_tmp)
    });
    if flagged {
        return true;
    }
    let operands: Vec<&str> = args
        .iter()
        .copied()
        .filter(|arg| !arg.starts_with('-'))
        .collect();
    match simple.program() {
        "tee" | "mkdir" | "touch" => operands.iter().any(|arg| is_tmp(arg)),
        "mktemp" => {
            args.iter().enumerate().any(|(index, arg)| {
                let dir = match *arg {
                    "-p" | "--tmpdir" => args.get(index + 1).copied(),
                    other => other
                        .strip_prefix("--tmpdir=")
                        .or_else(|| other.strip_prefix("-p").filter(|v| !v.is_empty())),
                };
                dir.is_some_and(is_tmp)
            }) || operands.iter().any(|arg| is_tmp(arg))
        }
        "cp" | "mv" | "install" | "rsync" | "ln" => {
            let target = args.iter().enumerate().find_map(|(index, arg)| match *arg {
                "-t" | "--target-directory" => args.get(index + 1).copied(),
                other => other.strip_prefix("--target-directory="),
            });
            target
                .or_else(|| operands.last().copied())
                .is_some_and(is_tmp)
        }
        _ => false,
    }
}

fn read_with_timeout(simple: &Simple<'_, '_>) -> bool {
    simple
        .args()
        .any(|arg| arg.starts_with('-') && !arg.starts_with("--") && arg.contains('t'))
}

fn full_match(simple: &Simple<'_, '_>) -> bool {
    simple.args().any(|arg| {
        arg == "--full"
            || (arg.starts_with('-')
                && !arg.starts_with("--")
                && arg[1..].chars().all(|c| c.is_ascii_alphabetic())
                && arg.contains('f'))
    })
}

/// Whether the command reads its program or its file content from stdin.
fn reads_stdin(simple: &Simple<'_, '_>) -> bool {
    let program = simple.program();
    let args: Vec<&str> = simple.args().collect();
    let interpreter = program.starts_with("python")
        || matches!(program, "node" | "ruby" | "perl")
        || matches!(program, "bash" | "sh" | "zsh" | "dash");
    if interpreter {
        let shell = matches!(program, "bash" | "sh" | "zsh" | "dash");
        if args.contains(&"-") || (shell && args.contains(&"-s")) {
            return true;
        }
        let code_flag = args.iter().any(|arg| {
            matches!(*arg, "-c" | "-m" | "-e" | "--eval")
                || (shell && arg.starts_with('-') && !arg.starts_with("--") && arg.contains('c'))
        });
        let script = args.iter().any(|arg| !arg.starts_with('-'));
        return !code_flag && !script;
    }
    match program {
        "cat" => simple
            .redirects
            .iter()
            .any(|redirect| redirect.writes() && !redirect.target.starts_with("/dev/")),
        "tee" => args.iter().any(|arg| !arg.starts_with('-')),
        "git" => {
            let commit = git_subcommand(&args);
            commit.is_some_and(|(sub, rest)| {
                sub == "commit"
                    && rest.iter().enumerate().any(|(index, arg)| {
                        ((*arg == "-F" || *arg == "--file") && rest.get(index + 1) == Some(&"-"))
                            || *arg == "-F-"
                            || *arg == "--file=-"
                    })
            })
        }
        _ => false,
    }
}

/// `python -c "…"` whose double-quoted code holds a backtick, which the shell runs as a command.
fn python_backtick(simple: &Simple<'_, '_>) -> bool {
    if !simple.program().starts_with("python") {
        return false;
    }
    simple.words.windows(2).any(|pair| {
        pair[0].0 == "-c"
            && pair[1]
                .1
                .is_some_and(|node| node.kind() == "string" && has_backtick(node, simple.source))
    })
}

fn has_backtick(node: Node<'_>, src: &str) -> bool {
    if node.kind() == "command_substitution" && text(node, src).starts_with('`') {
        return true;
    }
    let mut cursor = node.walk();
    let found = node
        .named_children(&mut cursor)
        .any(|child| has_backtick(child, src));
    found
}

/// Git's subcommand and the words after it, past the global options.
fn git_subcommand<'a>(args: &[&'a str]) -> Option<(&'a str, Vec<&'a str>)> {
    let mut index = 0;
    while let Some(arg) = args.get(index) {
        match *arg {
            "-C" | "-c" | "--git-dir" | "--work-tree" | "--namespace" | "--config-env" => {
                index += 2;
            }
            other if other.starts_with('-') => index += 1,
            other => return Some((other, args[index + 1..].to_vec())),
        }
    }
    None
}

/// `git commit` given its message on the command line: `-m`, `--message`, `-am`.
fn commit_message(simple: &Simple<'_, '_>) -> bool {
    let args: Vec<&str> = simple.args().collect();
    let Some(("commit", rest)) = git_subcommand(&args) else {
        return false;
    };
    let mut index = 0;
    while let Some(arg) = rest.get(index) {
        index += 1;
        if *arg == "--" {
            return false;
        }
        if *arg == "--message" || arg.starts_with("--message=") {
            return true;
        }
        if matches!(
            *arg,
            "--file"
                | "--reuse-message"
                | "--reedit-message"
                | "--template"
                | "--author"
                | "--date"
                | "--fixup"
                | "--squash"
                | "--trailer"
                | "--cleanup"
                | "--pathspec-from-file"
        ) {
            index += 1;
            continue;
        }
        if let Some(cluster) = arg.strip_prefix('-').filter(|c| !c.starts_with('-')) {
            for flag in cluster.chars() {
                match flag {
                    'm' => return true,
                    'F' | 'C' | 'c' | 't' => {
                        if cluster.ends_with(flag) {
                            index += 1;
                        }
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Fixtures {
        config: FixtureConfig,
        case: Vec<Case>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct FixtureConfig {
        orgs: Vec<String>,
        workspace_roots: Vec<PathBuf>,
        home: PathBuf,
    }

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Case {
        group: String,
        expect: String,
        command: String,
        cwd: Option<PathBuf>,
        message: Option<String>,
        #[serde(default)]
        defaults: bool,
    }

    const FIXTURES: &str = include_str!("../tests/fixtures/guard.toml");

    fn group(name: &str) -> Group {
        Group::ALL
            .into_iter()
            .find(|group| group.name() == name)
            .unwrap_or_else(|| panic!("unknown group {name}"))
    }

    #[test]
    fn fixtures() {
        let fixtures: Fixtures = toml::from_str(FIXTURES).expect("fixtures parse");
        let home = fixtures.config.home.clone();
        let all = Config::all(
            fixtures.config.orgs.clone(),
            fixtures.config.workspace_roots.clone(),
            Some(home.clone()),
        );
        let defaults = Config::defaults(Some(home.clone()));
        let mut failures = Vec::new();
        let mut counts = std::collections::BTreeMap::<(&str, &str), usize>::new();
        for case in &fixtures.case {
            let expected_group = group(&case.group);
            let config = if case.defaults { &defaults } else { &all };
            let cwd = case.cwd.clone().unwrap_or_else(|| home.join("scratch"));
            let got = verdict(&case.command, &cwd, config);
            let ok = match case.expect.as_str() {
                "refuse" => {
                    got == Verdict::Refuse(expected_group)
                        && case
                            .message
                            .as_deref()
                            .is_none_or(|needle| expected_group.message().contains(needle))
                }
                "allow" => got == Verdict::Allow,
                other => panic!("expect must be refuse or allow, not {other}"),
            };
            if !ok {
                failures.push(format!(
                    "{} {}: {:?}\n    got {got:?}",
                    case.expect, case.group, case.command
                ));
            }
            *counts
                .entry((expected_group.name(), case.expect.as_str()))
                .or_default() += 1;
        }
        for group in Group::ALL {
            for expect in ["refuse", "allow"] {
                assert!(
                    counts.get(&(group.name(), expect)).copied().unwrap_or(0) > 0,
                    "{} has no {expect} fixture",
                    group.name()
                );
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    #[test]
    fn every_refusal_is_one_short_line() {
        for group in Group::ALL {
            let message = group.message();
            assert!(
                !message.contains('\n') && message.len() < 200,
                "{}",
                group.name()
            );
        }
    }

    #[test]
    fn config_defaults_and_parse() {
        let defaults = Config::defaults(None);
        let on: Vec<&str> = Group::ALL
            .into_iter()
            .filter(|group| defaults.on(*group))
            .map(Group::name)
            .collect();
        assert_eq!(on, ["read_dev_zero", "pgrep_loops", "heredoc_stdin"]);

        let text = "[gh_writes]\nenabled = true\norgs = [\"example-org\"]\nworkspace_roots = [\"~/work\"]\n\n[tmp_writes]\nenabled = true\n\n[pgrep_loops]\nenabled = false\n";
        let config = Config::parse(text, Some(PathBuf::from("/srv/agent"))).expect("parses");
        assert!(config.on(Group::GhWrites));
        assert!(config.on(Group::TmpWrites));
        assert!(!config.on(Group::PgrepLoops));
        assert!(config.on(Group::HeredocStdin));
        assert!(!config.on(Group::CommitMessageFlag));
        assert_eq!(config.workspace_roots, [PathBuf::from("/srv/agent/work")]);

        assert!(Config::parse("[tmp_write]\nenabled = true\n", None).is_err());
        assert!(Config::parse("[gh_writes]\norgs = \"x\"\n", None).is_err());
    }

    #[test]
    fn config_path_order() {
        let home = Path::new("/srv/agent");
        assert_eq!(
            config_path(home, Some("/etc/g.toml".into()), Some("/x".into())),
            PathBuf::from("/etc/g.toml")
        );
        assert_eq!(
            config_path(home, None, Some("/x".into())),
            PathBuf::from("/x/b10x/guard.toml")
        );
        assert_eq!(
            config_path(home, Some(OsString::new()), None),
            PathBuf::from("/srv/agent/.config/b10x/guard.toml")
        );
    }

    #[test]
    fn nothing_on_allows_everything() {
        let mut config = Config::defaults(None);
        for group in Group::ALL {
            config.set(group, false);
        }
        assert_eq!(
            verdict("pkill -f x; read -t 1 < /dev/zero", Path::new("/"), &config),
            Verdict::Allow
        );
    }

    #[test]
    fn a_long_command_is_fast() {
        let config = Config::all(
            vec!["example-org".into()],
            vec![PathBuf::from("/work")],
            None,
        );
        let line = "for f in a b c; do echo \"$f\" | grep -q x && cp \"$f\" ~/.cache/x/; done\n";
        let command = line.repeat(40);
        let start = std::time::Instant::now();
        assert_eq!(
            verdict(&command, Path::new("/work/r"), &config),
            Verdict::Allow
        );
        assert!(start.elapsed() < std::time::Duration::from_millis(50));
    }
}
