//! CLI pins. A repository may commit `b10x.toml`:
//!
//! ```toml
//! [pins]
//! ess = "0.32.0"   # exactly this release
//! aep = "0.59"     # the newest 0.59.x
//! worktree = { version = "0.18.2", reason = "0.19.0 refused existing trees" }
//! ```
//!
//! `b10x` finds it from the current directory upward, stopping below `$HOME` or at the filesystem
//! root, and resolves a pinned CLI to its pinned release instead of the newest.
//!
//! A machine may hold CLIs back in every directory with a local pin file of the same form:
//! `$B10X_PINS`, else `$XDG_CONFIG_HOME/b10x/pins.toml`, else `~/.config/b10x/pins.toml`. For the
//! same CLI a local pin wins over the repository's.

use std::collections::BTreeMap;
use std::ffi::OsString;
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
    /// Why the CLI is held, as the pin file says.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Whether the pin is this machine's local pin rather than the repository's `b10x.toml`.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub local: bool,
}

impl Pinned {
    /// How a finding names the pin: `pinned to 0.59 by …/b10x.toml`, or
    /// `held at 0.71.3 by the local pin in …/pins.toml (reason)`.
    #[must_use]
    pub fn phrase(&self) -> String {
        if self.local {
            match &self.reason {
                Some(reason) => format!(
                    "held at {} by the local pin in {} ({})",
                    self.spec,
                    self.file,
                    one_line(reason)
                ),
                None => format!("held at {} by the local pin in {}", self.spec, self.file),
            }
        } else {
            format!("pinned to {} by {}", self.spec, self.file)
        }
    }
}

/// `text` with every whitespace run, newlines included, as one space: a pin's reason is printed
/// inside one line.
#[must_use]
pub fn one_line(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// One `[pins]` entry, as written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// `0.32.0` or `0.59`.
    pub spec: String,
    /// Why the CLI is held, when the table form gives one.
    pub reason: Option<String>,
}

impl Entry {
    /// An entry without a reason.
    #[must_use]
    pub fn new(spec: &str) -> Self {
        Entry {
            spec: spec.to_owned(),
            reason: None,
        }
    }
}

/// A pin file and its `[pins]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinFile {
    /// Where it is.
    pub path: PathBuf,
    /// CLI name → its entry.
    pub pins: BTreeMap<String, Entry>,
    /// Whether this is the machine's local pin file rather than a repository's `b10x.toml`.
    pub local: bool,
}

/// One effective pin: its entry and the file it comes from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pin {
    /// `0.32.0` or `0.59`, as written.
    pub spec: String,
    /// Why the CLI is held.
    pub reason: Option<String>,
    /// The pin file.
    pub file: PathBuf,
    /// Whether it is the local pin file.
    pub local: bool,
}

impl Pin {
    /// The pin as a plan carries it, resolved to `tag`.
    #[must_use]
    pub fn pinned(&self, tag: Option<String>) -> Pinned {
        Pinned {
            spec: self.spec.clone(),
            tag,
            file: self.file.to_string_lossy().into_owned(),
            reason: self.reason.clone(),
            local: self.local,
        }
    }
}

/// CLI name → the pin that applies to it.
pub type Pins = BTreeMap<String, Pin>;

/// The effective pins: the repository's, with every local pin replacing the repository's pin for
/// the same CLI.
#[must_use]
pub fn merge(repository: Option<&PinFile>, local: Option<&PinFile>) -> Pins {
    let mut pins = Pins::new();
    for file in repository.into_iter().chain(local) {
        for (name, entry) in &file.pins {
            pins.insert(
                name.clone(),
                Pin {
                    spec: entry.spec.clone(),
                    reason: entry.reason.clone(),
                    file: file.path.clone(),
                    local: file.local,
                },
            );
        }
    }
    pins
}

/// The local pin file's name under the configuration directory.
const LOCAL: &str = "b10x/pins.toml";

/// Where the local pin file is: `b10x_pins` (`$B10X_PINS`) when set and non-empty, else under
/// `xdg` (`$XDG_CONFIG_HOME`) when set and non-empty, else under `home/.config`.
#[must_use]
pub fn local_path_from(home: &Path, b10x_pins: Option<OsString>, xdg: Option<OsString>) -> PathBuf {
    if let Some(named) = b10x_pins.filter(|value| !value.is_empty()) {
        return PathBuf::from(named);
    }
    match xdg.filter(|value| !value.is_empty()) {
        Some(config) => PathBuf::from(config).join(LOCAL),
        None => home.join(".config").join(LOCAL),
    }
}

/// The local pin file, if there is one. Only a file that is cleanly absent at the default
/// locations pins nothing; a dangling link or an unreadable path there is an error, as is a
/// relative `b10x_pins` or a missing file it names.
pub fn read_local_from(
    home: &Path,
    b10x_pins: Option<OsString>,
    xdg: Option<OsString>,
) -> Result<Option<PinFile>, String> {
    let explicit = b10x_pins.as_ref().is_some_and(|value| !value.is_empty());
    if let Some(named) = b10x_pins.as_ref().filter(|value| !value.is_empty()) {
        if !Path::new(named).is_absolute() {
            return Err(format!(
                "$B10X_PINS must be an absolute path: {}",
                named.to_string_lossy()
            ));
        }
    }
    let path = local_path_from(home, b10x_pins, xdg);
    match std::fs::symlink_metadata(&path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            if explicit {
                Err(format!(
                    "{}: the local pin file $B10X_PINS names does not exist",
                    path.display()
                ))
            } else {
                Ok(None)
            }
        }
        Err(error) => Err(format!("{}: {error}", path.display())),
        Ok(_) => load_as(&path, true).map(Some),
    }
}

/// The note `b10x pin` and `b10x unpin` print when the local pin file holds `name`: that pin, not
/// the repository's, decides this machine.
#[must_use]
pub fn local_hold(local: Option<&PinFile>, name: &str) -> Option<String> {
    let file = local?;
    let entry = file.pins.get(name)?;
    Some(format!(
        "a local pin in {} holds {name} at {}; it wins over {FILE} on this machine.",
        file.path.display(),
        entry.spec
    ))
}

/// The local pin file, from the environment.
pub fn read_local(home: &Path) -> Result<Option<PinFile>, String> {
    read_local_from(
        home,
        std::env::var_os("B10X_PINS"),
        std::env::var_os("XDG_CONFIG_HOME"),
    )
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
        .map_err(|error| format!("{}: {}", path.display(), syntax(&text, &error)))
}

/// A TOML error as one line: its message and, when it has one, its line and column. The error's
/// own `Display` adds a multi-line source snippet, which a session prints as several lines.
fn syntax(text: &str, error: &toml::de::Error) -> String {
    let message = one_line(error.message());
    match error.span() {
        Some(span) => {
            let before = text.get(..span.start).unwrap_or(text);
            let line = before.matches('\n').count() + 1;
            let column = before
                .rfind('\n')
                .map_or(before, |newline| &before[newline + 1..])
                .chars()
                .count()
                + 1;
            format!("{message} (line {line}, column {column})")
        }
        None => message,
    }
}

/// Read one repository pin file; every pin must parse.
pub fn load(path: &Path) -> Result<PinFile, String> {
    load_as(path, false)
}

/// One `[pins]` value: `"0.59"` or `{ version = "0.59", reason = "…" }`.
fn entry(name: &str, value: &toml::Value) -> Result<Entry, String> {
    let entry = match value {
        toml::Value::String(spec) => Entry::new(spec),
        toml::Value::Table(fields) => {
            if let Some(unknown) = fields
                .keys()
                .find(|key| !matches!(key.as_str(), "version" | "reason"))
            {
                return Err(format!(
                    "pin `{name}` has an unknown key `{unknown}`; it takes `version` and `reason`"
                ));
            }
            let spec = fields
                .get("version")
                .ok_or_else(|| format!("pin `{name}` needs a `version`"))?
                .as_str()
                .ok_or_else(|| format!("pin `{name}`: `version` must be a string"))?;
            let reason = fields
                .get("reason")
                .map(|reason| {
                    reason
                        .as_str()
                        .map(str::to_owned)
                        .ok_or_else(|| format!("pin `{name}`: `reason` must be a string"))
                })
                .transpose()?;
            Entry {
                spec: spec.to_owned(),
                reason,
            }
        }
        _ => {
            return Err(format!(
                "pin `{name}` must be a string or a table with `version`"
            ))
        }
    };
    Spec::parse(&entry.spec).map_err(|error| format!("{name}: {error}"))?;
    Ok(entry)
}

fn load_as(path: &Path, local: bool) -> Result<PinFile, String> {
    let table = table(path)?;
    let mut pins = BTreeMap::new();
    if let Some(value) = table.get("pins") {
        let entries = value
            .as_table()
            .ok_or_else(|| format!("{}: `pins` must be a table", path.display()))?;
        for (name, value) in entries {
            let entry =
                entry(name, value).map_err(|error| format!("{}: {error}", path.display()))?;
            pins.insert(name.clone(), entry);
        }
    }
    Ok(PinFile {
        path: path.to_path_buf(),
        pins,
        local,
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
        assert_eq!(
            found.pins.get("aep").map(|entry| entry.spec.as_str()),
            Some("0.59")
        );
        assert!(!found.local);
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

    #[test]
    fn the_local_pin_file_is_b10x_pins_then_xdg_config_home_then_the_home_config() {
        let home = Path::new("/opt/b10x-home");
        assert_eq!(
            local_path_from(
                home,
                Some("/opt/pins/held.toml".into()),
                Some("/opt/xdg".into())
            ),
            PathBuf::from("/opt/pins/held.toml")
        );
        assert_eq!(
            local_path_from(home, Some(OsString::new()), Some("/opt/xdg".into())),
            PathBuf::from("/opt/xdg/b10x/pins.toml")
        );
        assert_eq!(
            local_path_from(home, None, Some(OsString::new())),
            PathBuf::from("/opt/b10x-home/.config/b10x/pins.toml")
        );
        assert_eq!(
            local_path_from(home, None, None),
            PathBuf::from("/opt/b10x-home/.config/b10x/pins.toml")
        );
    }

    #[test]
    fn a_missing_local_file_pins_nothing_unless_b10x_pins_names_it() {
        let root = scratch("local-missing");
        let home = root.join("home");
        assert_eq!(read_local_from(&home, None, None), Ok(None));
        assert_eq!(
            read_local_from(&home, None, Some(root.join("xdg").into())),
            Ok(None)
        );
        let named = root.join("held.toml");
        let error = read_local_from(&home, Some(named.clone().into()), None).unwrap_err();
        assert!(
            error.starts_with(&named.display().to_string()) && error.contains("does not exist"),
            "{error}"
        );
        std::fs::write(&named, "[pins]\naep = \"0.71.3\"\n").unwrap();
        let file = read_local_from(&home, Some(named.clone().into()), None)
            .unwrap()
            .unwrap();
        assert!(file.local);
        assert_eq!(file.path, named);
        let config = home.join(".config/b10x");
        std::fs::create_dir_all(&config).unwrap();
        std::fs::write(config.join("pins.toml"), "[pins]\ness = \"0.59\"\n").unwrap();
        let file = read_local_from(&home, None, None).unwrap().unwrap();
        assert_eq!(file.path, config.join("pins.toml"));
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn a_pin_is_a_string_or_a_table_with_an_optional_reason() {
        let root = scratch("forms");
        let path = root.join("pins.toml");
        std::fs::write(
            &path,
            "[pins]\naep = { version = \"0.71.3\", reason = \"0.72.0 refused existing stores\" }\ness = \"v0.59\"\nworktree = { version = \"0.18\" }\n",
        )
        .unwrap();
        let file = load(&path).unwrap();
        assert_eq!(
            file.pins.get("aep"),
            Some(&Entry {
                spec: "0.71.3".to_owned(),
                reason: Some("0.72.0 refused existing stores".to_owned()),
            })
        );
        assert_eq!(
            file.pins.get("ess"),
            Some(&Entry {
                spec: "v0.59".to_owned(),
                reason: None,
            }),
            "the spec is kept as written"
        );
        assert_eq!(
            file.pins.get("worktree").map(|entry| entry.spec.as_str()),
            Some("0.18")
        );
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn a_malformed_table_pin_is_refused() {
        let root = scratch("malformed");
        let path = root.join("pins.toml");
        for (text, says) in [
            (
                "[pins]\naep = { version = \"0.71.3\", why = \"x\" }\n",
                "`why`",
            ),
            ("[pins]\naep = { reason = \"x\" }\n", "`version`"),
            (
                "[pins]\naep = { version = \"0.71.3\", reason = 3 }\n",
                "`reason`",
            ),
            ("[pins]\naep = { version = 71 }\n", "`version`"),
            ("[pins]\naep = { version = \"soon\" }\n", "not a version"),
            ("[pins]\naep = 3\n", "aep"),
        ] {
            std::fs::write(&path, text).unwrap();
            let error = load(&path).unwrap_err();
            assert!(
                error.starts_with(&path.display().to_string()) && error.contains(says),
                "{text}: {error}"
            );
        }
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn a_local_pin_wins_over_the_repository_pin_for_the_same_cli() {
        let repository = PinFile {
            path: PathBuf::from("/work/repo/b10x.toml"),
            pins: BTreeMap::from([
                ("aep".to_owned(), Entry::new("0.70")),
                ("ess".to_owned(), Entry::new("0.59.0")),
            ]),
            local: false,
        };
        let local = PinFile {
            path: PathBuf::from("/opt/b10x-home/.config/b10x/pins.toml"),
            pins: BTreeMap::from([(
                "aep".to_owned(),
                Entry {
                    spec: "0.71.3".to_owned(),
                    reason: Some("held".to_owned()),
                },
            )]),
            local: true,
        };
        let merged = merge(Some(&repository), Some(&local));
        let aep = merged.get("aep").unwrap();
        assert_eq!(aep.spec, "0.71.3");
        assert!(aep.local);
        assert_eq!(aep.reason.as_deref(), Some("held"));
        assert_eq!(aep.file, local.path);
        let ess = merged.get("ess").unwrap();
        assert!(!ess.local);
        assert_eq!(ess.file, repository.path);
        assert!(merge(None, None).is_empty());
    }

    // Adversary: a local pin file at the default location that is a dangling symlink (a dotfiles
    // link whose target moved) is a hold that cannot be read. `path.exists()` follows the link and
    // says `false`, so it is read as "no file" and the hold is dropped without a word, in check
    // and in init/upgrade alike.
    #[test]
    fn adversary_a_dangling_default_local_pin_file_is_not_silently_ignored() {
        let root = scratch("adv-dangling");
        let home = root.join("home");
        let config = home.join(".config/b10x");
        std::fs::create_dir_all(&config).unwrap();
        std::os::unix::fs::symlink(root.join("dotfiles/pins.toml"), config.join("pins.toml"))
            .unwrap();
        let read = read_local_from(&home, None, None);
        assert!(read.is_err(), "the hold is dropped silently: {read:?}");
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn a_relative_b10x_pins_is_refused() {
        let error = read_local_from(Path::new("/opt/b10x-home"), Some("pins.toml".into()), None)
            .unwrap_err();
        assert_eq!(error, "$B10X_PINS must be an absolute path: pins.toml");
    }

    #[test]
    fn a_toml_syntax_error_is_one_line_with_its_position() {
        let root = scratch("syntax");
        let path = root.join(FILE);
        std::fs::write(&path, "[pins]\naep = = 0.71.3\n").unwrap();
        let error = load(&path).unwrap_err();
        assert!(!error.contains('\n'), "{error}");
        assert!(
            error.starts_with(&format!("{}: ", path.display())) && error.contains("line 2"),
            "{error}"
        );
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn a_local_hold_is_named_for_pin_and_unpin() {
        let local = PinFile {
            path: PathBuf::from("/opt/b10x-home/.config/b10x/pins.toml"),
            pins: BTreeMap::from([("aep".to_owned(), Entry::new("0.71.3"))]),
            local: true,
        };
        assert_eq!(
            local_hold(Some(&local), "aep").as_deref(),
            Some("a local pin in /opt/b10x-home/.config/b10x/pins.toml holds aep at 0.71.3; it wins over b10x.toml on this machine.")
        );
        assert_eq!(local_hold(Some(&local), "ess"), None);
        assert_eq!(local_hold(None, "aep"), None);
    }

    #[test]
    fn whitespace_runs_in_a_reason_print_as_one_space() {
        let pinned = Pinned {
            spec: "0.71.3".to_owned(),
            tag: None,
            file: "/opt/pins.toml".to_owned(),
            reason: Some("refused\n  existing\tstores".to_owned()),
            local: true,
        };
        assert_eq!(
            pinned.phrase(),
            "held at 0.71.3 by the local pin in /opt/pins.toml (refused existing stores)"
        );
    }
}
