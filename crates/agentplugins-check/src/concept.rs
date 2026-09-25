//! The plugin concept in `website/docs/structure.md`, enforced. Each rule below is one row of that
//! page; a structure decision that is not checked here drifts, which is how two AEP plugins shipped
//! after one had been agreed.
//!
//! - **R2** one plugin per product; plugin name = product id = the CLI it drives.
//! - **R3** a skill is an activity named in `-ing` form, one or two words, never its plugin's name.
//! - **R4** an agent is owned by exactly one skill of its plugin, which lists it under `## Agents`.
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

/// Skills and agents of one carried plugin, with R3 and R4 applied.
fn plugin(
    root: &Path,
    name: &str,
    problems: &mut Vec<String>,
) -> (BTreeSet<String>, BTreeSet<String>) {
    let directory = root.join("plugins").join(name);
    let mut skills = BTreeSet::new();
    let mut owners: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for path in entries(&directory.join("skills")) {
        let Some(folder) = path.file_name().and_then(|n| n.to_str()).map(str::to_owned) else {
            continue;
        };
        let Ok(text) = read(&path.join("SKILL.md")) else {
            problems.push(format!("R3 `{name}:{folder}` has no SKILL.md"));
            continue;
        };
        if !LIFECYCLE.contains(&folder.as_str()) && !activity(&folder) {
            problems.push(format!(
                "R3 `{name}:{folder}` is not an activity name (one or two words, the first ending in `ing`)"
            ));
        }
        if !LIFECYCLE.contains(&folder.as_str()) && folder == name {
            problems.push(format!("R3 `{name}:{folder}` is named after its plugin"));
        }
        if frontmatter_name(&text).as_deref() != Some(folder.as_str()) {
            problems.push(format!(
                "R3 `{name}:{folder}` declares another `name:` in its frontmatter"
            ));
        }
        for agent in listed_agents(&text) {
            owners.entry(agent).or_default().push(folder.clone());
        }
        skills.insert(folder);
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
        let text = read(&path).unwrap_or_default();
        if frontmatter_name(&text).as_deref() != Some(stem.as_str()) {
            problems.push(format!("R4 agent `{name}:{stem}` declares another `name:`"));
        }
        match owners.get(&stem).map(Vec::as_slice) {
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

    #[test]
    fn cli_versions_are_found_and_skill_versions_are_not() {
        assert_eq!(quoted_versions("at AEP 0.55.0 the verb"), ["aep 0.55.0"]);
        assert_eq!(quoted_versions("output of ESS `0.29.0`"), ["ess 0.29.0"]);
        assert!(quoted_versions("**Skill version 0.13.1** — the version").is_empty());
        assert!(quoted_versions("ess-cli 0.30.0 is not a quote of a CLI name").is_empty());
        assert!(quoted_versions("aep plan artifact list").is_empty());
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
