//! `agentplugins-check upstream` (network): everything this repository takes from another one, and
//! whether it has moved.
//!
//! The report the `following-upstream` skill starts from. It never fails on news: a newer release,
//! a moved workflow pin or a closed issue is work to do, and `tools` is the check that refuses.
//! It fails only when it cannot read something.
//!
//! Three kinds of dependency:
//!
//! - **Releases.** Each CLI a plugin drives, at the release this repository last verified or names:
//!   `verified.json` for `aep`, `ess` and `worktree`, and the `*_VERSION` pins of
//!   `.github/workflows/eval.yml` or a release named in a skill for the rest. A newer release gets
//!   its `CHANGELOG.md` sections from the pinned release (exclusive) to the newest (inclusive).
//! - **Workflow pins.** Every `uses: beyond10x/<repo>/…@<commit>` against that repository's `main`.
//! - **Cited issues.** Every `beyond10x/<repo>#<n>` in plugin or website text: a closed one is a
//!   workaround that may be removed.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::tools;

/// A release this repository depends on, and where its pin is written.
struct Tracked {
    name: &'static str,
    repository: &'static str,
    pin: Pin,
}

enum Pin {
    /// The CLI's entry in `verified.json`.
    Verified,
    Commit {
        file: &'static str,
        prefix: &'static str,
    },
    /// The first `<prefix><version>` in this file.
    Text {
        file: &'static str,
        prefix: &'static str,
    },
}

const TRACKED: &[Tracked] = &[
    Tracked {
        name: "aep",
        repository: "beyond10x/aep",
        pin: Pin::Verified,
    },
    Tracked {
        name: "ess",
        repository: "beyond10x/ess",
        pin: Pin::Verified,
    },
    Tracked {
        name: "worktree",
        repository: "beyond10x/worktree",
        pin: Pin::Verified,
    },
    Tracked {
        name: "eval AEP",
        repository: "beyond10x/aep",
        pin: Pin::Text {
            file: ".github/workflows/eval.yml",
            prefix: "AEP_VERSION: '",
        },
    },
    Tracked {
        name: "eval ESS",
        repository: "beyond10x/ess",
        pin: Pin::Text {
            file: ".github/workflows/eval.yml",
            prefix: "ESS_VERSION: '",
        },
    },
    Tracked {
        name: "eval Connectors",
        repository: "beyond10x/connectors",
        pin: Pin::Text {
            file: ".github/workflows/eval.yml",
            prefix: "CONNECTORS_VERSION: '",
        },
    },
    Tracked {
        name: "eval Worktree",
        repository: "beyond10x/worktree",
        pin: Pin::Text {
            file: ".github/workflows/eval.yml",
            prefix: "WORKTREE_VERSION: '",
        },
    },
    Tracked {
        name: "planning protocols",
        repository: "beyond10x/aep",
        pin: Pin::Commit {
            file: ".engineering/project.yaml",
            prefix: "protocols: git+https://github.com/beyond10x/aep#",
        },
    },
    Tracked {
        name: "website Docs System",
        repository: "beyond10x/docs-system",
        pin: Pin::Commit {
            file: "website/package.json",
            prefix: "git+https://github.com/beyond10x/docs-system.git#",
        },
    },
    Tracked {
        name: "metaharness",
        repository: "beyond10x/metaharness",
        pin: Pin::Text {
            file: ".github/workflows/eval.yml",
            prefix: "METAHARNESS_VERSION: '",
        },
    },
    Tracked {
        name: "connectors",
        repository: "beyond10x/connectors",
        pin: Pin::Text {
            file: "plugins/connectors/skills/integrating/SKILL.md",
            prefix: "[Connectors `",
        },
    },
];

/// The version string that starts right after `prefix` in `text`.
#[must_use]
pub fn pinned_in(text: &str, prefix: &str) -> Option<String> {
    let starts_identifier = prefix
        .chars()
        .next()
        .is_some_and(|c| c.is_alphanumeric() || c == '_');
    let start = text
        .match_indices(prefix)
        .find(|(index, _)| {
            !starts_identifier
                || text[..*index]
                    .chars()
                    .next_back()
                    .is_none_or(|c| !c.is_alphanumeric() && c != '_' && c != '-')
        })?
        .0
        + prefix.len();
    let version: String = text[start..]
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '.' || *c == '-')
        .collect();
    (!version.is_empty()).then_some(version)
}

/// The `CHANGELOG.md` sections for releases newer than `from` and no newer than `to`, in the order
/// the file has them. A section starts at a `## ` heading that names a version.
#[must_use]
pub fn sections_between(changelog: &str, from: &str, to: &str) -> Vec<String> {
    let (Some(from), Some(to)) = (tools::key(from), tools::key(to)) else {
        return Vec::new();
    };
    let mut sections = Vec::new();
    let mut current: Option<String> = None;
    for line in changelog.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            if let Some(section) = current.take() {
                sections.push(section);
            }
            let version = heading
                .trim_start_matches('[')
                .split([']', ' ', '—'])
                .next()
                .unwrap_or_default();
            if tools::key(version).is_some_and(|key| key > from && key <= to) {
                current = Some(format!("{line}\n"));
            }
            continue;
        }
        if let Some(section) = current.as_mut() {
            section.push_str(line);
            section.push('\n');
        }
    }
    sections.extend(current);
    sections
}

/// Every `beyond10x/<repo>#<number>` in `text`.
#[must_use]
pub fn cited_issues(text: &str) -> BTreeSet<(String, u64)> {
    let mut found = BTreeSet::new();
    let mut rest = text;
    while let Some(at) = rest.find("beyond10x/") {
        rest = &rest[at + "beyond10x/".len()..];
        let repository: String = rest
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '.')
            .collect();
        let after = &rest[repository.len()..];
        if let Some(digits) = after.strip_prefix('#') {
            let number: String = digits.chars().take_while(char::is_ascii_digit).collect();
            let boundary = digits[number.len()..]
                .chars()
                .next()
                .is_none_or(|c| !c.is_ascii_alphanumeric());
            if let (true, Ok(number)) = (boundary, number.parse()) {
                found.insert((repository, number));
            }
        }
    }
    found
}

/// Every `uses: beyond10x/<repo>/…@<commit>` in `text`, as (repo, commit).
#[must_use]
pub fn workflow_pins(text: &str) -> BTreeSet<(String, String)> {
    text.lines()
        .filter_map(|line| {
            line.trim()
                .trim_start_matches("- ")
                .strip_prefix("uses: beyond10x/")
        })
        .filter_map(|rest| {
            let repository = rest.split('/').next()?.to_owned();
            let commit = rest
                .rsplit_once('@')?
                .1
                .split_whitespace()
                .next()?
                .to_owned();
            (commit.len() == 40).then_some((repository, commit))
        })
        .collect()
}

fn files(root: &Path, extension: &str) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some(extension) {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}

fn fetch(url: &str) -> Result<String, String> {
    tools::run("curl", &["-fsSL", "--retry", "2", url])
}

fn short(commit: &str) -> &str {
    commit.get(..7).unwrap_or(commit)
}

pub(crate) fn release_commit(repository: &str, tag: &str) -> Result<String, String> {
    let output = tools::run(
        "git",
        &[
            "ls-remote",
            &format!("https://github.com/{repository}"),
            &format!("refs/tags/{tag}"),
            &format!("refs/tags/{tag}^{{}}"),
        ],
    )?;
    output
        .lines()
        .last()
        .and_then(|line| line.split_whitespace().next())
        .map(str::to_owned)
        .ok_or_else(|| format!("{repository}: release {tag} has no tag commit"))
}

fn report_releases(root: &Path) -> Result<usize, String> {
    let verified = tools::verified(root)?;
    let mut moved = 0;
    println!("# Upstream report\n\n## Releases\n");
    for tracked in TRACKED {
        let pinned = match &tracked.pin {
            Pin::Verified => verified.get(tracked.name).cloned(),
            Pin::Text { file, prefix } | Pin::Commit { file, prefix } => {
                std::fs::read_to_string(root.join(file))
                    .ok()
                    .and_then(|text| pinned_in(&text, prefix))
            }
        }
        .ok_or_else(|| format!("{}: no pinned release found", tracked.name))?;
        let newest = tools::latest(tracked.repository)?;
        if matches!(tracked.pin, Pin::Commit { .. }) {
            let commit = release_commit(tracked.repository, &newest)?;
            let behind = commit != pinned;
            moved += usize::from(behind);
            println!(
                "- `{}` pinned {}, newest {newest} is {}{}",
                tracked.name,
                short(&pinned),
                short(&commit),
                if behind { " — **moved**" } else { "" }
            );
            continue;
        }
        let behind = tools::key(&newest) > tools::key(&pinned);
        println!(
            "- `{}` pinned {pinned}, newest {newest}{}",
            tracked.name,
            if behind { " — **moved**" } else { "" }
        );
        if behind {
            moved += 1;
            let url = format!(
                "https://raw.githubusercontent.com/{}/{newest}/CHANGELOG.md",
                tracked.repository
            );
            match fetch(&url) {
                Ok(changelog) => {
                    for section in sections_between(&changelog, &pinned, &newest) {
                        println!("\n```markdown\n{section}```\n");
                    }
                }
                Err(error) => println!("  (no changelog at {url}: {error})"),
            }
        }
    }

    Ok(moved)
}

/// Print the report; fail only when something cannot be read.
pub fn report(root: &Path) -> Result<(), String> {
    let mut moved = report_releases(root)?;
    println!("\n## Workflow pins\n");
    let mut pins = BTreeSet::new();
    for workflow in files(&root.join(".github/workflows"), "yml") {
        let text = std::fs::read_to_string(&workflow).map_err(|error| error.to_string())?;
        let generated = text.contains("Generated")
            || text.contains("generated")
                && workflow
                    .file_name()
                    .is_some_and(|name| name.to_string_lossy().starts_with("b10x-docs-"));
        for (repository, commit) in workflow_pins(&text) {
            pins.insert((repository, commit, generated));
        }
    }
    for (repository, commit, generated) in pins {
        let head = tools::run(
            "git",
            &[
                "ls-remote",
                &format!("https://github.com/beyond10x/{repository}"),
                "refs/heads/main",
            ],
        )?;
        let main = head.split_whitespace().next().unwrap_or_default();
        let owner = if generated {
            " (generated; Atlas reconciliation owns updates)"
        } else {
            " (repository maintained)"
        };
        if main == commit {
            println!("- `{repository}` pinned {}, main{owner}", short(&commit));
        } else {
            moved += 1;
            println!(
                "- `{repository}` pinned {}, main is {} — **moved**{owner}",
                short(&commit),
                short(main)
            );
        }
    }

    println!("\n## Cited issues\n");
    let mut issues = BTreeSet::new();
    for dir in ["plugins", "website/docs"] {
        for file in files(&root.join(dir), "md") {
            let text = std::fs::read_to_string(&file).map_err(|error| error.to_string())?;
            issues.extend(cited_issues(&text));
        }
    }
    for (repository, number) in issues {
        let json = fetch(&format!(
            "https://api.github.com/repos/beyond10x/{repository}/issues/{number}"
        ))?;
        let value: serde_json::Value =
            serde_json::from_str(&json).map_err(|error| error.to_string())?;
        let state = value.get("state").and_then(|s| s.as_str()).unwrap_or("?");
        if state == "closed" {
            moved += 1;
        }
        println!(
            "- beyond10x/{repository}#{number} {state}{}",
            if state == "closed" {
                " — **read the text that cites it: a workaround may go**"
            } else {
                ""
            }
        );
    }
    println!("\n{moved} item(s) moved.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_pin_is_read_after_its_prefix() {
        let workflow = "METAHARNESS_VERSION: '0.9.1'\nESS_VERSION: '0.53.0'\n";
        assert_eq!(
            pinned_in(workflow, "ESS_VERSION: '"),
            Some("0.53.0".to_owned())
        );
        assert_eq!(
            pinned_in("METAHARNESS_VERSION: '0.9.1'", "ESS_VERSION: '"),
            None
        );
        assert_eq!(
            pinned_in(
                "protocols: git+https://github.com/beyond10x/aep#abcdef",
                "protocols: git+https://github.com/beyond10x/aep#"
            ),
            Some("abcdef".to_owned())
        );
        assert_eq!(
            pinned_in(
                "\"git+https://github.com/beyond10x/docs-system.git#fedcba\"",
                "git+https://github.com/beyond10x/docs-system.git#"
            ),
            Some("fedcba".to_owned())
        );
        assert_eq!(
            pinned_in(
                "x\n      METAHARNESS_VERSION: '0.8.0'\n",
                "METAHARNESS_VERSION: '"
            ),
            Some("0.8.0".to_owned())
        );
        assert_eq!(
            pinned_in("[Connectors `v0.7.2` release](…)", "[Connectors `"),
            Some("v0.7.2".to_owned())
        );
    }

    #[test]
    fn only_the_sections_after_the_pin_are_returned() {
        let changelog = "# Changelog\n\n## [0.64.0] — 2026-09-28\n\nnew\n\n## [0.63.1] — 2026-09-28\n\nfix\n\n## [0.63.0]\n\nold\n";
        let sections = sections_between(changelog, "0.63.0", "0.64.0");
        assert_eq!(sections.len(), 2);
        assert!(sections[0].starts_with("## [0.64.0]") && sections[0].contains("new"));
        assert!(sections[1].starts_with("## [0.63.1]") && !sections[1].contains("old"));
        let plain = "## 0.39.0 — x\n\na\n\n## 0.38.0 — y\n\nb\n";
        assert_eq!(sections_between(plain, "0.38.0", "0.39.0").len(), 1);
    }

    #[test]
    fn issues_and_workflow_pins_are_found() {
        assert_eq!(
            workflow_pins(
                "- uses: beyond10x/gates/check@339b4b8462f19b4c9d3716e6a44ed2a3691eb9d8 # pinned"
            )
            .len(),
            1
        );
        let issues = cited_issues("see beyond10x/ess#186 and (beyond10x/aep#8), not beyond10x/ess, nor git+https://github.com/beyond10x/aep#8b4342a41fdd9143");
        assert_eq!(
            issues.into_iter().collect::<Vec<_>>(),
            vec![("aep".to_owned(), 8), ("ess".to_owned(), 186)]
        );
        assert!(
            cited_issues("git+https://github.com/beyond10x/aep#8b4342a41fdd9143").is_empty(),
            "a commit in a git+ locator is not an issue"
        );
        let pins = workflow_pins(
            "    uses: beyond10x/docs-system/.github/actions/check@339b4b8462f19b4c9d3716e6a44ed2a3691eb9d8\n    uses: actions/checkout@v4\n",
        );
        assert_eq!(
            pins.into_iter().collect::<Vec<_>>(),
            vec![(
                "docs-system".to_owned(),
                "339b4b8462f19b4c9d3716e6a44ed2a3691eb9d8".to_owned()
            )]
        );
    }
}
