//! Installing one binary at one exact release: a checksummed release archive, or `cargo install`
//! from the tag. Either way the binary is staged first and moved into place with one rename.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::{Digest, Sha256};

use crate::catalog::{Install, Method};
use crate::inventory::home;

/// The Rust target triple of release archives for this machine.
pub fn target() -> Result<&'static str, String> {
    match (std::env::consts::ARCH, std::env::consts::OS) {
        ("x86_64", "linux") => Ok("x86_64-unknown-linux-gnu"),
        ("aarch64", "linux") => Ok("aarch64-unknown-linux-gnu"),
        ("x86_64", "macos") => Ok("x86_64-apple-darwin"),
        ("aarch64", "macos") => Ok("aarch64-apple-darwin"),
        (arch, os) => Err(format!("no release archive for {arch}-{os}")),
    }
}

fn run(program: &str, arguments: &[&str]) -> Result<(), String> {
    let output = Command::new(program)
        .args(arguments)
        .output()
        .map_err(|error| format!("{program}: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "{program} {} failed: {}",
            arguments.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

/// The release archive of `name` at `tag` for `target`: `<name>-<version>-<target>.tar.gz`.
#[must_use]
pub fn archive_name(name: &str, tag: &str, target: &str) -> String {
    format!("{}{target}.tar.gz", archive_prefix(name, tag))
}

/// What every archive name of `name` at `tag` starts with, before the target.
fn archive_prefix(name: &str, tag: &str) -> String {
    format!("{name}-{}-", tag.trim_start_matches('v'))
}

/// `(digest, file)` for every line of a `SHA256SUMS` document.
fn listed(sums: &str) -> impl Iterator<Item = (&str, &str)> {
    sums.lines().filter_map(|line| {
        let (digest, name) = line.split_once(char::is_whitespace)?;
        Some((digest, name.trim().trim_start_matches('*')))
    })
}

/// The hex SHA-256 listed for `file` in a `SHA256SUMS` document.
#[must_use]
pub fn listed_digest<'a>(sums: &'a str, file: &str) -> Option<&'a str> {
    listed(sums).find_map(|(digest, name)| (name == file).then_some(digest))
}

/// The targets a `SHA256SUMS` document lists an archive of `name` at `tag` for.
#[must_use]
pub fn listed_targets(sums: &str, name: &str, tag: &str) -> BTreeSet<String> {
    let prefix = archive_prefix(name, tag);
    listed(sums)
        .filter_map(|(_, file)| file.strip_prefix(&prefix)?.strip_suffix(".tar.gz"))
        .filter(|target| !target.is_empty() && !target.contains('/'))
        .map(str::to_owned)
        .collect()
}

/// Lower-case hex of bytes.
#[must_use]
pub fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    bytes.iter().fold(String::new(), |mut out, byte| {
        let _ = write!(out, "{byte:02x}");
        out
    })
}

fn sha256(path: &Path) -> Result<String, String> {
    let bytes = std::fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
    Ok(hex(&Sha256::digest(bytes)))
}

fn staging(name: &str) -> Result<PathBuf, String> {
    let path = home()
        .join(".cache/b10x/install")
        .join(format!("{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&path);
    std::fs::create_dir_all(&path).map_err(|error| format!("{}: {error}", path.display()))?;
    Ok(path)
}

fn from_archive(name: &str, tag: &str, repository: &str, stage: &Path) -> Result<PathBuf, String> {
    let target = target()?;
    let version = tag.trim_start_matches('v');
    let archive = archive_name(name, tag, target);
    let base = format!("https://github.com/{repository}/releases/download/{tag}");
    let archive_path = stage.join(&archive);
    let sums_path = stage.join("SHA256SUMS");
    for (url, path) in [
        (format!("{base}/{archive}"), &archive_path),
        (format!("{base}/SHA256SUMS"), &sums_path),
    ] {
        run(
            "curl",
            &["-fsSL", "--retry", "2", "-o", &path.to_string_lossy(), &url],
        )?;
    }
    let sums = std::fs::read_to_string(&sums_path).map_err(|error| error.to_string())?;
    let expected = listed_digest(&sums, &archive)
        .ok_or_else(|| format!("SHA256SUMS does not list {archive}"))?;
    let actual = sha256(&archive_path)?;
    if actual != expected {
        return Err(format!(
            "{archive}: checksum {actual} is not the listed {expected}"
        ));
    }
    run(
        "tar",
        &[
            "-xzf",
            &archive_path.to_string_lossy(),
            "-C",
            &stage.to_string_lossy(),
        ],
    )?;
    let binary = stage.join(format!("{name}-{version}-{target}")).join(name);
    if binary.is_file() {
        Ok(binary)
    } else {
        Err(format!("{archive} has no {name}"))
    }
}

fn from_cargo(
    name: &str,
    tag: &str,
    repository: &str,
    package: &str,
    stage: &Path,
) -> Result<PathBuf, String> {
    let url = format!("https://github.com/{repository}");
    run(
        "cargo",
        &[
            "install",
            "--git",
            &url,
            "--tag",
            tag,
            "--locked",
            "--root",
            &stage.to_string_lossy(),
            package,
        ],
    )
    .map_err(|error| {
        if error.starts_with("cargo:") {
            format!("{name} needs a Rust toolchain (https://rustup.rs): {error}")
        } else {
            error
        }
    })?;
    Ok(stage.join("bin").join(name))
}

/// Install `name` at `tag` into `directory`, replacing any copy there in one rename.
pub fn install(
    name: &str,
    tag: &str,
    method: Method,
    install: &Install,
    directory: &Path,
) -> Result<PathBuf, String> {
    let stage = staging(name)?;
    let result = (|| {
        let built = match (method, &install.archive, &install.cargo) {
            (Method::Prebuilt, Some(archive), _) => {
                from_archive(name, tag, &archive.repository, &stage)?
            }
            (Method::Cargo, _, Some(cargo)) => {
                from_cargo(name, tag, &cargo.repository, &cargo.package, &stage)?
            }
            (method, _, _) => return Err(format!("{name} cannot be installed by {method:?}")),
        };
        std::fs::create_dir_all(directory)
            .map_err(|error| format!("{}: {error}", directory.display()))?;
        let destination = directory.join(name);
        let incoming = directory.join(format!(".{name}.b10x-new"));
        std::fs::copy(&built, &incoming)
            .map_err(|error| format!("{}: {error}", incoming.display()))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&incoming, std::fs::Permissions::from_mode(0o755))
                .map_err(|error| error.to_string())?;
        }
        std::fs::rename(&incoming, &destination)
            .map_err(|error| format!("{}: {error}", destination.display()))?;
        Ok(destination)
    })();
    let _ = std::fs::remove_dir_all(&stage);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_are_matched_by_exact_file_name() {
        let sums = "aaa  ess-0.30.0-x86_64-unknown-linux-gnu.tar.gz\nbbb *ess-0.30.0-aarch64-apple-darwin.tar.gz\n";
        assert_eq!(
            listed_digest(sums, "ess-0.30.0-x86_64-unknown-linux-gnu.tar.gz"),
            Some("aaa")
        );
        assert_eq!(
            listed_digest(sums, "ess-0.30.0-aarch64-apple-darwin.tar.gz"),
            Some("bbb")
        );
        assert_eq!(listed_digest(sums, "ess-0.30.0.tar.gz"), None);
    }

    #[test]
    fn targets_are_read_from_the_archive_names_sums_lists() {
        let sums = "\
aaa  b10x-harness-0.13.2-x86_64-unknown-linux-gnu.tar.gz
bbb  b10x-harness-0.13.2-x86_64-unknown-linux-gnu.tar.gz.sig
ccc *b10x-harness-0.13.1-aarch64-unknown-linux-gnu.tar.gz
ddd  harness-0.13.2-aarch64-apple-darwin.tar.gz
eee  b10x-harness-lsp-0.13.2-aarch64-unknown-linux-gnu.tar.gz
fff  b10x-harness-0.13.2-.tar.gz
";
        assert_eq!(
            listed_targets(sums, "b10x-harness", "0.13.2"),
            BTreeSet::from(["x86_64-unknown-linux-gnu".to_owned()])
        );
        assert_eq!(
            listed_targets(sums, "b10x-harness", "v0.13.2"),
            listed_targets(sums, "b10x-harness", "0.13.2"),
            "a leading v on the tag is not part of the archive name"
        );
        assert!(listed_targets("", "b10x-harness", "0.13.2").is_empty());
        for target in listed_targets(sums, "b10x-harness", "0.13.2") {
            let name = archive_name("b10x-harness", "0.13.2", &target);
            assert_eq!(listed_digest(sums, &name), Some("aaa"));
        }
    }
}
