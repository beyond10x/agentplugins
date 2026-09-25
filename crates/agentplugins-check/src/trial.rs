//! Whether a headless trial run tested what it claims to: the plugins in its sandbox, at the
//! version under test, and nothing from the machine it ran on.
//!
//! A trial run as a sub-agent of a working session inherits that session's agents, so its critics
//! were the host's and not the plugin's under test (trial 3, 2026-09-25). The run's own `init`
//! event lists what it loaded; this reads it and refuses anything that did not come from the
//! sandbox.
//!
//! A plugin the sandbox never installed is refused too, wherever it loaded from: Claude Code also
//! reads `.claude/settings*.json` in the directories above the working directory, and one left in
//! `~/.cache` by an earlier trial enabled `aep@b10x` in every sandbox below it (trial 3).

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde_json::Value;

/// The marketplace every plugin under test comes from.
const MARKETPLACE: &str = "b10x";

/// What the sandbox itself provides.
struct Sandbox<'a> {
    /// The sandbox directory; its `home/` is the run's `HOME`.
    root: &'a Path,
    /// A directory marketplace plugins may load from in place (the checkout under test).
    marketplace: Option<&'a Path>,
    /// `name@marketplace` of every plugin the sandbox's own registry held when the run started,
    /// with the versions it held.
    installed: BTreeMap<String, BTreeSet<String>>,
    /// The sandbox was seeded with older plugins on purpose (an upgrade trial): a plugin must be at
    /// the version the sandbox installed, not at the version under test.
    seeded: bool,
}

/// Check every `init` event of a stream-json run; returns a one-line summary.
pub fn isolation(
    run: &Path,
    sandbox: &Path,
    marketplace: Option<&Path>,
    version: &str,
    seeded: bool,
) -> Result<String, String> {
    let text = std::fs::read_to_string(run)
        .map_err(|error| format!("reading {}: {error}", run.display()))?;
    let root = std::fs::canonicalize(sandbox)
        .map_err(|error| format!("sandbox {}: {error}", sandbox.display()))?;
    let marketplace = marketplace
        .map(std::fs::canonicalize)
        .transpose()
        .map_err(|error| format!("marketplace: {error}"))?;
    // `task trial:run` copies the registry before the run: an upgrade changes it during the run.
    let registry = [
        root.join("registry-at-start.json"),
        root.join("home/.claude/plugins/installed_plugins.json"),
    ]
    .into_iter()
    .find(|path| path.is_file())
    .ok_or_else(|| format!("{} has no plugin registry", root.display()))?;
    let registry: Value = std::fs::read_to_string(&registry)
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .ok_or_else(|| format!("reading {}", registry.display()))?;
    let installed = registry["plugins"]
        .as_object()
        .map(|plugins| {
            plugins
                .iter()
                .map(|(id, entries)| {
                    let versions = entries
                        .as_array()
                        .into_iter()
                        .flatten()
                        .filter_map(|entry| entry["version"].as_str().map(str::to_owned))
                        .collect();
                    (id.clone(), versions)
                })
                .collect()
        })
        .unwrap_or_default();
    check(
        &text,
        &Sandbox {
            root: &root,
            marketplace: marketplace.as_deref(),
            installed,
            seeded,
        },
        version,
    )
}

#[allow(clippy::too_many_lines)]
fn check(text: &str, sandbox: &Sandbox<'_>, version: &str) -> Result<String, String> {
    let root = sandbox.root;
    let mut problems = Vec::new();
    let mut inits = 0;
    let mut tested = BTreeSet::new();
    for (number, line) in text.lines().enumerate() {
        let Ok(event) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        if event["type"] != "system" || event["subtype"] != "init" {
            continue;
        }
        inits += 1;
        let at = format!("line {}", number + 1);
        if let Some(cwd) = event["cwd"].as_str() {
            if !Path::new(cwd).starts_with(root) {
                problems.push(format!("{at}: ran in {cwd}, outside the sandbox"));
            }
        }
        for server in event["mcp_servers"].as_array().into_iter().flatten() {
            let name = server["name"].as_str().unwrap_or("?");
            problems.push(format!(
                "{at}: MCP server `{name}` was connected; trials run with `--strict-mcp-config` and none"
            ));
        }
        let mut loaded = BTreeSet::new();
        for plugin in event["plugins"].as_array().into_iter().flatten() {
            let name = plugin["name"].as_str().unwrap_or_default();
            let path = plugin["path"].as_str().unwrap_or_default();
            let source = plugin["source"].as_str().unwrap_or_default();
            loaded.insert(name.to_owned());
            if path == "builtin" {
                continue;
            }
            let path = PathBuf::from(path);
            let in_place = sandbox
                .marketplace
                .is_some_and(|marketplace| path.starts_with(marketplace));
            if !path.starts_with(root.join("home")) && !in_place {
                problems.push(format!(
                    "{at}: plugin `{source}` loaded from {}, outside the sandbox home and the marketplace under test",
                    path.display()
                ));
            }
            if !sandbox.installed.contains_key(source) {
                problems.push(format!(
                    "{at}: plugin `{source}` is not in the sandbox's installed_plugins.json; a settings file outside the sandbox enabled it"
                ));
            }
            if source.ends_with(&format!("@{MARKETPLACE}")) {
                tested.insert(name.to_owned());
                let loaded_version = plugin["version"].as_str().unwrap_or_default();
                let expected = if sandbox.seeded {
                    sandbox
                        .installed
                        .get(source)
                        .is_some_and(|versions| versions.contains(loaded_version))
                } else {
                    loaded_version == version
                };
                if !expected {
                    problems.push(format!(
                        "{at}: plugin `{source}` is {loaded_version}, not {}",
                        if sandbox.seeded {
                            "a version the sandbox was seeded with".to_owned()
                        } else {
                            format!("the version under test {version}")
                        }
                    ));
                }
            } else {
                problems.push(format!(
                    "{at}: plugin `{source}` is not from the `{MARKETPLACE}` marketplace"
                ));
            }
        }
        for list in ["agents", "skills"] {
            for id in event[list].as_array().into_iter().flatten() {
                let id = id.as_str().unwrap_or_default();
                if let Some((owner, _)) = id.split_once(':') {
                    if !loaded.contains(owner) {
                        problems.push(format!(
                            "{at}: {} `{id}` belongs to no plugin the sandbox loaded",
                            list.trim_end_matches('s')
                        ));
                    }
                }
            }
        }
    }
    if inits == 0 {
        problems.push(
            "no `init` event: not a stream-json run (`--output-format stream-json --verbose`)"
                .to_owned(),
        );
    } else if tested.is_empty() {
        problems.push(format!("no `@{MARKETPLACE}` plugin was loaded"));
    }
    if problems.is_empty() {
        let tested: Vec<String> = tested.into_iter().collect();
        Ok(format!(
            "isolated: {inits} session(s), plugins {} at {version}, all from {}",
            tested.join(", "),
            root.display()
        ))
    } else {
        Err(format!(
            "the run is not an isolated trial of {version}:\n  {}",
            problems.join("\n  ")
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init(plugins: &str, agents: &str) -> String {
        format!(
            r#"{{"type":"system","subtype":"init","cwd":"/opt/trial/work","plugins":[{plugins}],"agents":[{agents}],"skills":["ess:specifying","verify"]}}"#
        )
    }

    fn sandbox() -> Sandbox<'static> {
        Sandbox {
            root: Path::new("/opt/trial"),
            marketplace: Some(Path::new("/opt/checkout")),
            installed: [("ess@b10x".to_owned(), ["0.14.3".to_owned()].into())].into(),
            seeded: false,
        }
    }

    #[test]
    fn a_seeded_upgrade_trial_starts_from_the_seeded_version() {
        let old = r#"{"name":"ess","path":"/opt/trial/home/.claude/plugins/cache/b10x/ess/0.12.0","source":"ess@b10x","version":"0.12.0"}"#;
        let mut seeded = sandbox();
        seeded.installed = [("ess@b10x".to_owned(), ["0.12.0".to_owned()].into())].into();
        seeded.seeded = true;
        check(&init(old, ""), &seeded, "0.14.3").unwrap();
        seeded.seeded = false;
        let error = check(&init(old, ""), &seeded, "0.14.3").unwrap_err();
        assert!(error.contains("not the version under test"), "{error}");
    }

    #[test]
    fn a_plugin_the_sandbox_never_installed_fails_the_run() {
        let leaked = r#"{"name":"aep","path":"/opt/checkout/plugins/aep","source":"aep@b10x","version":"0.14.3"}"#;
        let error = check(&init(leaked, ""), &sandbox(), "0.14.3").unwrap_err();
        assert!(
            error.contains("not in the sandbox's installed_plugins.json"),
            "{error}"
        );
        assert!(!error.contains("outside the sandbox home"), "{error}");
    }

    const SANDBOXED: &str = r#"{"name":"ess","path":"/opt/trial/home/.claude/plugins/cache/b10x/ess/0.14.3","source":"ess@b10x","version":"0.14.3"},{"name":"telemetry","path":"builtin","source":"telemetry@builtin"}"#;

    #[test]
    fn a_sandboxed_run_passes() {
        let run = init(SANDBOXED, r#""claude","ess:author""#);
        let summary = check(&run, &sandbox(), "0.14.3").unwrap();
        assert!(summary.contains("plugins ess at 0.14.3"), "{summary}");
    }

    #[test]
    fn a_host_agent_fails_the_run() {
        let run = init(SANDBOXED, r#""host-plugin:plan-critic-design""#);
        let error = check(&run, &sandbox(), "0.14.3").unwrap_err();
        assert!(
            error.contains("agent `host-plugin:plan-critic-design`"),
            "{error}"
        );
    }

    #[test]
    fn another_version_or_a_host_path_fails_the_run() {
        let host = r#"{"name":"ess","path":"/opt/host/.claude/plugins/cache/b10x/ess/0.14.2","source":"ess@b10x","version":"0.14.2"}"#;
        let error = check(&init(host, ""), &sandbox(), "0.14.3").unwrap_err();
        assert!(error.contains("outside the sandbox home"), "{error}");
        assert!(error.contains("not the version under test"), "{error}");
    }

    #[test]
    fn an_account_mcp_server_fails_the_run() {
        let run = init(SANDBOXED, "").replace(
            "\"agents\"",
            "\"mcp_servers\":[{\"name\":\"claude.ai Docs\",\"status\":\"connected\"}],\"agents\"",
        );
        let error = check(&run, &sandbox(), "0.14.3").unwrap_err();
        assert!(error.contains("MCP server `claude.ai Docs`"), "{error}");
    }

    #[test]
    fn a_stream_without_init_fails() {
        let error = check("{\"type\":\"result\"}", &sandbox(), "0.14.3").unwrap_err();
        assert!(error.contains("no `init` event"), "{error}");
    }
}
