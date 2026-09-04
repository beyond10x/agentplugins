//! Validates the curated marketplace identity and focused plugin contents.

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

mod evals;

const PLUGINS: &[(&str, &[&str])] = &[
    (
        "beyond10x",
        &[
            "skills/beyond10x/SKILL.md",
            "skills/beyond10x/references/resources.md",
            "skills/plugin-creator/SKILL.md",
            "skills/plugin-creator/references/compatibility.md",
        ],
    ),
    (
        "aep-plan",
        &[
            "skills/planning/SKILL.md",
            "skills/planning/references/critic-rubric.md",
            "skills/story-migration/SKILL.md",
            "agents/decomposer.md",
            "agents/plan-reviewer.md",
            "agents/reverse-engineer.md",
            "agents/plan-critic-acceptance.md",
            "agents/plan-critic-design.md",
            "agents/plan-critic-scope.md",
            "agents/plan-critic-parallel-safety.md",
        ],
    ),
    (
        "aep-drive",
        &[
            "skills/wave/SKILL.md",
            "skills/drive/SKILL.md",
            "agents/story-scoper.md",
            "agents/implementor.md",
            "agents/adversary.md",
        ],
    ),
    ("ess-specify", &["skills/specify/SKILL.md"]),
    ("workspace-hygiene", &["skills/worktree/SKILL.md"]),
];

fn json(path: &Path) -> Result<serde_json::Value, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("reading {}: {error}", path.display()))?;
    serde_json::from_str(&text).map_err(|error| format!("parsing {}: {error}", path.display()))
}

fn marketplace(root: &Path, relative: &str) -> Result<(), String> {
    let document = json(&root.join(relative))?;
    if document.get("name").and_then(serde_json::Value::as_str) != Some("beyond10x") {
        return Err(format!(
            "{relative} does not declare marketplace `beyond10x`"
        ));
    }
    let entries = document
        .get("plugins")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| format!("{relative} has no plugins array"))?;
    if entries.len() != PLUGINS.len() {
        return Err(format!(
            "{relative} contains {} plugins; expected {} focused plugins",
            entries.len(),
            PLUGINS.len()
        ));
    }
    for (index, (plugin, _)) in PLUGINS.iter().enumerate() {
        let actual = entries[index]
            .get("name")
            .and_then(serde_json::Value::as_str);
        if actual != Some(plugin) {
            return Err(format!(
                "{relative} plugin {index} is {actual:?}; expected `{plugin}`"
            ));
        }
        let count = entries
            .iter()
            .filter(|entry| entry.get("name").and_then(serde_json::Value::as_str) == Some(plugin))
            .count();
        if count != 1 {
            return Err(format!(
                "{relative} contains {count} entries for `{plugin}`"
            ));
        }
    }
    Ok(())
}

fn plugin(root: &Path, name: &str, required: &[&str]) -> Result<(), String> {
    let directory = root.join("plugins").join(name);
    for relative in [".codex-plugin/plugin.json", ".claude-plugin/plugin.json"] {
        let document = json(&directory.join(relative))?;
        if document.get("name").and_then(serde_json::Value::as_str) != Some(name) {
            return Err(format!("{name}/{relative} carries another plugin name"));
        }
        if document.get("version").and_then(serde_json::Value::as_str)
            != Some(env!("CARGO_PKG_VERSION"))
        {
            return Err(format!(
                "{name}/{relative} does not carry workspace version {}",
                env!("CARGO_PKG_VERSION")
            ));
        }
    }
    for relative in required {
        if !directory.join(relative).is_file() {
            return Err(format!("plugin `{name}` is missing `{relative}`"));
        }
    }
    Ok(())
}

/// The frontmatter keys every plan critic has to declare.
const CRITIC_PINS: &[&str] = &["model", "effort"];

/// Read the YAML frontmatter block of a markdown file, without a YAML parser: the block is the
/// lines between the first `---` and the next one, and a file that does not open with `---` has
/// none at all.
fn frontmatter(text: &str) -> Option<&str> {
    let body = text.strip_prefix("---\n")?;
    let end = body.find("\n---")?;
    Some(&body[..=end])
}

/// A plan critic states what it costs to run. `model:` and `effort:` decide which model answers a
/// plan critique and how hard it thinks, and a critic that declares neither is routed by whatever
/// the calling session happened to be on — so four verdicts arrive with no comparable price and no
/// way to tell an expensive panel from a cheap one after the fact.
///
/// Read as text rather than parsed: the keys are single-line scalars at the top level of the block,
/// and a YAML dependency for two `key:` prefixes would be a parser to keep in step with whichever
/// one each harness uses.
fn critic_pins(root: &Path) -> Result<(), String> {
    let directory = root.join("plugins/aep-plan/agents");
    let mut entries = std::fs::read_dir(&directory)
        .map_err(|error| format!("reading {}: {error}", directory.display()))?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("reading {}: {error}", directory.display()))?
        .into_iter()
        .filter(|path| {
            let is_markdown = path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("md"));
            is_markdown
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with("plan-critic-"))
        })
        .collect::<Vec<_>>();
    entries.sort();

    if entries.is_empty() {
        return Err(format!(
            "no `plan-critic-*.md` under {}",
            directory.display()
        ));
    }

    for path in entries {
        let text = std::fs::read_to_string(&path)
            .map_err(|error| format!("reading {}: {error}", path.display()))?;
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
            .to_owned();
        let block = frontmatter(&text)
            .ok_or_else(|| format!("plan critic `{name}` has no frontmatter block"))?;
        for key in CRITIC_PINS {
            let declared = block.lines().any(|line| {
                line.strip_prefix(key)
                    .and_then(|rest| rest.strip_prefix(':'))
                    .is_some_and(|value| !value.trim().is_empty())
            });
            if !declared {
                return Err(format!(
                    "plan critic `{name}` declares no `{key}:` in its frontmatter"
                ));
            }
        }
    }
    Ok(())
}

/// A plugin name this repository used to publish, and the spelling that replaced it.
struct Retired {
    /// What it was called.
    old: &'static str,
    /// What it is called now.
    new: &'static str,
    /// Characters that, following the name, mean it is not a plugin reference at all: `adp/1` is a
    /// protocol id and `adp/default` a workflow id, both wire identifiers in `aep`'s formats that
    /// belong to the profile rather than to the plugin, and neither is this repository's to rename.
    ///
    /// A `/` in *front* outweighs it, because `plugins/adp/skills/wave` is a path into the plugin
    /// directory and is exactly the reference this sweep is looking for.
    wire_next: &'static [char],
}

/// Every plugin name this repository has retired.
///
/// `AGENTS.md` § *Invariants* — *"Do not mention or depend on retired plugin references, former
/// marketplace identities, or the historical source-repository name"* — was, until this table
/// existed, an invariant a person upheld by remembering to run `rg`. The 0.6.2 rename found 173
/// occurrences across nine kinds of file, and the one that survives such a sweep is not caught by
/// the gate: it is caught by an adopter pasting an install line for a plugin the marketplace no
/// longer offers.
const RETIRED: &[Retired] = &[
    Retired {
        old: "aep-planning",
        new: "aep-plan",
        wire_next: &[],
    },
    Retired {
        old: "ess-schema",
        new: "ess-specify",
        wire_next: &[],
    },
    Retired {
        old: "adp",
        new: "aep-drive",
        wire_next: &['/'],
    },
];

/// Directories never walked looking for a retired name, wherever they sit.
const RETIRED_SKIP_DIRS: &[&str] = &[".git", "target", "node_modules"];

/// Repository-relative prefixes a retired name is allowed to survive under.
///
/// A changelog records what the names *were* and a dated change record is what was written on the
/// day; rewriting either is rewriting history rather than renaming a plugin. `.engineering/` is the
/// planning store, whose only writer is the `aep` CLI. And a recorded transcript under
/// `evals/*/recorded/` — with the manifest that dates it and the README that says how it was
/// produced — is evidence of a run that happened under the old names: editing one would be
/// falsifying an observation, not updating a reference.
///
/// The last entry is this file, which has to spell what it forbids in [`RETIRED`] above. It is not a
/// loophole for a real reference: every plugin name in this file is a name [`plugin`] and
/// [`marketplace`] resolve against the tree, or a path [`critic_pins`] opens, so a stale one here
/// fails a check rather than escaping one.
const RETIRED_ALLOWED: &[&str] = &[
    "CHANGELOG.md",
    "changes/",
    ".engineering/",
    "crates/agentplugins-check/src/main.rs",
];

/// Whether a byte can be part of the same word as a name, so `handpicked` does not read as `adp`.
fn word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// What a line says about itself when the old name on it is a quotation, not a reference.
///
/// One case exists and it is `evals/golden-path-end-to-end/expectations.trace.yaml`: an
/// `env.agent_available` row states what the *recorded* harness listed, and that recording was made
/// before the names changed. Renaming the row does not update a reference — it asserts something the
/// transcript beside it disproves, which is how two gating rows went red and how the replay gate
/// above came to exist. The marker is on the line and in the diff, so extending it is a decision
/// somebody makes in the open rather than a file quietly falling out of the sweep.
///
/// **It exempts nothing outside [`quotable`].** A marker any file could carry is an opt-out from the
/// sweep with no reviewer attached: a README, an install block or a manifest could excuse itself in
/// one trailing comment. The claim it makes — *this spelling is what the transcript beside it
/// contains* — is only true where a transcript is beside it, and that is one shape of file.
const RECORDED_SPELLING: &str = "recorded-under-this-name";

/// Whether a file is one whose rows may quote the spelling a recording used.
///
/// `evals/<case>/expectations.trace.yaml` and nothing else: the specification a recorded transcript
/// is replayed against, in the directory that holds the transcript.
fn quotable(relative: &str) -> bool {
    matches!(
        relative.split('/').collect::<Vec<_>>().as_slice(),
        ["evals", _, "expectations.trace.yaml"]
    )
}

/// The 1-based line numbers on which `text` names `retired` as something other than a wire id.
///
/// `quotable` says whether a line carrying [`RECORDED_SPELLING`] is exempt, which is a fact about
/// the file rather than about the line — see that constant.
fn retired_hits(text: &str, retired: &Retired, quotable: bool) -> Vec<usize> {
    let mut lines = Vec::new();
    for (number, line) in text.lines().enumerate() {
        if quotable && line.contains(RECORDED_SPELLING) {
            continue;
        }
        let bytes = line.as_bytes();
        let mut from = 0;
        while let Some(offset) = line[from..].find(retired.old) {
            let start = from + offset;
            let end = start + retired.old.len();
            from = end;
            if start > 0 && word_byte(bytes[start - 1]) {
                continue;
            }
            let next = bytes.get(end).copied();
            if next.is_some_and(word_byte) {
                continue;
            }
            let inside_a_path = start > 0 && bytes[start - 1] == b'/';
            if !inside_a_path
                && next.is_some_and(|byte| retired.wire_next.contains(&char::from(byte)))
            {
                continue;
            }
            lines.push(number + 1);
            break;
        }
    }
    lines
}

/// Where a path segment spelling a retired plugin is a reference and not a coincidence.
///
/// The three trees whose directory layout *is* the plugin's identity: `plugins/<name>` is what
/// `AGENTS.md` § *Invariants* means by *"plugin folder names and manifest names are identical"*,
/// `website/docs/plugins/<name>.md` is the doc id an adopter links, and `evals/<case>` is the id a
/// case carries. Elsewhere a segment is prose in a filename and the content sweep is the right
/// reader for it.
const RETIRED_PATH_ROOTS: &[&str] = &["plugins/", "website/docs/plugins/", "evals/"];

/// Whether a repository-relative path is filed under a retired plugin name.
///
/// Files match on any segment — an incomplete `git mv` leaves the parent renamed and one file
/// behind, and the file's own text need not spell anything. Directories match on their own last
/// segment only, so one leftover tree is named once rather than once per level.
///
/// The final segment is compared with a single extension stripped, because
/// `website/docs/plugins/<retired>.md` is the doc id and the `.md` is not part of it.
fn retired_path(relative: &str, is_dir: bool, retired: &Retired) -> bool {
    if !RETIRED_PATH_ROOTS
        .iter()
        .any(|root| relative.starts_with(root))
    {
        return false;
    }
    let segments: Vec<&str> = relative.split('/').collect();
    let last = segments.len() - 1;
    let bare = |index: usize| {
        let segment = segments[index];
        if index == last {
            segment.rsplit_once('.').map_or(segment, |(stem, _)| stem)
        } else {
            segment
        }
    };
    if is_dir {
        return bare(last) == retired.old;
    }
    (0..segments.len()).any(|index| bare(index) == retired.old)
}

/// No file this repository authors still names a plugin it stopped publishing — in its text, or in
/// where it sits.
///
/// Checked over the tree rather than over a list of known reference sites, because a list of sites
/// is the same hand-maintained thing the invariant already was: the file that gets missed is by
/// definition the one nobody thought to list.
///
/// **Both halves are needed, and the content half alone was not enough.** A rename that moves a
/// directory and misses one file inside it leaves a path that names the retired plugin and a body
/// that does not, and nothing else here reads it: [`marketplace`] checks the five entries it expects
/// are present and in order, and [`plugin`] resolves `plugins/<name>` for each of those five, so a
/// sixth directory beside them is read by neither.
fn retired_names(root: &Path) -> Result<(), String> {
    let mut found = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(directory) = stack.pop() {
        let mut entries = std::fs::read_dir(&directory)
            .map_err(|error| format!("reading {}: {error}", directory.display()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("reading {}: {error}", directory.display()))?
            .into_iter()
            .map(|entry| entry.path())
            .collect::<Vec<_>>();
        entries.sort();
        for path in entries {
            let Ok(relative) = path.strip_prefix(root) else {
                continue;
            };
            let relative = relative.to_string_lossy().replace('\\', "/");
            let name = path
                .file_name()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or_default();
            let is_dir = path.is_dir();
            let recorded = relative.starts_with("evals/") && name == "recorded";
            let allowed = RETIRED_ALLOWED
                .iter()
                .any(|allowed| format!("{relative}/").starts_with(allowed) || relative == *allowed);
            if !allowed && !recorded {
                for retired in RETIRED {
                    if retired_path(&relative, is_dir, retired) {
                        found.push(format!(
                            "  {relative} is filed under `{}`, which is now `{}`",
                            retired.old, retired.new
                        ));
                    }
                }
            }
            if is_dir {
                if !RETIRED_SKIP_DIRS.contains(&name) && !allowed && !recorded {
                    stack.push(path);
                }
                continue;
            }
            if allowed || recorded {
                continue;
            }
            let Ok(bytes) = std::fs::read(&path) else {
                continue;
            };
            let text = String::from_utf8_lossy(&bytes);
            let quotable = quotable(&relative);
            for retired in RETIRED {
                for line in retired_hits(&text, retired, quotable) {
                    found.push(format!(
                        "  {relative}:{line} names `{}`, which is now `{}`",
                        retired.old, retired.new
                    ));
                }
            }
        }
    }
    if found.is_empty() {
        return Ok(());
    }
    found.sort();
    Err(format!(
        "{} retired plugin name(s) remain, and `AGENTS.md` § Invariants forbids depending on \
         one:\n{}",
        found.len(),
        found.join("\n")
    ))
}

/// A command whose first level was regrouped into areas, and the verbs that moved.
///
/// AEP 0.52.0 and ESS 0.12.0 each replaced a flat list of first-level verbs with a small set of
/// areas — the same areas their crate trees are divided into — and kept every flat spelling as a
/// hidden alias with identical output. So nothing here is about a command that stopped working: it
/// is about what a document *teaches*, and a document that teaches the compatibility surface
/// teaches a reader a spelling neither `--help` nor either repository's own prose will confirm.
struct Regrouped {
    /// Every name the command is published under. `aep` and `protocol` are two names for one
    /// binary (`AGENTS.md` § *Invariants*), and a command matcher in the eval corpus spells both.
    tools: &'static [&'static str],
    /// Each verb that used to be spelled at the first level, and the area it is spelled under now.
    moved: &'static [(&'static str, &'static str)],
}

/// The two commands this repository's instructions drive, and where their first level went.
///
/// Read off the two command surfaces rather than off a release note: `aep`'s at
/// `crates/edge/aep-cli/src/app.rs` in tag `0.52.0` (`GovernCommand`, `PlanCommand`, `DriveGroup`
/// and `ObserveCommand`), and `ess`'s at `crates/edge/ess-cli/src/main.rs` in tag `0.12.0`
/// (`SpecifyCommand`, `GenerateCommand`, `VerifyCommand` and `ImportCommand`).
///
/// Three spellings are deliberately absent, because none of them moved:
///
/// * `aep drive` and `aep doctor`. `drive run` was always spelled `drive run`; `doctor` belongs to
///   no area.
/// * `ess generate`. The area and the verb share a name, so `ess generate --path …` is what it
///   always was — while `ess schema`, a sibling of that verb, is now `ess generate schema`.
const REGROUPED: &[Regrouped] = &[
    Regrouped {
        tools: &["aep", "protocol"],
        moved: &[
            ("validate", "govern"),
            ("resolve", "govern"),
            ("inspect", "govern"),
            ("evaluate", "govern"),
            ("explain", "govern"),
            ("describe", "govern"),
            ("schema", "govern"),
            ("workflow", "govern"),
            ("artifact", "plan"),
            ("serve", "plan"),
            ("entity", "plan"),
            ("audit", "plan"),
            ("workspace", "plan"),
            ("conformance", "plan"),
            ("reverse", "plan"),
            ("eval", "drive"),
            ("trace", "observe"),
            ("contract", "observe"),
            ("property", "observe"),
            ("specification", "observe"),
            ("evidence", "observe"),
        ],
    },
    Regrouped {
        tools: &["ess"],
        moved: &[
            ("validate", "specify"),
            ("compile", "specify"),
            ("compose", "specify"),
            ("inspect", "specify"),
            ("graph", "specify"),
            ("realization", "specify"),
            ("runtime", "specify"),
            ("synthesize", "generate"),
            ("project", "generate"),
            ("schema", "generate"),
            ("build", "generate"),
            ("component", "generate"),
            ("release", "generate"),
            ("stack", "generate"),
            ("deployment", "generate"),
            ("conform", "verify"),
            ("diff", "verify"),
            ("impact", "verify"),
            ("import", "infra"),
        ],
    },
];

/// The extensions an authored document carries here: markdown prose, and the YAML the eval corpus
/// is written in. Rust sources are not documents — see [`FLAT_ALLOWED`].
const DOCUMENT_EXTENSIONS: &[&str] = &["md", "yaml", "yml"];

/// Repository-relative prefixes where a flat spelling is not a lesson.
///
/// A changelog entry says what a command *was* called and a dated change record is what was written
/// on the day; `.engineering/` is the planning store, whose only writer is the `aep` CLI.
///
/// `.github/workflows/` is the fourth and the only one that is not a record. It is a program, and
/// it pins the binary it runs — `AEP_VERSION: '0.44.0'` in `eval.yml`, *"an eval whose runner moved
/// between two runs measured two things"* — so its spelling has to be the surface that version
/// actually has, which is the flat one. A workflow that typed the grouped spelling would not teach
/// a reader anything; it would fail to parse an argument on the runner. The comments beside those
/// calls are exempt with them, because a comment describing the call below it in another spelling
/// is worse than either.
const FLAT_ALLOWED: &[&str] = &[
    "CHANGELOG.md",
    "changes/",
    ".engineering/",
    ".github/workflows/",
];

/// A line with the regex spellings a command matcher is written in read back as the command a
/// person would type.
///
/// `evals/*/expectations.trace.yaml` selects a call with a regex over both names of the binary and
/// over runs of spaces — `'(aep|protocol) +artifact +new'` — so the flat spelling a row teaches is
/// invisible to a reader looking for `aep artifact`. Three substitutions put it back: the
/// alternation is one of the two names, and `\s+` and ` +` are a space. A row already widened to
/// accept the grouped spelling reads `aep (plan )?artifact` after this and carries no flat
/// spelling, which is the point — the widening is what makes the row match either.
fn as_typed(line: &str) -> String {
    line.replace("(aep|protocol)", "aep")
        .replace("\\s+", " ")
        .replace("\\s", " ")
        .replace(" +", " ")
}

/// The 1-based line numbers on which `text` spells a first-level verb flat, each with the area it
/// is spelled under now.
///
/// A hit is the command's name, a space, and a moved verb — nothing else. `aep plan artifact new`
/// is clean because `plan` is not a verb that moved, `aep drive eval run` because `drive` is not
/// either, and `ess generate project` because the word after `ess` is `generate`, which stayed. A
/// name inside another word (`process`, `aep-cli`, `ess-specify`) is not the command.
fn flat_hits(text: &str, group: &Regrouped) -> Vec<(usize, &'static str, &'static str)> {
    let mut hits = Vec::new();
    for (number, line) in text.lines().enumerate() {
        let typed = as_typed(line);
        let bytes = typed.as_bytes();
        for tool in group.tools {
            let mut from = 0;
            while let Some(offset) = typed[from..].find(tool) {
                let start = from + offset;
                let end = start + tool.len();
                from = end;
                if start > 0 && (word_byte(bytes[start - 1]) || bytes[start - 1] == b'-') {
                    continue;
                }
                let Some(rest) = typed[end..].strip_prefix(' ') else {
                    continue;
                };
                let rest = rest.trim_start_matches(' ');
                let width = rest
                    .find(|character: char| !character.is_ascii_alphanumeric() && character != '-')
                    .unwrap_or(rest.len());
                let Some((verb, area)) =
                    group.moved.iter().find(|(verb, _)| *verb == &rest[..width])
                else {
                    continue;
                };
                hits.push((number + 1, *verb, *area));
            }
        }
    }
    hits
}

/// Whether a repository-relative path is a recorded transcript or the manifest that dates it.
///
/// Everything under `evals/<case>/recorded/` except its `README.md`, which is not evidence of a run
/// — it is the instructions for making one, and it is read by a person who then types them.
fn transcript(relative: &str) -> bool {
    matches!(
        relative.split('/').collect::<Vec<_>>().as_slice(),
        ["evals", _, "recorded", rest @ ..] if rest.last() != Some(&"README.md")
    )
}

/// No authored document teaches a first-level verb the two CLIs no longer print.
///
/// Over the tree, for the reason the sibling sweep above is: a list of the documents that name a
/// command is the hand-maintained thing that misses the one nobody thought to list. The 0.52.0 and
/// 0.12.0 groupings reached twenty-two files here, and every flat spelling still works — so nothing
/// fails, nothing warns, and a document teaching the old surface is invisible to every check that
/// runs a command.
fn flat_spellings(root: &Path) -> Result<(), String> {
    let mut found = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(directory) = stack.pop() {
        let mut entries = std::fs::read_dir(&directory)
            .map_err(|error| format!("reading {}: {error}", directory.display()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("reading {}: {error}", directory.display()))?
            .into_iter()
            .map(|entry| entry.path())
            .collect::<Vec<_>>();
        entries.sort();
        for path in entries {
            let Ok(relative) = path.strip_prefix(root) else {
                continue;
            };
            let relative = relative.to_string_lossy().replace('\\', "/");
            let name = path
                .file_name()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or_default();
            let allowed = FLAT_ALLOWED
                .iter()
                .any(|allowed| format!("{relative}/").starts_with(allowed) || relative == *allowed);
            if path.is_dir() {
                if !RETIRED_SKIP_DIRS.contains(&name) && !allowed {
                    stack.push(path);
                }
                continue;
            }
            if allowed || transcript(&relative) {
                continue;
            }
            let extension = path
                .extension()
                .and_then(|extension| extension.to_str())
                .unwrap_or_default();
            if !DOCUMENT_EXTENSIONS.contains(&extension) {
                continue;
            }
            let Ok(bytes) = std::fs::read(&path) else {
                continue;
            };
            let text = String::from_utf8_lossy(&bytes);
            for group in REGROUPED {
                for (line, verb, area) in flat_hits(&text, group) {
                    found.push(format!(
                        "  {relative}:{line} teaches `{} {verb}`, which is now `{} {area} {verb}`",
                        group.tools[0], group.tools[0]
                    ));
                }
            }
        }
    }
    if found.is_empty() {
        return Ok(());
    }
    found.sort();
    Err(format!(
        "{} flat CLI spelling(s) remain in authored documents, and `AGENTS.md` § Invariants says \
         the grouped spelling is the authored one:\n{}",
        found.len(),
        found.join("\n")
    ))
}

fn check(root: &Path) -> Result<(), String> {
    marketplace(root, ".agents/plugins/marketplace.json")?;
    marketplace(root, ".claude-plugin/marketplace.json")?;
    for (name, required) in PLUGINS {
        plugin(root, name, required)?;
    }
    critic_pins(root)?;
    retired_names(root)?;
    flat_spellings(root)?;
    evals::evals(root)
}

fn verify_release(root: &Path, version: &str) -> Result<(), String> {
    if version != env!("CARGO_PKG_VERSION") {
        return Err(format!(
            "tag `{version}` does not match workspace version {}",
            env!("CARGO_PKG_VERSION")
        ));
    }

    let changelog = std::fs::read_to_string(root.join("CHANGELOG.md"))
        .map_err(|error| format!("reading CHANGELOG.md: {error}"))?;
    let heading = format!("## [{version}] — ");
    if !changelog.lines().any(|line| line.starts_with(&heading)) {
        return Err(format!("CHANGELOG.md has no dated `{heading}` heading"));
    }

    let object_type = Command::new("git")
        .args(["cat-file", "-t", &format!("refs/tags/{version}")])
        .current_dir(root)
        .output()
        .map_err(|error| format!("running git cat-file: {error}"))?;
    if !object_type.status.success() || object_type.stdout != b"tag\n" {
        return Err(format!("`{version}` is missing or is not an annotated tag"));
    }

    Ok(())
}

fn main() -> ExitCode {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("checker is under the repository root")
        .to_path_buf();
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if let Some(code) = evals::cli(&root, &arguments) {
        return code;
    }
    let result = match arguments.as_slice() {
        [] => check(&root),
        [release, verify, version] if release == "release" && verify == "verify" => {
            check(&root).and_then(|()| verify_release(&root, version))
        }
        _ => Err(
            "usage: agentplugins-check [release verify <version> | evals | evals scope <path>...]"
                .to_owned(),
        ),
    };

    match result {
        Ok(()) => {
            println!(
                "valid: marketplace beyond10x, {} focused plugin(s)",
                PLUGINS.len()
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::from(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_committed_marketplace_is_valid() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("checker is under repository root");
        check(root).expect("the committed marketplace validates");
    }

    /// The four critics and the rubric they share are required content, not optional extras: a
    /// panel missing one perspective still returns four confident verdicts, and nothing else in
    /// this repository would notice the file had gone.
    #[test]
    fn a_deleted_critic_fails_the_check() {
        let required: &[&str] = PLUGINS
            .iter()
            .find(|(name, _)| *name == "aep-plan")
            .map(|(_, required)| *required)
            .expect("aep-plan is one of the focused plugins");
        let critic = "agents/plan-critic-design.md";
        assert!(
            required.contains(&critic),
            "the critic panel must be required content"
        );

        let repository = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("checker is under repository root");
        let sandbox =
            std::env::temp_dir().join(format!("agentplugins-check-critic-{}", std::process::id()));
        let plugin_root = sandbox.join("plugins").join("aep-plan");
        let write = |target: &Path, bytes: &str| {
            std::fs::create_dir_all(target.parent().expect("every entry has a directory"))
                .expect("the sandbox is writable");
            std::fs::write(target, bytes).expect("the sandbox is writable");
        };
        for manifest in [".codex-plugin/plugin.json", ".claude-plugin/plugin.json"] {
            let committed =
                std::fs::read_to_string(repository.join("plugins/aep-plan").join(manifest))
                    .expect("the committed manifest is readable");
            write(&plugin_root.join(manifest), &committed);
        }
        for relative in required.iter().filter(|relative| **relative != critic) {
            write(&plugin_root.join(relative), "");
        }

        let error = plugin(&sandbox, "aep-plan", required)
            .expect_err("a plugin missing one of its critics must fail the check");
        std::fs::remove_dir_all(&sandbox).expect("the sandbox is removable");
        assert_eq!(error, format!("plugin `aep-plan` is missing `{critic}`"));
    }

    /// A critic that declares no `model:`/`effort:` runs on whatever the calling session was on,
    /// which is the state 0.4.0 shipped in and which nothing here noticed. The check reads the
    /// frontmatter block, so this test exercises the reader on the shapes it has to tell apart
    /// rather than the four committed files, which the check below already covers.
    #[test]
    fn a_critic_without_both_pins_fails_the_check() {
        let sandbox = std::env::temp_dir().join(format!(
            "agentplugins-check-pins-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let agents = sandbox.join("plugins/aep-plan/agents");
        let write = |name: &str, body: &str| {
            std::fs::create_dir_all(&agents).expect("the sandbox is writable");
            std::fs::write(agents.join(name), body).expect("the sandbox is writable");
        };

        let pinned = "---\nname: plan-critic-design\nmodel: sonnet\neffort: high\n---\n\nbody\n";
        write("plan-critic-design.md", pinned);
        critic_pins(&sandbox).expect("a critic carrying both keys passes");

        write(
            "plan-critic-design.md",
            "---\nname: plan-critic-design\nmodel: sonnet\n---\n\nbody\n",
        );
        assert_eq!(
            critic_pins(&sandbox).expect_err("a critic missing `effort:` must fail"),
            "plan critic `plan-critic-design.md` declares no `effort:` in its frontmatter"
        );

        write(
            "plan-critic-design.md",
            "---\nname: plan-critic-design\nmodel:\neffort: high\n---\n\nbody\n",
        );
        assert_eq!(
            critic_pins(&sandbox).expect_err("a key with no value is not a declaration"),
            "plan critic `plan-critic-design.md` declares no `model:` in its frontmatter"
        );

        write("plan-critic-design.md", "no frontmatter here\n");
        assert_eq!(
            critic_pins(&sandbox).expect_err("a critic with no frontmatter must fail"),
            "plan critic `plan-critic-design.md` has no frontmatter block"
        );

        std::fs::remove_dir_all(&sandbox).expect("the sandbox is removable");
    }

    /// The committed critics carry the pin. This is the half of the check that would catch a fifth
    /// critic added without one.
    #[test]
    fn every_committed_critic_declares_its_pin() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("checker is under repository root");
        critic_pins(root).expect("every committed plan critic declares `model:` and `effort:`");
    }

    /// The rename is only done when nothing still names the old plugins. Over the tree, not over a
    /// list of reference sites: `aep-planning`, `adp` and `ess-schema` were named by both
    /// marketplace manifests, ten plugin manifests, nine skill and agent bodies, eight eval cases,
    /// eight expectation documents, five website pages, three website sources, this crate and two
    /// READMEs, and the one that survives a hand sweep is the one no list had.
    #[test]
    fn no_authored_file_names_a_retired_plugin() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("checker is under repository root");
        retired_names(root).expect("nothing this repository authors names a retired plugin");
    }

    /// What the sweep counts as a reference. `adp` is three letters, so it has to be a whole word
    /// and it has to not be the `adp/…` wire id — the profile the plugin drives keeps its
    /// identifier in `aep`'s formats, and a rename that touched `workflow: adp/default` would have
    /// invalidated every case document in the corpus.
    #[test]
    fn the_sweep_reads_a_reference_and_not_a_wire_id() {
        let adp = &RETIRED[2];
        assert_eq!(adp.old, "adp");

        assert_eq!(
            retired_hits("install the adp plugin\n", adp, false),
            vec![1]
        );
        assert_eq!(retired_hits("plugins/adp/skills\n", adp, false), vec![1]);
        assert!(
            retired_hits("workflow: adp/default\n", adp, false).is_empty(),
            "`adp/default` is a workflow id, not a plugin"
        );
        assert!(
            retired_hits("the protocol adp/1\n", adp, false).is_empty(),
            "`adp/1` is a protocol id, not a plugin"
        );
        assert!(
            retired_hits("handpicked madpeople\n", adp, false).is_empty(),
            "three letters inside a word are not a plugin name"
        );

        let planning = &RETIRED[0];
        assert_eq!(planning.old, "aep-planning");
        assert_eq!(
            retired_hits("a\nplugins/aep-planning/x\n", planning, false),
            vec![2]
        );
        assert!(retired_hits("plugins/aep-plan/x\n", planning, false).is_empty());
    }

    /// The marker excuses a line in a specification a transcript is replayed against, and nowhere
    /// else. A marker any file could carry is an opt-out from the sweep with no reviewer attached.
    #[test]
    fn the_recorded_spelling_marker_is_confined_to_an_expectations_document() {
        let planning = &RETIRED[0];
        let row = format!("  agent: aep-planning:decomposer  # {RECORDED_SPELLING}\n");

        assert!(
            retired_hits(&row, planning, true).is_empty(),
            "a quoted recording is evidence, not a stale reference"
        );
        assert_eq!(
            retired_hits(&row, planning, false),
            vec![1],
            "the same comment in a README is an install line excusing itself"
        );

        assert!(quotable(
            "evals/golden-path-end-to-end/expectations.trace.yaml"
        ));
        for elsewhere in [
            "README.md",
            "evals/README.md",
            "evals/golden-path-end-to-end/case.yaml",
            "evals/golden-path-end-to-end/recorded/expectations.trace.yaml",
            "website/docs/install.md",
            "expectations.trace.yaml",
        ] {
            assert!(!quotable(elsewhere), "{elsewhere}");
        }
    }

    /// A retired name that reaches a file is a failing gate and not a warning, and the failure says
    /// which file and which line — the whole point of moving this off `rg` and into the gate.
    #[test]
    fn a_retired_name_in_an_authored_file_fails_the_check() {
        let sandbox = std::env::temp_dir().join(format!(
            "agentplugins-check-retired-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let write = |relative: &str, body: &str| {
            let target = sandbox.join(relative);
            std::fs::create_dir_all(target.parent().expect("every entry has a directory"))
                .expect("the sandbox is writable");
            std::fs::write(target, body).expect("the sandbox is writable");
        };

        write("README.md", "install `ess-schema` from the marketplace\n");
        let error =
            retired_names(&sandbox).expect_err("a retired name in an authored file must fail");
        assert_eq!(
            error,
            "1 retired plugin name(s) remain, and `AGENTS.md` § Invariants forbids depending on \
             one:\n  README.md:1 names `ess-schema`, which is now `ess-specify`"
        );

        // The three places the old names are the truth: what the changelog says the plugins were
        // called, what a dated change record said on its day, and the transcript of a run that
        // happened under them.
        write("README.md", "install `ess-specify` from the marketplace\n");
        write("CHANGELOG.md", "renamed `ess-schema` to `ess-specify`\n");
        write("changes/2026-09-03-rename.yaml", "plugin: adp\n");
        write(
            "evals/a-case/recorded/run.events.jsonl",
            "{\"skill\":\"adp:wave\"}\n",
        );
        write(
            ".engineering/planning/story/x.md",
            "the `aep-planning` plugin\n",
        );
        retired_names(&sandbox).expect("history, change records and transcripts keep their names");

        std::fs::remove_dir_all(&sandbox).expect("the sandbox is removable");
    }

    /// The name a file is *filed under* is a reference too, and the sweep never looks at one.
    ///
    /// [`retired_names`] reads bytes and never paths, so the leftover an incomplete `git mv` makes
    /// — the parent moved, one file left behind under the old directory — survives it whenever
    /// that file's own text does not happen to spell the name. Nothing else in this crate closes
    /// it: [`marketplace`] checks that the five entries it expects are present and in order, and
    /// [`plugin`] resolves `plugins/<name>` for each of those five, so neither reads a sixth
    /// directory beside them. `AGENTS.md` § *Invariants* — *"Plugin folder names and manifest
    /// names are identical"* and *"Do not mention or depend on retired plugin references"* — is
    /// about the folder as much as about the prose.
    ///
    /// Measured on a copy of this worktree in a scratch directory, 2026-09-03: a
    /// `plugins/<retired>/skills/wave/SKILL.md` whose body never spells the retired name, and a
    /// `website/docs/plugins/<retired>.md` whose body never spells it either, both left
    /// `cargo run --bin agentplugins-check` printing
    /// `valid: marketplace beyond10x, 5 focused plugin(s)`.
    ///
    /// The retired spelling is assembled rather than written, so that the sibling sweep over the
    /// tree counts the same number of lines with this test present as without it.
    #[test]
    fn a_path_named_for_a_retired_plugin_fails_the_check() {
        let sandbox = std::env::temp_dir().join(format!(
            "agentplugins-check-retired-path-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let write = |relative: &str, body: &str| {
            let target = sandbox.join(relative);
            std::fs::create_dir_all(target.parent().expect("every entry has a directory"))
                .expect("the sandbox is writable");
            std::fs::write(target, body).expect("the sandbox is writable");
        };
        let retired = RETIRED[2].old;

        // A leftover from a rename that moved the parent and missed one file. Its text names the
        // new world; only where it sits still names the old one.
        let plugin_file = format!("plugins/{retired}/skills/wave/SKILL.md");
        let page = format!("website/docs/plugins/{retired}.md");
        write(
            &plugin_file,
            "---\nname: wave\ndescription: run a wave\n---\n\nUse the renamed plugin.\n",
        );
        write(
            &page,
            "---\ntitle: AEP Drive\n---\n\n# The renamed plugin\n",
        );

        let error = retired_names(&sandbox);
        std::fs::remove_dir_all(&sandbox).expect("the sandbox is removable");
        let error =
            error.expect_err("a path filed under a retired plugin name must fail the check");
        assert!(
            error.contains(&plugin_file) && error.contains(&page),
            "{error}"
        );
    }

    /// The grouping is only migrated when no authored document still teaches the flat surface.
    /// Over the tree, and not over a list of the documents that name a command: the flat spellings
    /// reached skills, agent charters, references, the website, both READMEs and the eval corpus,
    /// and every one of them still runs, so the one a hand sweep misses is caught by nothing.
    #[test]
    fn no_authored_document_teaches_a_flat_cli_spelling() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("checker is under repository root");
        flat_spellings(root).expect("every authored document uses the grouped spelling");
    }

    /// What the sweep counts as a flat spelling. The grouped forms have to read clean, or the
    /// migration cannot land; the words that merely contain a command's name have to read clean,
    /// or the sweep is noise; and the regex form a command matcher is written in has to read as
    /// the command it selects, or the eval corpus keeps teaching what the skills stopped teaching.
    #[test]
    fn the_flat_sweep_reads_a_typed_command_and_not_a_word() {
        let aep = &REGROUPED[0];
        assert_eq!(aep.tools, &["aep", "protocol"]);
        let ess = &REGROUPED[1];
        assert_eq!(ess.tools, &["ess"]);

        assert_eq!(
            flat_hits("$ aep artifact new story x\n", aep),
            vec![(1, "artifact", "plan")]
        );
        assert_eq!(
            flat_hits("run `protocol reverse scan`\n", aep),
            vec![(1, "reverse", "plan")]
        );
        assert_eq!(
            flat_hits("a\n$ aep eval run --corpus evals\n", aep),
            vec![(2, "eval", "drive")]
        );
        assert_eq!(
            flat_hits("$ ess validate --path .\n", ess),
            vec![(1, "validate", "specify")]
        );
        assert_eq!(
            flat_hits("$ ess project openapi\n", ess),
            vec![(1, "project", "generate")]
        );

        for grouped in [
            "$ aep plan artifact new story x\n",
            "$ aep drive eval run --corpus evals\n",
            "$ aep drive run --workflow adp/default\n",
            "$ aep doctor\n",
            "$ protocol plan reverse scan\n",
        ] {
            assert!(flat_hits(grouped, aep).is_empty(), "{grouped}");
        }
        for grouped in [
            "$ ess specify validate --path .\n",
            "$ ess generate --path . --kind schema\n",
            "$ ess generate project openapi\n",
            "$ ess verify conform\n",
        ] {
            assert!(flat_hits(grouped, ess).is_empty(), "{grouped}");
        }

        // A command's name inside another word, inside a path, or inside a plugin id.
        for elsewhere in [
            "cargo build -p aep-cli\n",
            "the plugin aep-drive\n",
            "beyond10x/agentplugins@ess-specify@0.7.0\n",
            "the process validates nothing\n",
            "an aeproject\n",
        ] {
            assert!(flat_hits(elsewhere, aep).is_empty(), "{elsewhere}");
            assert!(flat_hits(elsewhere, ess).is_empty(), "{elsewhere}");
        }

        // The shape a `trace-spec/1` command matcher is written in, before and after widening.
        assert_eq!(
            flat_hits(
                "          command: {regex: '(aep|protocol) artifact +body'}\n",
                aep
            ),
            vec![(1, "artifact", "plan")]
        );
        assert!(
            flat_hits(
                "          command: {regex: '(aep|protocol) +(plan +)?artifact +body'}\n",
                aep
            )
            .is_empty(),
            "a row widened to accept either spelling teaches neither"
        );
    }

    /// A flat spelling that reaches an authored document is a failing gate naming the file, the
    /// line and the grouped spelling — and the records that keep theirs keep them.
    #[test]
    fn a_flat_spelling_in_an_authored_document_fails_the_check() {
        let sandbox = std::env::temp_dir().join(format!(
            "agentplugins-check-flat-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&sandbox);
        let write = |relative: &str, body: &str| {
            let target = sandbox.join(relative);
            std::fs::create_dir_all(target.parent().expect("every entry has a directory"))
                .expect("the sandbox is writable");
            std::fs::write(target, body).expect("the sandbox is writable");
        };

        write(
            "README.md",
            "Run `aep artifact validate` before you stop.\n",
        );
        assert_eq!(
            flat_spellings(&sandbox).expect_err("a flat spelling in an authored document fails"),
            "1 flat CLI spelling(s) remain in authored documents, and `AGENTS.md` § Invariants \
             says the grouped spelling is the authored one:\n  README.md:1 teaches `aep \
             artifact`, which is now `aep plan artifact`"
        );

        // The four places the flat spelling is the record: what the changelog says the command
        // was, what a dated change record said on its day, the planning store the CLI writes, and
        // a transcript of a run made under it. And one place it is not: the README beside that
        // transcript, which is instructions a person types.
        write(
            "README.md",
            "Run `aep plan artifact validate` before you stop.\n",
        );
        write(
            "CHANGELOG.md",
            "`aep artifact` is now `aep plan artifact`\n",
        );
        write("changes/2026-09-04-grouping.yaml", "was: aep eval run\n");
        write(".engineering/planning/story/x.md", "run `ess validate`\n");
        write(
            "evals/a-case/recorded/run.manifest.yaml",
            "command: ess validate --path .\n",
        );
        flat_spellings(&sandbox).expect("history, change records and transcripts keep theirs");

        write(
            "evals/a-case/recorded/README.md",
            "```console\n$ aep eval run --case evals/a-case\n```\n",
        );
        let error = flat_spellings(&sandbox)
            .expect_err("the instructions beside a transcript are typed by a person");
        std::fs::remove_dir_all(&sandbox).expect("the sandbox is removable");
        assert!(
            error.contains("evals/a-case/recorded/README.md:2 teaches `aep eval`"),
            "{error}"
        );
    }

    #[test]
    fn a_release_version_must_match_the_workspace() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("checker is under repository root");
        let error = verify_release(root, "9.9.9").expect_err("a mismatched version must fail");
        assert!(error.contains("does not match workspace version"));
    }
}
