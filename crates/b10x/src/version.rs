//! Reading versions out of `--version` lines and release tags.

/// The last `x.y.z` token in a `--version` line or tag, without a leading `v`.
#[must_use]
pub fn parse(text: &str) -> Option<String> {
    text.split_whitespace()
        .rev()
        .map(|token| token.trim_start_matches('v'))
        .find(|token| key(token).is_some())
        .map(str::to_owned)
}

/// A comparable key for a bare `x.y.z` version.
#[must_use]
pub fn key(version: &str) -> Option<(u64, u64, u64)> {
    let mut parts = version.trim_start_matches('v').split('.');
    let triple = (
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
    );
    parts.next().is_none().then_some(triple)
}

/// Whether two version spellings name the same release (`v0.11.0` = `0.11.0`).
#[must_use]
pub fn same(left: &str, right: &str) -> bool {
    match (key(left), key(right)) {
        (Some(left), Some(right)) => left == right,
        _ => left == right,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_last_version_token_wins() {
        assert_eq!(parse("protocol 0.57.0").as_deref(), Some("0.57.0"));
        assert_eq!(parse("b10x-worktree-cli 0.4.1\n").as_deref(), Some("0.4.1"));
        assert_eq!(parse("connectors v0.11.0").as_deref(), Some("0.11.0"));
        assert_eq!(parse("no version here"), None);
    }

    #[test]
    fn tags_and_versions_compare() {
        assert!(same("v0.11.0", "0.11.0"));
        assert!(!same("0.26.0", "0.30.0"));
        assert!(key("0.30.0") > key("0.26.0"));
    }
}
