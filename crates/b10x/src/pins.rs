//! Per-repository CLI pins. A repository may commit `b10x.toml`:
//!
//! ```toml
//! [pins]
//! ess = "0.32.0"   # exactly this release
//! aep = "0.59"     # the newest 0.59.x
//! ```
//!
//! `b10x` finds it from the current directory upward, stopping below `$HOME` or at the filesystem
//! root, and resolves a pinned CLI to its pinned release instead of the newest.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::version;

/// The pin file's name.
pub const FILE: &str = "b10x.toml";

/// What a pin asks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spec {
    /// Exactly `x.y.z`.
    Exact((u64, u64, u64)),
    /// The newest `x.y.*`.
    Line(u64, u64),
}

impl Spec {
    /// Parse `0.32.0` or `0.59` (a leading `v` is allowed).
    pub fn parse(text: &str) -> Result<Self, String> {
        let bare = text.trim().trim_start_matches('v');
        if let Some(key) = version::key(bare) {
            return Ok(Spec::Exact(key));
        }
        let mut parts = bare.split('.');
        match (
            parts.next().and_then(|part| part.parse().ok()),
            parts.next().and_then(|part| part.parse().ok()),
            parts.next(),
        ) {
            (Some(major), Some(minor), None) => Ok(Spec::Line(major, minor)),
            _ => Err(format!(
                "`{text}` is not a version: use `x.y.z` for one release or `x.y` for the newest `x.y.*`"
            )),
        }
    }

    /// Whether a version (or tag) satisfies this pin.
    #[must_use]
    pub fn matches(&self, found: &str) -> bool {
        match (self, version::key(found)) {
            (Spec::Exact(want), Some(have)) => *want == have,
            (Spec::Line(major, minor), Some((a, b, _))) => (*major, *minor) == (a, b),
            _ => false,
        }
    }

    /// The newest tag that satisfies this pin.
    #[must_use]
    pub fn select<'a>(&self, tags: impl IntoIterator<Item = &'a str>) -> Option<&'a str> {
        tags.into_iter()
            .filter(|tag| self.matches(tag))
            .max_by_key(|tag| version::key(tag))
    }

    /// Whether `version` is newer than anything this pin allows.
    #[must_use]
    pub fn older_than(&self, version: &str) -> bool {
        match (self, version::key(version)) {
            (Spec::Exact(want), Some(have)) => have > *want,
            (Spec::Line(major, minor), Some((a, b, _))) => (a, b) > (*major, *minor),
            _ => false,
        }
    }
}

/// A pin resolved against the releases, as a plan carries it.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Pinned {
    /// What the file asks for: `0.32.0` or `0.59`.
    pub spec: String,
    /// The release it resolves to; `None` when none could be read.
    pub tag: Option<String>,
    /// The pin file.
    pub file: String,
}

/// A pin file and its `[pins]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinFile {
    /// Where it is.
    pub path: PathBuf,
    /// CLI name → spec, as written.
    pub pins: BTreeMap<String, String>,
}

/// The nearest `b10x.toml` from `start` upward. `$HOME` itself and everything above it are not
/// searched, so a stray file in the home directory pins nothing.
#[must_use]
pub fn find(start: &Path, home: &Path) -> Option<PathBuf> {
    let mut directory = Some(start);
    while let Some(current) = directory {
        if current == home {
            return None;
        }
        let candidate = current.join(FILE);
        if candidate.is_file() {
            return Some(candidate);
        }
        directory = current.parent();
    }
    None
}

fn table(path: &Path) -> Result<toml::Table, String> {
    let text =
        std::fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
    text.parse::<toml::Table>()
        .map_err(|error| format!("{}: {error}", path.display()))
}

/// Read one pin file; every pin must parse.
pub fn load(path: &Path) -> Result<PinFile, String> {
    let table = table(path)?;
    let mut pins = BTreeMap::new();
    if let Some(value) = table.get("pins") {
        let entries = value
            .as_table()
            .ok_or_else(|| format!("{}: `pins` must be a table", path.display()))?;
        for (name, spec) in entries {
            let spec = spec
                .as_str()
                .ok_or_else(|| format!("{}: pin `{name}` must be a string", path.display()))?;
            Spec::parse(spec).map_err(|error| format!("{}: {name}: {error}", path.display()))?;
            pins.insert(name.clone(), spec.to_owned());
        }
    }
    Ok(PinFile {
        path: path.to_path_buf(),
        pins,
    })
}

/// The pins that apply in `start`, if a pin file is found.
pub fn read(start: &Path, home: &Path) -> Result<Option<PinFile>, String> {
    find(start, home).map(|path| load(&path)).transpose()
}

/// Set one pin in `path`, creating the file when needed; other content is kept.
pub fn set(path: &Path, name: &str, spec: &str) -> Result<(), String> {
    let mut table = if path.is_file() {
        table(path)?
    } else {
        toml::Table::new()
    };
    let pins = table
        .entry("pins")
        .or_insert_with(|| toml::Value::Table(toml::Table::new()))
        .as_table_mut()
        .ok_or_else(|| format!("{}: `pins` must be a table", path.display()))?;
    pins.insert(name.to_owned(), toml::Value::String(spec.to_owned()));
    write(path, &table)
}

/// Remove one pin from `path`; the file goes when nothing is left in it. `false` when not pinned.
pub fn remove(path: &Path, name: &str) -> Result<bool, String> {
    let mut table = table(path)?;
    let Some(pins) = table.get_mut("pins").and_then(toml::Value::as_table_mut) else {
        return Ok(false);
    };
    if pins.remove(name).is_none() {
        return Ok(false);
    }
    if pins.is_empty() {
        table.remove("pins");
    }
    if table.is_empty() {
        std::fs::remove_file(path).map_err(|error| format!("{}: {error}", path.display()))?;
        return Ok(true);
    }
    write(path, &table)?;
    Ok(true)
}

fn write(path: &Path, table: &toml::Table) -> Result<(), String> {
    let text = toml::to_string(table).map_err(|error| error.to_string())?;
    std::fs::write(path, text).map_err(|error| format!("{}: {error}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("b10x-pins-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        root
    }

    #[test]
    fn specs_parse_exact_and_minor_lines() {
        assert_eq!(Spec::parse("0.32.0"), Ok(Spec::Exact((0, 32, 0))));
        assert_eq!(Spec::parse("v0.59"), Ok(Spec::Line(0, 59)));
        assert!(Spec::parse("0").is_err());
        assert!(Spec::parse("latest").is_err());
        assert!(Spec::parse("0.1.2.3").is_err());
    }

    #[test]
    fn exact_and_minor_line_pins_select_their_release() {
        let tags = ["0.58.4", "0.59.0", "0.59.3", "v0.59.1", "0.60.0"];
        assert_eq!(
            Spec::parse("0.59").unwrap().select(tags.iter().copied()),
            Some("0.59.3")
        );
        assert_eq!(
            Spec::parse("0.59.1").unwrap().select(tags.iter().copied()),
            Some("v0.59.1")
        );
        assert_eq!(
            Spec::parse("0.61").unwrap().select(tags.iter().copied()),
            None
        );
        assert!(Spec::parse("0.59").unwrap().older_than("0.60.0"));
        assert!(!Spec::parse("0.59").unwrap().older_than("0.59.9"));
        assert!(Spec::parse("0.32.0").unwrap().older_than("0.32.1"));
    }

    #[test]
    fn the_nearest_pin_file_is_found_below_home_only() {
        let home = scratch("lookup");
        let repo = home.join("work/repo");
        let deep = repo.join("crates/a/src");
        std::fs::create_dir_all(&deep).unwrap();
        assert_eq!(find(&deep, &home), None);
        std::fs::write(home.join(FILE), "[pins]\ness = \"0.1.0\"\n").unwrap();
        assert_eq!(find(&deep, &home), None, "a file in $HOME pins nothing");
        std::fs::write(
            repo.join(FILE),
            "[pins]\ness = \"0.32.0\"\naep = \"0.59\"\n",
        )
        .unwrap();
        let found = read(&deep, &home).unwrap().unwrap();
        assert_eq!(found.path, repo.join(FILE));
        assert_eq!(found.pins.get("aep").map(String::as_str), Some("0.59"));
        std::fs::write(repo.join(FILE), "[pins]\ness = \"soon\"\n").unwrap();
        assert!(read(&deep, &home).is_err());
        std::fs::remove_dir_all(&home).unwrap();
    }

    #[test]
    fn pins_are_set_and_removed_keeping_other_content() {
        let root = scratch("edit");
        let path = root.join(FILE);
        set(&path, "ess", "0.32.0").unwrap();
        set(&path, "aep", "0.59").unwrap();
        assert_eq!(load(&path).unwrap().pins.len(), 2);
        assert!(remove(&path, "ess").unwrap());
        assert!(!remove(&path, "ess").unwrap());
        assert!(remove(&path, "aep").unwrap());
        assert!(!path.exists(), "an empty pin file is removed");
        std::fs::write(&path, "[other]\nkeep = true\n").unwrap();
        set(&path, "ess", "0.32").unwrap();
        remove(&path, "ess").unwrap();
        assert!(std::fs::read_to_string(&path)
            .unwrap()
            .contains("keep = true"));
        std::fs::remove_dir_all(&root).unwrap();
    }
}
