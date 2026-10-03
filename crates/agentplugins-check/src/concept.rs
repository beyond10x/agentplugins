//! The plugin concept in `website/docs/structure.md`, enforced. Each rule below is one row of that
//! page; a structure decision that is not checked here drifts, which is how two AEP plugins shipped
//! after one had been agreed.
//!
//! - **R2** one plugin per product; plugin name = product id = the CLI it drives.
//! - **R3** a skill is a lifecycle skill (`init`, `upgrade`), an activity named in `-ing` form, or
//!   a command: a verb the operator alone starts (`disable-model-invocation: true`), at most
//!   [`COMMAND_LINES`] lines, handing off to exactly one activity of its plugin. None is named after
//!   its plugin.
//! - **R4** an agent is owned by exactly one skill of its plugin, which lists it under `## Agents`.
//! - **R5** a skill quotes no CLI version, and a `**Skill version X**` line names the version its
//!   plugin's `.claude-plugin/plugin.json` carries.
//! - **R7** every `<plugin>:<skill-or-agent>` id written in this repository resolves.
//! - **R8** one README row, one plugin page and one sidebar entry per plugin; the README stays short,
//!   besides a generated tree of every skill and agent.
//!
//! R1, R5 and R6 are the marketplace, manifest-version and retired-name checks in `main.rs`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// Plugins the catalog knows, by where they live.
pub struct Plugins {
    /// Carried in `plugins/<name>/`.
    pub carried: BTreeSet<String>,
    /// Pointed at in another repository.
    pub remote: BTreeSet<String>,
}

impl Plugins {
    fn all(&self) -> BTreeSet<String> {
        self.carried.union(&self.remote).cloned().collect()
    }
}

/// The two lifecycle skills every plugin carries (R3); every other skill is an activity.
const LIFECYCLE: &[&str] = &["init", "upgrade"];

/// CLIs whose versions a plugin must not quote (R5): the newest release is the only one they describe.
const CLIS: &[&str] = &[
    "aep",
    "ess",
    "worktree",
    "metaharness",
    "b10x-harness",
    "protocol",
];

/// Every `<cli> x.y.z` in a line, case-insensitive, with optional backticks and a `v`.
#[must_use]
pub fn quoted_versions(line: &str) -> Vec<String> {
    let lower = line.to_ascii_lowercase();
    let bytes = lower.as_bytes();
    let mut found = Vec::new();
    for cli in CLIS {
        let mut from = 0;
        while let Some(offset) = lower[from..].find(cli) {
            let start = from + offset;
            from = start + cli.len();
            if start > 0 && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'-') {
                continue;
            }
            let rest = lower[from..]
                .trim_start_matches('`')
                .trim_start_matches(' ')
                .trim_start_matches('`');
            let rest = rest.strip_prefix('v').unwrap_or(rest);
            let digits: String = rest
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == '.')
                .collect();
            if digits.split('.').filter(|part| !part.is_empty()).count() >= 3
                && lower[from..].starts_with([' ', '`'])
            {
                found.push(format!("{cli} {digits}"));
            }
        }
    }
    found
}

fn versions(directory: &Path, name: &str, problems: &mut Vec<String>) {
    let mut stack = vec![directory.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for path in entries(&dir) {
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
                let text = read(&path).unwrap_or_default();
                for (number, line) in text.lines().enumerate() {
                    for quote in quoted_versions(line) {
                        problems.push(format!(
                            "R5 plugins/{name}/{}:{} quotes `{quote}`; skills describe the newest release and name no CLI version",
                            path.strip_prefix(directory).unwrap_or(&path).display(),
                            number + 1
                        ));
                    }
                }
            }
        }
    }
}

/// The most lines the README may have before its generated documentation block.
const README_LINES: usize = 30;

/// Directories never read for references.
const SKIP_DIRS: &[&str] = &[".git", "target", "node_modules", "build", ".docusaurus"];

/// Paths whose text records history or recorded output, not current references.
const HISTORY: &[&str] = &[
    "CHANGELOG.md",
    "changes/",
    ".engineering/",
    "crates/",
    "catalog.json",
];

fn read(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|error| format!("reading {}: {error}", path.display()))
}

fn frontmatter_name(text: &str) -> Option<String> {
    let body = text.strip_prefix("---\n")?;
    let end = body.find("\n---")?;
    body[..end]
        .lines()
        .find_map(|line| line.strip_prefix("name:"))
        .map(|name| name.trim().trim_matches('"').to_owned())
}

fn entries(directory: &Path) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(directory)
        .map(|read| {
            read.filter_map(|entry| entry.ok().map(|entry| entry.path()))
                .collect()
        })
        .unwrap_or_default();
    paths.sort();
    paths
}

/// R3: whether a skill name is an activity: one or two hyphen-joined words, the first ending in `ing`.
#[must_use]
pub fn activity(name: &str) -> bool {
    let words: Vec<&str> = name.split('-').collect();
    (1..=2).contains(&words.len())
        && words
            .iter()
            .all(|word| !word.is_empty() && word.bytes().all(|b| b.is_ascii_lowercase()))
        && words[0].len() > 4
        && words[0].ends_with("ing")
}

/// R3: whether a skill name is a command: one or two hyphen-joined lowercase words, the first a verb
/// that does not end in `ing` (which would make it an activity).
#[must_use]
pub fn command_name(name: &str) -> bool {
    let words: Vec<&str> = name.split('-').collect();
    (1..=2).contains(&words.len())
        && words
            .iter()
            .all(|word| !word.is_empty() && word.bytes().all(|b| b.is_ascii_lowercase()))
        && !words[0].ends_with("ing")
}

/// The most lines a command's body may have (R3): past that it is carrying behaviour, which
/// belongs in the activity skill it hands off to.
const COMMAND_LINES: usize = 20;

/// The body of a markdown file after its frontmatter, without leading or trailing blank lines.
fn body(text: &str) -> &str {
    let rest = text
        .strip_prefix("---\n")
        .and_then(|rest| rest.find("\n---").map(|end| &rest[end + 4..]))
        .unwrap_or(text);
    rest.trim()
}

/// Whether the frontmatter sets `key: true`.
fn frontmatter_true(text: &str, key: &str) -> bool {
    let Some(block) = text
        .strip_prefix("---\n")
        .and_then(|rest| rest.find("\n---").map(|end| &rest[..end]))
    else {
        return false;
    };
    block.lines().any(|line| {
        line.strip_prefix(key)
            .and_then(|rest| rest.strip_prefix(':'))
            .is_some_and(|value| value.trim() == "true")
    })
}

/// R3 over one command skill of `plugin`. `skills` are the plugin's skill folders and `activities`
/// the subset that are activities; a command hands off to exactly one of those.
///
/// A command is started by the operator only (`disable-model-invocation: true`), so a model never
/// picks it in place of the activity; it is short, so the behaviour stays in one place; and it
/// names the activity it hands off to, so a rename of that activity breaks the gate, not the
/// command.
#[must_use]
pub fn command(
    plugin: &str,
    folder: &str,
    text: &str,
    skills: &BTreeSet<String>,
    activities: &BTreeSet<String>,
) -> Vec<String> {
    let mut problems = Vec::new();
    if folder == plugin {
        problems.push(format!("R3 `{plugin}:{folder}` is named after its plugin"));
        return problems;
    }
    if !command_name(folder) {
        problems.push(format!(
            "R3 `{plugin}:{folder}` is neither an activity (one or two words, the first ending in `ing`) nor a command (one or two words, the first a verb)"
        ));
        return problems;
    }
    if !frontmatter_true(text, "disable-model-invocation") {
        problems.push(format!(
            "R3 command `{plugin}:{folder}` does not set `disable-model-invocation: true`; only the operator starts a command"
        ));
    }
    let body = body(text);
    let lines = body.lines().count();
    if lines > COMMAND_LINES {
        problems.push(format!(
            "R3 command `{plugin}:{folder}` has {lines} lines of body; the most is {COMMAND_LINES}, and the procedure belongs in the activity it hands off to"
        ));
    }
    let own: BTreeSet<String> = [plugin.to_owned()].into();
    let mut handoffs = BTreeSet::new();
    for (_, name) in body.lines().flat_map(|line| ids(line, &own)) {
        if !skills.contains(&name) {
            problems.push(format!(
                "R3 command `{plugin}:{folder}` names `{plugin}:{name}`, which is no skill of `{plugin}`"
            ));
        } else if activities.contains(&name) {
            handoffs.insert(name);
        }
    }
    match handoffs.len() {
        1 => {}
        0 => problems.push(format!(
            "R3 command `{plugin}:{folder}` hands off to no activity skill of `{plugin}`; name the one it hands off to as `{plugin}:<activity>`"
        )),
        n => problems.push(format!(
            "R3 command `{plugin}:{folder}` names {n} activity skills ({}); a command hands off to exactly one",
            handoffs.into_iter().collect::<Vec<_>>().join(", ")
        )),
    }
    problems
}

/// R3: whether a skill's `agents/openai.yaml` makes it operator-only in Codex, which reads
/// `policy.allow_implicit_invocation` rather than Claude Code's `disable-model-invocation`.
#[must_use]
pub fn codex_operator_only(yaml: Option<&str>) -> bool {
    yaml.and_then(|text| serde_yaml::from_str::<serde_yaml::Value>(text).ok())
        .and_then(|document| {
            document
                .get("policy")?
                .get("allow_implicit_invocation")?
                .as_bool()
        })
        == Some(false)
}

/// The most lines an agent's body may have (R4): an agent is a thin Claude Code adapter over a
/// role whose procedure lives in its owning skill, which Codex loads and Codex does not load
/// `agents/`.
const AGENT_LINES: usize = 20;

/// R4 over one agent file of `plugin`, owned by the skill `owner` (if exactly one lists it).
///
/// The agent is thin: its body is at most [`AGENT_LINES`] lines and names its owning skill as
/// `<plugin>:<owner>`, so the role's behaviour sits where both hosts read it. An agent that
/// carries the procedure itself is the Claude-only copy that Codex never sees.
#[must_use]
pub fn agent(plugin: &str, stem: &str, text: &str, owner: Option<&str>) -> Vec<String> {
    let mut problems = Vec::new();
    let body = body(text);
    let lines = body.lines().count();
    if lines > AGENT_LINES {
        problems.push(format!(
            "R4 agent `{plugin}:{stem}` has {lines} lines of body; the most is {AGENT_LINES}: put the role's procedure in its owning skill (or its `references/`), which Codex loads, and keep the agent a thin adapter"
        ));
    }
    if let Some(owner) = owner {
        let own: BTreeSet<String> = [plugin.to_owned()].into();
        if !body
            .lines()
            .flat_map(|line| ids(line, &own))
            .any(|(_, name)| name == owner)
        {
            problems.push(format!(
                "R4 agent `{plugin}:{stem}` does not name `{plugin}:{owner}`, the skill that owns it; name it, so the role is followed from the skill both hosts load"
            ));
        }
    }
    problems
}

/// Relative markdown links (`](path.md)`, optionally with a `#fragment`) in a text, in order.
#[must_use]
pub fn links(text: &str) -> Vec<String> {
    text.split("](")
        .skip(1)
        .filter_map(|rest| rest.split_once(')').map(|(target, _)| target))
        .map(|target| target.split('#').next().unwrap_or_default())
        .filter(|target| {
            Path::new(target)
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
                && !target.contains("://")
                && !target.starts_with('/')
        })
        .map(str::to_owned)
        .collect()
}

/// R3 and R4 as a plugin author reads them: the phrases every page that teaches the plugin
/// structure must state, taken from the constants the checker enforces, so the teaching and the
/// gate cannot drift apart. Returns the phrases `text` is missing.
#[must_use]
pub fn teaches(text: &str) -> Vec<String> {
    [
        "disable-model-invocation: true".to_owned(),
        "allow_implicit_invocation: false".to_owned(),
        format!("at most {COMMAND_LINES} lines"),
        format!("at most {AGENT_LINES} lines"),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>()
    .into_iter()
    .filter(|phrase| !text.contains(phrase.as_str()))
    .collect()
}

/// `b10x:routing` over every carried plugin's skills: each skill is reachable from the routing
/// skill, and a table row about upgrading routes to no `init` skill (`init` sets a product up;
/// `upgrade` checks it and offers the upgrade).
#[must_use]
pub fn routing(text: &str, skills: &BTreeMap<String, BTreeSet<String>>) -> Vec<String> {
    let mut problems = Vec::new();
    let plugins: BTreeSet<String> = skills.keys().cloned().collect();
    let named: BTreeSet<(String, String)> =
        text.lines().flat_map(|line| ids(line, &plugins)).collect();
    for (plugin, members) in skills {
        for skill in members {
            if !named.contains(&(plugin.clone(), skill.clone())) {
                problems.push(format!(
                    "b10x:routing never names `{plugin}:{skill}`; every skill is reachable from the routing skill"
                ));
            }
        }
    }
    for line in text.lines() {
        let Some(request) = line
            .strip_prefix('|')
            .and_then(|rest| rest.split_once('|'))
            .map(|(cell, _)| cell.to_ascii_lowercase())
        else {
            continue;
        };
        if !request.contains("upgrade") {
            continue;
        }
        for (plugin, skill) in ids(line, &plugins) {
            if skill == "init" {
                problems.push(format!(
                    "b10x:routing routes an upgrade request to `{plugin}:init`; upgrades go to `{plugin}:upgrade`: `{}`",
                    line.trim()
                ));
            }
        }
    }
    problems
}

/// R5: the `**Skill version X**` line a skill may carry, with its 1-based line number.
#[must_use]
pub fn skill_version(text: &str) -> Option<(usize, String)> {
    text.lines().enumerate().find_map(|(number, line)| {
        let rest = line.split_once("**Skill version ")?.1;
        let (version, _) = rest.split_once("**")?;
        Some((number + 1, version.trim().to_owned()))
    })
}

/// The agent names a skill lists under its `## Agents` heading, one `` - `name` `` bullet each.
#[must_use]
pub fn listed_agents(skill: &str) -> Vec<String> {
    let mut agents = Vec::new();
    let mut inside = false;
    for line in skill.lines() {
        if line.starts_with("## ") {
            inside = line.trim() == "## Agents";
            continue;
        }
        if inside {
            if let Some(rest) = line.strip_prefix("- `") {
                if let Some((name, _)) = rest.split_once('`') {
                    agents.push(name.to_owned());
                }
            }
        }
    }
    agents
}

/// R4 over one agent file of `plugin`: its name, its thinness, its links and its one owner.
fn agent_file(
    path: &Path,
    name: &str,
    stem: &str,
    owners: &BTreeMap<String, Vec<String>>,
    problems: &mut Vec<String>,
) {
    let text = read(path).unwrap_or_default();
    if frontmatter_name(&text).as_deref() != Some(stem) {
        problems.push(format!("R4 agent `{name}:{stem}` declares another `name:`"));
    }
    let owner = match owners.get(stem).map(Vec::as_slice) {
        Some([owner]) => Some(owner.as_str()),
        _ => None,
    };
    problems.extend(agent(name, stem, &text, owner));
    for link in links(body(&text)) {
        if !path.parent().unwrap_or(path).join(&link).is_file() {
            problems.push(format!(
                "R4 agent `{name}:{stem}` links `{link}`, which does not exist"
            ));
        }
    }
    match owners.get(stem).map(Vec::as_slice) {
        None | Some([]) => problems.push(format!(
            "R4 agent `{name}:{stem}` is owned by no skill; list it under `## Agents` in the skill that dispatches it"
        )),
        Some([_]) => {}
        Some(many) => problems.push(format!(
            "R4 agent `{name}:{stem}` is listed by {} skills ({}); exactly one owns it",
            many.len(),
            many.join(", ")
        )),
    }
}

/// Skills and agents of one carried plugin, with R3 and R4 applied.
fn plugin(
    root: &Path,
    name: &str,
    problems: &mut Vec<String>,
) -> (BTreeSet<String>, BTreeSet<String>) {
    let directory = root.join("plugins").join(name);
    let mut skills = BTreeSet::new();
    let mut activities = BTreeSet::new();
    let mut commands = Vec::new();
    let mut owners: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let manifest = read(&directory.join(".claude-plugin/plugin.json"))
        .ok()
        .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())
        .and_then(|document| document.get("version")?.as_str().map(str::to_owned));
    for path in entries(&directory.join("skills")) {
        let Some(folder) = path.file_name().and_then(|n| n.to_str()).map(str::to_owned) else {
            continue;
        };
        let Ok(text) = read(&path.join("SKILL.md")) else {
            problems.push(format!("R3 `{name}:{folder}` has no SKILL.md"));
            continue;
        };
        if activity(&folder) {
            if folder == name {
                problems.push(format!("R3 `{name}:{folder}` is named after its plugin"));
            }
            activities.insert(folder.clone());
        } else if !LIFECYCLE.contains(&folder.as_str()) {
            commands.push((folder.clone(), text.clone()));
        }
        if frontmatter_name(&text).as_deref() != Some(folder.as_str()) {
            problems.push(format!(
                "R3 `{name}:{folder}` declares another `name:` in its frontmatter"
            ));
        }
        if let Some((line, version)) = skill_version(&text) {
            if manifest.as_deref() != Some(version.as_str()) {
                problems.push(format!(
                    "R5 plugins/{name}/skills/{folder}/SKILL.md:{line} says skill version {version}; `.claude-plugin/plugin.json` carries {}",
                    manifest.as_deref().unwrap_or("no version")
                ));
            }
        }
        for agent in listed_agents(&text) {
            owners.entry(agent).or_default().push(folder.clone());
        }
        skills.insert(folder);
    }
    for (folder, text) in &commands {
        problems.extend(command(name, folder, text, &skills, &activities));
        let yaml = read(
            &directory
                .join("skills")
                .join(folder)
                .join("agents/openai.yaml"),
        )
        .ok();
        if command_name(folder) && folder != name && !codex_operator_only(yaml.as_deref()) {
            problems.push(format!(
                "R3 command `{name}:{folder}` has no `agents/openai.yaml` with `policy.allow_implicit_invocation: false`; Codex would let the model start it"
            ));
        }
    }
    for lifecycle in LIFECYCLE {
        if !skills.contains(*lifecycle) {
            problems.push(format!(
                "R3 `{name}` has no `{lifecycle}` skill; every plugin carries `init` and `upgrade`"
            ));
        }
    }
    versions(&directory, name, problems);
    let mut agents = BTreeSet::new();
    for path in entries(&directory.join("agents")) {
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|n| n.to_str()).map(str::to_owned) else {
            continue;
        };
        agent_file(&path, name, &stem, &owners, problems);
        agents.insert(stem);
    }
    for (agent, skills_listing) in &owners {
        if !agents.contains(agent) {
            problems.push(format!(
                "R4 `{name}:{}` lists agent `{agent}`, which does not exist",
                skills_listing.join("`, `")
            ));
        }
    }
    (skills, agents)
}

/// R2 over `catalog.json`.
fn products(catalog: &serde_json::Value, problems: &mut Vec<String>) {
    for product in catalog
        .get("products")
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
    {
        let id = product
            .get("id")
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default();
        let plugins: Vec<&str> = product
            .get("plugins")
            .and_then(serde_json::Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(serde_json::Value::as_str)
            .collect();
        if plugins != [id] {
            problems.push(format!(
                "R2 product `{id}` must be exactly one plugin named `{id}`, not {plugins:?}"
            ));
        }
        let first_binary = product
            .get("binaries")
            .and_then(serde_json::Value::as_array)
            .and_then(|binaries| binaries.first())
            .and_then(|binary| binary.get("name"))
            .and_then(serde_json::Value::as_str);
        if let Some(binary) = first_binary {
            if binary != id {
                problems.push(format!(
                    "R2 product `{id}` drives `{binary}` first; its CLI must carry the product's name"
                ));
            }
        }
    }
}

fn authored(root: &Path, relative: &str) -> bool {
    let historical = HISTORY
        .iter()
        .any(|history| relative == *history || relative.starts_with(history));
    let recorded = relative.starts_with("evals/") && relative.contains("/recorded/");
    let text = [".md", ".yaml", ".yml", ".json", ".ts", ".tsx", ".svg"]
        .iter()
        .any(|extension| relative.ends_with(extension));
    text && !historical && !recorded && root.join(relative).is_file()
}

fn walk(root: &Path) -> Vec<String> {
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(directory) = stack.pop() {
        for path in entries(&directory) {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();
            if path.is_dir() {
                if !SKIP_DIRS.contains(&name) {
                    stack.push(path);
                }
            } else if let Ok(relative) = path.strip_prefix(root) {
                files.push(relative.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    files.sort();
    files
}

/// Every `<plugin>:<name>` id in a line, for the given plugin names.
#[must_use]
pub fn ids(line: &str, plugins: &BTreeSet<String>) -> Vec<(String, String)> {
    let bytes = line.as_bytes();
    let mut found = Vec::new();
    for plugin in plugins {
        let mut from = 0;
        while let Some(offset) = line[from..].find(&format!("{plugin}:")) {
            let start = from + offset;
            let after = start + plugin.len() + 1;
            from = after;
            let before_ok = start == 0 || {
                let b = bytes[start - 1];
                !(b.is_ascii_alphanumeric()
                    || b == b'-'
                    || b == b'_'
                    || b == b'/'
                    || b == b'.'
                    || b == b'@')
            };
            if !before_ok {
                continue;
            }
            let name: String = line[after..]
                .chars()
                .take_while(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '-')
                .collect();
            let name = name.trim_end_matches('-').to_owned();
            if name.is_empty() || !name.starts_with(|c: char| c.is_ascii_lowercase()) {
                continue;
            }
            found.push((plugin.clone(), name));
        }
    }
    found
}

/// R7 over carried plugins; ids of remote plugins are checked by `agentplugins-check remote`.
fn references(
    root: &Path,
    plugins: &Plugins,
    known: &BTreeMap<String, BTreeSet<String>>,
    problems: &mut Vec<String>,
) {
    let names = plugins.all();
    for relative in walk(root) {
        if !authored(root, &relative) {
            continue;
        }
        let Ok(text) = read(&root.join(&relative)) else {
            continue;
        };
        for (number, line) in text.lines().enumerate() {
            for (plugin, name) in ids(line, &names) {
                let Some(members) = known.get(&plugin) else {
                    continue;
                };
                if !members.contains(&name) {
                    problems.push(format!(
                        "R7 {relative}:{} names `{plugin}:{name}`, which is no skill or agent of `{plugin}`",
                        number + 1
                    ));
                }
            }
        }
    }
}

/// README rows: the plugin names in the first column of rows that start with ``| [`name`]``.
#[must_use]
pub fn readme_rows(readme: &str) -> Vec<String> {
    readme
        .lines()
        .filter_map(|line| line.strip_prefix("| [`"))
        .filter_map(|rest| rest.split_once('`').map(|(name, _)| name.to_owned()))
        .collect()
}

/// Markers around the README's plugin tree, which [`tree`] generates.
const TREE_START: &str = "<!-- plugin-tree:start -->";
const TREE_END: &str = "<!-- plugin-tree:end -->";

/// One carried plugin's skills and agents, by name.
pub struct Contents {
    /// Skill folder names.
    pub skills: BTreeSet<String>,
    /// Agent file stems.
    pub agents: BTreeSet<String>,
}

/// The README's plugin tree, in table order: every skill (lifecycle first) and agent, each linked
/// to its source file.
#[must_use]
pub fn tree(order: &[String], contents: &BTreeMap<String, Contents>) -> String {
    let mut lines = vec![TREE_START.to_owned()];
    for name in order {
        let Some(plugin) = contents.get(name) else {
            continue;
        };
        lines.push(format!(
            "- [`{name}`](plugins/{name}/) · [docs](website/docs/plugins/{name}.md)"
        ));
        let lifecycle = LIFECYCLE.iter().map(|s| (*s).to_owned());
        let rest = plugin
            .skills
            .iter()
            .filter(|s| !LIFECYCLE.contains(&s.as_str()))
            .cloned();
        let skills: Vec<String> = lifecycle
            .chain(rest)
            .filter(|s| plugin.skills.contains(s))
            .map(|s| format!("[`{s}`](plugins/{name}/skills/{s}/SKILL.md)"))
            .collect();
        lines.push(format!("  - skills: {}", skills.join(" · ")));
        if !plugin.agents.is_empty() {
            let agents: Vec<String> = plugin
                .agents
                .iter()
                .map(|a| format!("[`{a}`](plugins/{name}/agents/{a}.md)"))
                .collect();
            lines.push(format!("  - agents: {}", agents.join(" · ")));
        }
    }
    lines.push(TREE_END.to_owned());
    lines.join("\n")
}

/// R8.
fn docs(
    root: &Path,
    plugins: &Plugins,
    contents: &BTreeMap<String, Contents>,
    problems: &mut Vec<String>,
) -> Result<(), String> {
    let all = plugins.all();
    let readme = read(&root.join("README.md"))?;
    let head = readme
        .split("<!-- b10x-docs:start -->")
        .next()
        .unwrap_or_default();
    let written = match (head.find(TREE_START), head.find(TREE_END)) {
        (Some(start), Some(end)) if start < end => &head[start..end + TREE_END.len()],
        _ => "",
    };
    let prose = head.lines().count() - written.lines().count();
    if prose > README_LINES {
        problems.push(format!(
            "R8 README.md has {prose} lines before its documentation block, besides the plugin tree; the most is {README_LINES}: one paragraph, the plugin table, one link line",
        ));
    }
    let rows = readme_rows(head);
    let expected = tree(&rows, contents);
    if written != expected {
        problems.push(format!(
            "R8 README.md plugin tree is missing or stale; it must read:\n{expected}"
        ));
    }
    let rows: BTreeSet<String> = rows.into_iter().collect();
    if rows != all {
        problems.push(format!(
            "R8 README.md table lists {rows:?}; the catalog has {all:?}"
        ));
    }
    let pages: BTreeSet<String> = entries(&root.join("website/docs/plugins"))
        .iter()
        .filter(|path| path.extension().and_then(|e| e.to_str()) == Some("md"))
        .filter_map(|path| path.file_stem().and_then(|n| n.to_str()).map(str::to_owned))
        .collect();
    if pages != all {
        problems.push(format!(
            "R8 website/docs/plugins has pages {pages:?}; the catalog has {all:?}"
        ));
    }
    let sidebar = read(&root.join("website/sidebars.ts"))?;
    let listed: BTreeSet<String> = sidebar
        .split("'plugins/")
        .skip(1)
        .filter_map(|rest| rest.split_once('\'').map(|(name, _)| name.to_owned()))
        .collect();
    if listed != all {
        problems.push(format!(
            "R8 website/sidebars.ts lists {listed:?}; the catalog has {all:?}"
        ));
    }
    Ok(())
}

/// Check the concept. `plugins` comes from the catalog and marketplace checks that ran first.
pub fn check(root: &Path, plugins: &Plugins) -> Result<(), String> {
    let mut problems = Vec::new();
    let catalog: serde_json::Value = serde_json::from_str(&read(&root.join("catalog.json"))?)
        .map_err(|error| format!("parsing catalog.json: {error}"))?;
    products(&catalog, &mut problems);
    let mut known = BTreeMap::new();
    let mut contents = BTreeMap::new();
    for name in &plugins.carried {
        let (skills, agents) = plugin(root, name, &mut problems);
        known.insert(
            name.clone(),
            skills.union(&agents).cloned().collect::<BTreeSet<_>>(),
        );
        contents.insert(name.clone(), Contents { skills, agents });
    }
    if plugins.carried.contains("b10x") {
        let routes = "plugins/b10x/skills/routing/SKILL.md";
        let skills: BTreeMap<String, BTreeSet<String>> = contents
            .iter()
            .map(|(name, plugin)| (name.clone(), plugin.skills.clone()))
            .collect();
        problems.extend(routing(&read(&root.join(routes))?, &skills));
        for page in [
            "plugins/b10x/skills/authoring-plugins/SKILL.md",
            "website/docs/structure.md",
        ] {
            for phrase in teaches(&read(&root.join(page))?) {
                problems.push(format!(
                    "R3/R4 {page} does not state `{phrase}`, which the gate enforces; the page that teaches the structure states it"
                ));
            }
        }
    }
    references(root, plugins, &known, &mut problems);
    docs(root, plugins, &contents, &mut problems)?;
    if problems.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "{} concept violation(s) (website/docs/structure.md):\n  {}",
            problems.len(),
            problems.join("\n  ")
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write as _;

    #[test]
    fn activity_names() {
        for good in [
            "planning",
            "migrating",
            "implementing",
            "authoring-plugins",
            "testing-conformance",
        ] {
            assert!(activity(good), "{good}");
        }
        for bad in [
            "wave",
            "drive",
            "setup",
            "guide",
            "plugin-creator",
            "ess",
            "sing",
            "planning-a-b",
            "Planning",
        ] {
            assert!(!activity(bad), "{bad}");
        }
    }

    fn set(names: &[&str]) -> BTreeSet<String> {
        names.iter().map(|s| (*s).to_owned()).collect()
    }

    fn command_text(flag: bool, body_lines: usize, handoff: &str) -> String {
        let mut text = String::from("---\nname: cleanup\ndescription: Clean up.\n");
        if flag {
            text.push_str("disable-model-invocation: true\n");
        }
        text.push_str("argument-hint: \"[--repo <path>]\"\n---\n\n");
        writeln!(text, "Read `{handoff}` and follow it.").unwrap();
        for n in 1..body_lines {
            writeln!(text, "- step {n}").unwrap();
        }
        text
    }

    fn worktree_command(folder: &str, text: &str) -> Vec<String> {
        let skills = set(&["init", "upgrade", "managing-worktrees", folder]);
        let activities = set(&["managing-worktrees"]);
        command("worktree", folder, text, &skills, &activities)
    }

    #[test]
    fn command_names_are_verbs_not_activities() {
        for good in ["cleanup", "wave", "drive", "review-plan", "decompose"] {
            assert!(command_name(good), "{good}");
        }
        for bad in [
            "planning",
            "managing-worktrees",
            "Wave",
            "",
            "a-b-c",
            "wave-",
        ] {
            assert!(!command_name(bad), "{bad}");
        }
    }

    #[test]
    fn a_valid_command_is_accepted() {
        let text = command_text(true, 20, "worktree:managing-worktrees");
        assert_eq!(worktree_command("cleanup", &text), Vec::<String>::new());
    }

    #[test]
    fn a_command_without_disable_model_invocation_is_refused() {
        let text = command_text(false, 5, "worktree:managing-worktrees");
        let problems = worktree_command("cleanup", &text);
        assert!(
            problems.len() == 1 && problems[0].contains("disable-model-invocation: true"),
            "{problems:?}"
        );
    }

    #[test]
    fn a_command_over_twenty_lines_is_refused() {
        let text = command_text(true, 21, "worktree:managing-worktrees");
        let problems = worktree_command("cleanup", &text);
        assert!(
            problems.len() == 1 && problems[0].contains("21 lines of body"),
            "{problems:?}"
        );
    }

    #[test]
    fn a_command_naming_a_missing_skill_is_refused() {
        let text = command_text(true, 5, "worktree:managing-trees");
        let problems = worktree_command("cleanup", &text);
        assert!(
            problems
                .iter()
                .any(|p| p.contains("`worktree:managing-trees`, which is no skill")),
            "{problems:?}"
        );
        assert!(
            problems
                .iter()
                .any(|p| p.contains("hands off to no activity")),
            "{problems:?}"
        );
    }

    #[test]
    fn a_command_handing_off_to_a_lifecycle_skill_or_two_activities_is_refused() {
        let text = command_text(true, 5, "worktree:init");
        let problems = worktree_command("cleanup", &text);
        assert!(
            problems.len() == 1 && problems[0].contains("hands off to no activity"),
            "{problems:?}"
        );
        let skills = set(&["init", "upgrade", "planning", "implementing", "wave"]);
        let activities = set(&["planning", "implementing"]);
        let text = command_text(true, 5, "aep:implementing` after `aep:planning");
        let problems = command("aep", "wave", &text, &skills, &activities);
        assert!(
            problems.len() == 1 && problems[0].contains("2 activity skills"),
            "{problems:?}"
        );
    }

    #[test]
    fn a_command_named_after_its_plugin_is_refused() {
        let text = command_text(true, 5, "worktree:managing-worktrees")
            .replace("name: cleanup", "name: worktree");
        let problems = worktree_command("worktree", &text);
        assert!(
            problems.len() == 1 && problems[0].contains("named after its plugin"),
            "{problems:?}"
        );
    }

    #[test]
    fn a_command_is_operator_only_in_codex_too() {
        let off =
            "interface:\n  display_name: \"X\"\npolicy:\n  allow_implicit_invocation: false\n";
        assert!(codex_operator_only(Some(off)));
        let on = off.replace("false", "true");
        assert!(!codex_operator_only(Some(&on)));
        assert!(!codex_operator_only(Some(
            "interface:\n  display_name: \"X\"\n"
        )));
        assert!(!codex_operator_only(None));
    }

    fn agent_text(body_lines: usize, handoff: &str) -> String {
        let mut text = String::from("---\nname: author\ndescription: Write.\n---\n\n");
        writeln!(text, "Follow the `{handoff}` skill completely.").unwrap();
        for n in 1..body_lines {
            writeln!(text, "- charter {n}").unwrap();
        }
        text
    }

    #[test]
    fn a_thin_agent_naming_its_owner_is_accepted() {
        let text = agent_text(20, "ess:specifying");
        assert_eq!(
            agent("ess", "author", &text, Some("specifying")),
            Vec::<String>::new()
        );
    }

    #[test]
    fn an_agent_carrying_its_procedure_is_refused() {
        let text = agent_text(21, "ess:specifying");
        let problems = agent("ess", "author", &text, Some("specifying"));
        assert!(
            problems.len() == 1 && problems[0].contains("21 lines of body"),
            "{problems:?}"
        );
    }

    #[test]
    fn an_agent_that_does_not_name_its_owning_skill_is_refused() {
        let text = agent_text(5, "ess:retrofitting");
        let problems = agent("ess", "author", &text, Some("specifying"));
        assert!(
            problems.len() == 1 && problems[0].contains("does not name `ess:specifying`"),
            "{problems:?}"
        );
        // An unowned agent is R4's other problem; it is not reported twice here.
        assert_eq!(agent("ess", "author", &text, None), Vec::<String>::new());
    }

    #[test]
    fn relative_markdown_links_are_read_and_urls_are_not() {
        let text = "Read [it](../skills/planning/references/decomposer.md#top) and \
                    [rubric](critic-rubric.md); not [site](https://x.dev/a.md) or [x](#anchor).";
        assert_eq!(
            links(text),
            [
                "../skills/planning/references/decomposer.md",
                "critic-rubric.md"
            ]
        );
    }

    #[test]
    fn a_page_teaching_the_structure_states_every_enforced_limit() {
        let good =
            "a command sets `disable-model-invocation: true`, has at most 20 lines of body, \
                    and its `agents/openai.yaml` sets `allow_implicit_invocation: false`; an agent \
                    has at most 20 lines of body and names its owning skill";
        assert_eq!(teaches(good), Vec::<String>::new());
        let missing = teaches("a command is a thin skill that carries the command's behaviour");
        assert_eq!(missing.len(), 3, "{missing:?}");
        assert!(missing
            .iter()
            .any(|m| m.contains("disable-model-invocation: true")));
        assert!(missing
            .iter()
            .any(|m| m.contains("allow_implicit_invocation: false")));
        assert!(missing.iter().any(|m| m.contains("at most 20 lines")));
    }

    fn routed() -> BTreeMap<String, BTreeSet<String>> {
        let mut skills = BTreeMap::new();
        skills.insert("b10x".to_owned(), set(&["init", "upgrade", "routing"]));
        skills.insert("ess".to_owned(), set(&["init", "upgrade", "specifying"]));
        skills
    }

    const ROUTES: &str = "| Request | Route |\n|---|---|\n\
        | Install the plugins | `b10x:init` |\n\
        | Check for or apply an upgrade | `b10x:upgrade` |\n\
        | Choose a plugin | `b10x:routing` |\n\
        | Set up one product | `ess:init` |\n\
        | Upgrade one product | `ess:upgrade` |\n\
        | Specify a system | `ess:specifying` |\n";

    #[test]
    fn a_routing_table_reaching_every_skill_is_accepted() {
        assert_eq!(routing(ROUTES, &routed()), Vec::<String>::new());
    }

    #[test]
    fn an_upgrade_request_routed_to_init_is_refused() {
        let text = ROUTES.replace(
            "| Install the plugins | `b10x:init` |",
            "| Install, upgrade or repair the plugins | `b10x:init` |",
        );
        let problems = routing(&text, &routed());
        assert!(
            problems.len() == 1
                && problems[0].contains("`b10x:init`")
                && problems[0].contains("upgrade"),
            "{problems:?}"
        );
    }

    #[test]
    fn a_skill_the_routing_skill_never_names_is_refused() {
        let text = ROUTES.replace("| Upgrade one product | `ess:upgrade` |\n", "");
        let problems = routing(&text, &routed());
        assert!(
            problems.len() == 1 && problems[0].contains("`ess:upgrade`"),
            "{problems:?}"
        );
    }

    #[test]
    fn a_skill_version_line_is_read_with_its_line_number() {
        let text = "---\nname: x\n---\n\n**Skill version 0.14.15** — the version\n";
        assert_eq!(skill_version(text), Some((5, "0.14.15".to_owned())));
        assert_eq!(skill_version("# no version line\n"), None);
    }

    #[test]
    fn cli_versions_are_found_and_skill_versions_are_not() {
        assert_eq!(quoted_versions("at AEP 0.55.0 the verb"), ["aep 0.55.0"]);
        assert_eq!(quoted_versions("output of ESS `0.29.0`"), ["ess 0.29.0"]);
        assert_eq!(
            quoted_versions("**Skill version 0.13.1** — the version"),
            Vec::<String>::new()
        );
        assert_eq!(
            quoted_versions("ess-cli 0.30.0 is not a quote of a CLI name"),
            Vec::<String>::new()
        );
        assert_eq!(
            quoted_versions("aep plan artifact list"),
            Vec::<String>::new()
        );
    }

    #[test]
    fn agents_are_read_from_the_agents_section_only() {
        let skill = "# X\n\nUses `implementor` in prose.\n\n## Agents\n\n- `story-scoper` — scopes\n- `adversary` — attacks\n\n## Next\n\n- `other` — not an agent list\n";
        assert_eq!(listed_agents(skill), ["story-scoper", "adversary"]);
    }

    #[test]
    fn ids_are_found_at_word_starts_only() {
        let plugins: BTreeSet<String> = ["aep", "b10x"].iter().map(|s| (*s).to_owned()).collect();
        assert_eq!(
            ids(
                "use `aep:planning` and b10x:init; see beyond10x/aep:x, https://x/aep:y, aep: 1",
                &plugins
            ),
            [
                ("aep".to_owned(), "planning".to_owned()),
                ("b10x".to_owned(), "init".to_owned())
            ]
        );
    }

    #[test]
    fn the_tree_lists_lifecycle_skills_first_and_links_every_file() {
        let mut contents = BTreeMap::new();
        contents.insert(
            "ess".to_owned(),
            Contents {
                skills: ["specifying", "upgrade", "init"].map(str::to_owned).into(),
                agents: ["author"].map(str::to_owned).into(),
            },
        );
        contents.insert(
            "b10x".to_owned(),
            Contents {
                skills: ["init", "upgrade"].map(str::to_owned).into(),
                agents: BTreeSet::new(),
            },
        );
        let order = ["ess".to_owned(), "b10x".to_owned()];
        assert_eq!(
            tree(&order, &contents),
            "<!-- plugin-tree:start -->\n\
             - [`ess`](plugins/ess/) · [docs](website/docs/plugins/ess.md)\n  \
             - skills: [`init`](plugins/ess/skills/init/SKILL.md) · [`upgrade`](plugins/ess/skills/upgrade/SKILL.md) · [`specifying`](plugins/ess/skills/specifying/SKILL.md)\n  \
             - agents: [`author`](plugins/ess/agents/author.md)\n\
             - [`b10x`](plugins/b10x/) · [docs](website/docs/plugins/b10x.md)\n  \
             - skills: [`init`](plugins/b10x/skills/init/SKILL.md) · [`upgrade`](plugins/b10x/skills/upgrade/SKILL.md)\n\
             <!-- plugin-tree:end -->"
        );
    }

    #[test]
    fn readme_rows_read_the_first_column() {
        let readme = "| plugin | for |\n|---|---|\n| [`ess`](x) | a |\n| [`aep`](y) | b |\n";
        assert_eq!(readme_rows(readme), ["ess", "aep"]);
    }

    #[test]
    fn a_product_with_two_plugins_is_refused() {
        let catalog = serde_json::json!({"products": [{"id": "aep", "plugins": [concat!("aep-", "plan"), concat!("aep-", "drive")], "binaries": [{"name": "aep"}]}]});
        let mut problems = Vec::new();
        products(&catalog, &mut problems);
        assert_eq!(problems.len(), 1, "{problems:?}");
    }
}
