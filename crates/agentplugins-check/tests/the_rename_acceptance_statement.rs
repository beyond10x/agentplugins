//! The acceptance statement of `story:rename-plugins-to-product-and-verb`, asserted directly.
//!
//! # Why this file is kept rather than deleted
//!
//! The unit's own `retired_names` sweep and its unit tests assert that the sweep *finds nothing*
//! and that its rules behave as written. They cannot assert the half the story's amended statement
//! actually turns on — **that the exemption set is exactly the listed one**. Add `"website/"` to
//! `RETIRED_ALLOWED` and leave a retired name in an install block: `no_authored_file_names_a_
//! retired_plugin` still passes, because it asks the sweep; every rule test still passes, because
//! none of them names `website/`; and the adopter still pastes an install line for a plugin the
//! marketplace no longer offers.
//!
//! So this is a **second, independent walk** of the same tree that applies only the exemptions the
//! story states, and compares the result with nothing. A sweep that quietly grows an exemption
//! passes its own tests and fails this one.
//!
//! # § *Acceptance*, transcribed
//!
//! > the `retired_names` sweep in `agentplugins-check` finds no retired plugin id in any authored
//! > file, with exactly these exemptions, each visible in a diff: the AEP wire ids `…/1` and
//! > `…/default`; expectation rows in `evals/*/expectations.trace.yaml` marked
//! > `# recorded-under-this-name` because the recording predates the rename; the sweep's own
//! > `RETIRED` table in `crates/agentplugins-check/src/main.rs`; `CHANGELOG.md`, `changes/`,
//! > `.engineering/` and `evals/*/recorded/` as records. The sweep also refuses a path segment
//! > equal to a retired name under `plugins/` and `website/docs/plugins/`.
//!
//! [`EXEMPT_FILES`] and [`EXEMPT_PREFIXES`] below are that list and nothing else. The table's
//! exemption is written here as the whole of `main.rs` because that is what the sweep exempts —
//! the file has to spell what it forbids in its table, its matcher doc and its rule tests, and
//! every plugin name in it is one `plugin`, `marketplace` or `critic_pins` resolves against the
//! tree, so a stale one there fails a check rather than escaping one.
//!
//! # Two deliberate differences from the `rg` command the statement used to quote
//!
//! **Hidden entries are walked**, except `.git`. `rg` does not walk them by default, which is why
//! the command in revision 2 of the story could not see either `marketplace.json` or any of the ten
//! `plugin.json` — the files the rename most had to reach. Only `.git` is skipped, and `target/`
//! and `node_modules/` are build output rather than authored files.
//!
//! **Nothing is shelled out to.** A gate that needs `ripgrep` on the machine reports nothing on a
//! machine without one, and `rg` exits 1 both for *"no matches"* and for *"no such tool"* — the
//! shape that makes an absent filter look like a clean sheet.

use std::path::{Path, PathBuf};

/// The `old` column of `main.rs`'s `RETIRED`, and whether the statement anchors each on word
/// boundaries.
///
/// Spelt in halves. This file is not exempt from the sweep it is about — deliberately, since an
/// exemption here would be a third one the statement does not carry — so writing the names out
/// would fail two of the unit's own tests for a reason about this file rather than about the tree.
const WANTED: &[(&str, bool)] = &[
    (concat!("aep-", "planning"), false),
    (concat!("a", "dp"), true),
    (concat!("ess-", "schema"), false),
];

/// Files and directories never walked: version control and build output, not authored files.
const NOT_AUTHORED: &[&str] = &[".git", "target", "node_modules"];

/// Exempt in full, by exact repository-relative path.
const EXEMPT_FILES: &[&str] = &[
    // A changelog records what the names were.
    "CHANGELOG.md",
    // Where the sweep's own `RETIRED` table lives; see the header.
    "crates/agentplugins-check/src/main.rs",
];

/// Exempt in full, by repository-relative prefix.
const EXEMPT_PREFIXES: &[&str] = &[
    // Dated change records, written on the day.
    "changes/",
    // The planning store, whose only writer is the `aep` CLI.
    ".engineering/",
];

/// The marker a row carries when its spelling is what the transcript beside it contains.
const RECORDED_SPELLING: &str = "recorded-under-this-name";

/// The trees whose directory layout is a plugin's identity, per the statement's last sentence.
const PATH_ROOTS: &[&str] = &["plugins/", "website/docs/plugins/"];

/// `rg`'s word character class, which is what `\b` is defined against.
fn word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// Whether `line` names `wanted` as something other than an AEP wire id.
///
/// A wire id is the name followed by `/` — `…/1` is the protocol and `…/default` the workflow, both
/// identifiers in `aep`'s formats that name the profile rather than the plugin. A `/` in front
/// outweighs it: `plugins/<name>/skills/wave` is a path into the plugin directory.
fn names(line: &str, wanted: &str, anchored: bool) -> bool {
    let bytes = line.as_bytes();
    let mut from = 0;
    while let Some(offset) = line[from..].find(wanted) {
        let start = from + offset;
        let end = start + wanted.len();
        from = end;
        let next = bytes.get(end).copied();
        if anchored {
            if start > 0 && word_byte(bytes[start - 1]) {
                continue;
            }
            if next.is_some_and(word_byte) {
                continue;
            }
        }
        let inside_a_path = start > 0 && bytes[start - 1] == b'/';
        if !inside_a_path && next == Some(b'/') {
            continue;
        }
        return true;
    }
    false
}

/// Whether a repository-relative path is one whose rows may quote the spelling a recording used.
fn quotable(relative: &str) -> bool {
    matches!(
        relative.split('/').collect::<Vec<_>>().as_slice(),
        ["evals", _, "expectations.trace.yaml"]
    )
}

/// Whether the walk reads this path at all: records and build output, per the statement.
fn exempt(relative: &str, name: &str) -> bool {
    if NOT_AUTHORED.contains(&name) {
        return true;
    }
    if EXEMPT_FILES.contains(&relative) {
        return true;
    }
    if EXEMPT_PREFIXES
        .iter()
        .any(|prefix| format!("{relative}/").starts_with(prefix))
    {
        return true;
    }
    matches!(
        relative.split('/').collect::<Vec<_>>().as_slice(),
        ["evals", _, "recorded", ..]
    )
}

/// Every violation of the statement: a line that names a retired plugin, and a path filed under one.
fn hits(root: &Path, directory: &Path, found: &mut Vec<String>) {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(directory)
        .expect("the repository is readable")
        .map(|entry| entry.expect("the repository is readable").path())
        .collect();
    entries.sort();
    for path in entries {
        let relative = path
            .strip_prefix(root)
            .expect("the walk starts at the root")
            .to_string_lossy()
            .replace('\\', "/");
        let name = path
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or_default()
            .to_owned();
        if exempt(&relative, &name) {
            continue;
        }

        // § *Acceptance*, last sentence: a path segment equal to a retired name under the trees
        // whose layout is the plugin's identity. A directory is named once, on its own last
        // segment, so one leftover tree is one line rather than one line per level.
        let is_dir = path.is_dir();
        if PATH_ROOTS.iter().any(|prefix| relative.starts_with(prefix)) {
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
            let filed = if is_dir {
                WANTED.iter().any(|(wanted, _)| bare(last) == *wanted)
            } else {
                (0..segments.len()).any(|index| WANTED.iter().any(|(w, _)| bare(index) == *w))
            };
            if filed {
                found.push(format!("{relative}: filed under a retired plugin name"));
            }
        }

        if is_dir {
            hits(root, &path, found);
            continue;
        }
        let Ok(bytes) = std::fs::read(&path) else {
            continue;
        };
        let quotable = quotable(&relative);
        for (number, line) in String::from_utf8_lossy(&bytes).lines().enumerate() {
            if quotable && line.contains(RECORDED_SPELLING) {
                continue;
            }
            if WANTED
                .iter()
                .any(|(wanted, anchored)| names(line, wanted, *anchored))
            {
                found.push(format!("{relative}:{}: {}", number + 1, line.trim()));
            }
        }
    }
}

/// § *Acceptance*: nothing outside the stated exemptions names a retired plugin, in its text or in
/// where it sits.
#[test]
fn the_acceptance_statements_sweep_returns_nothing() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the checker is under the repository root")
        .to_path_buf();
    let mut found = Vec::new();
    hits(&root, &root, &mut found);
    assert!(
        found.is_empty(),
        "the story's acceptance statement says this returns nothing; it returns {} line(s):\n{}",
        found.len(),
        found.join("\n")
    );
}

/// The walk itself, on a tree built to violate the statement four ways — because a walk that
/// returned nothing for a reason other than a clean tree would assert nothing above, and an
/// exemption list is only worth something if the thing it exempts from actually fires.
#[test]
fn the_walk_finds_each_violation_the_statement_names() {
    let sandbox = std::env::temp_dir().join(format!(
        "agentplugins-acceptance-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&sandbox);
    let retired = WANTED[0].0;
    let three_letters = WANTED[1].0;
    let write = |relative: &str, body: &str| {
        let target = sandbox.join(relative);
        std::fs::create_dir_all(target.parent().expect("every entry has a directory"))
            .expect("the sandbox is writable");
        std::fs::write(target, body).expect("the sandbox is writable");
    };

    // Four violations: prose, a path segment, a doc id, and a marker used outside an expectations
    // document.
    write("website/docs/install.md", &format!("install `{retired}`\n"));
    write(
        &format!("plugins/{three_letters}/skills/wave/SKILL.md"),
        "---\nname: wave\n---\n\nnothing here spells it\n",
    );
    write(&format!("website/docs/plugins/{retired}.md"), "# a page\n");
    write(
        "README.md",
        &format!("  install: {retired}  # {RECORDED_SPELLING}\n"),
    );

    // And five things the statement exempts, none of which may appear.
    write("CHANGELOG.md", &format!("renamed `{retired}`\n"));
    write("changes/a.yaml", &format!("plugin: {three_letters}\n"));
    write(".engineering/planning/story/a.md", &format!("{retired}\n"));
    write(
        "evals/a-case/recorded/run.events.jsonl",
        &format!("{{\"skill\":\"{three_letters}:wave\"}}\n"),
    );
    write(
        "evals/a-case/expectations.trace.yaml",
        &format!(
            "        agent: {retired}:decomposer  # {RECORDED_SPELLING}\n\
             workflow: {three_letters}/default\n"
        ),
    );

    let mut found = Vec::new();
    hits(&sandbox, &sandbox, &mut found);
    std::fs::remove_dir_all(&sandbox).expect("the sandbox is removable");
    found.sort();

    let mut expected = vec![
        format!("README.md:1: install: {retired}  # {RECORDED_SPELLING}"),
        format!("plugins/{three_letters}: filed under a retired plugin name"),
        format!("plugins/{three_letters}/skills/wave/SKILL.md: filed under a retired plugin name"),
        format!("website/docs/install.md:1: install `{retired}`"),
        format!("website/docs/plugins/{retired}.md: filed under a retired plugin name"),
    ];
    expected.sort();
    assert_eq!(found, expected);
}
