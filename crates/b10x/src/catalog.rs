//! The catalog: which products exist, which plugins and binaries make each one, and which names
//! are retired. It carries no version of anything it points at — versions are read at run time from
//! the host, the marketplace and the product's newest release.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// The catalog this binary was built with, used when no marketplace copy can be read.
pub const EMBEDDED: &str = include_str!("../../../catalog.json");

/// The only catalog format this binary reads.
pub const FORMAT: &str = "b10x.catalog/2";

/// The whole catalog.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Catalog {
    /// Always [`FORMAT`].
    pub format: String,
    /// The marketplace every plugin installs from.
    pub marketplace: Marketplace,
    /// Plugins installed whatever the user selects.
    pub base: Vec<String>,
    /// Products the user chooses between.
    pub products: Vec<Product>,
    /// Retired plugin name → the plugin that replaces it.
    pub retired_plugins: BTreeMap<String, String>,
    /// Marketplace names that no longer serve anything.
    pub retired_marketplaces: Vec<String>,
}

/// The marketplace identity and its GitHub repository.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Marketplace {
    /// `b10x`.
    pub name: String,
    /// `owner/repo`.
    pub repository: String,
}

/// One product: a set of plugins and the binaries they drive.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Product {
    /// What the user selects, e.g. `ess`.
    pub id: String,
    /// Offered, never preselected.
    #[serde(default)]
    pub optional: bool,
    /// What the user wants to do, as the onboarding question offers it.
    pub intent: String,
    /// One line for the selection prompt.
    pub summary: String,
    /// Plugin names in the marketplace.
    pub plugins: Vec<String>,
    /// Binaries the plugins run.
    pub binaries: Vec<Binary>,
}

/// A binary a product's plugins run. Every binary is bound to its repository's newest release.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Binary {
    /// Executable name on `PATH`.
    pub name: String,
    /// Reported but never installed unprompted.
    #[serde(default)]
    pub optional: bool,
    /// Operating systems it runs on (`linux`, `macos`); empty means every one.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub platforms: Vec<String>,
    /// The ways it can be installed.
    pub install: Install,
}

impl Binary {
    /// Whether it runs on the operating system of `target` (a release-archive triple).
    #[must_use]
    pub fn runs_on(&self, target: Option<&str>) -> bool {
        self.platforms.is_empty()
            || target.is_some_and(|target| {
                self.platforms.iter().any(|os| match os.as_str() {
                    "linux" => target.contains("-linux"),
                    "macos" => target.contains("-apple-darwin"),
                    _ => false,
                })
            })
    }
}

/// The ways a binary can be installed; at least one is present.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Install {
    /// `<name>-<version>-<target>.tar.gz` plus `SHA256SUMS` on the release.
    pub archive: Option<Archive>,
    /// `cargo install --git … --tag … <package>`.
    pub cargo: Option<Cargo>,
}

/// A release that carries prebuilt archives.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Archive {
    /// `owner/repo`.
    pub repository: String,
}

/// A cargo package that builds the binary.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Cargo {
    /// `owner/repo`.
    pub repository: String,
    /// Cargo package.
    pub package: String,
}

/// How to install a binary this time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Method {
    /// The release's checksummed archive.
    Prebuilt,
    /// `cargo install` from the release tag.
    Cargo,
}

impl Install {
    /// The GitHub repository the binary is released from.
    #[must_use]
    pub fn repository(&self) -> &str {
        self.archive
            .as_ref()
            .map(|archive| archive.repository.as_str())
            .or_else(|| self.cargo.as_ref().map(|cargo| cargo.repository.as_str()))
            .unwrap_or_default()
    }
}

impl Catalog {
    /// Parse and check a catalog document.
    pub fn parse(text: &str) -> Result<Self, String> {
        let catalog: Catalog =
            serde_json::from_str(text).map_err(|error| format!("catalog: {error}"))?;
        if catalog.format != FORMAT {
            return Err(format!(
                "catalog format `{}` is not `{FORMAT}`",
                catalog.format
            ));
        }
        Ok(catalog)
    }

    /// The catalog compiled into this binary.
    ///
    /// # Panics
    /// Never for a released binary: the build's own tests parse it.
    #[must_use]
    pub fn embedded() -> Self {
        Self::parse(EMBEDDED).expect("the embedded catalog parses")
    }

    /// The product that ships this plugin.
    #[must_use]
    pub fn product_of(&self, plugin: &str) -> Option<&Product> {
        self.products
            .iter()
            .find(|product| product.plugins.iter().any(|name| name == plugin))
    }

    /// A product by id.
    #[must_use]
    pub fn product(&self, id: &str) -> Option<&Product> {
        self.products.iter().find(|product| product.id == id)
    }

    /// The current name for a plugin name, which is itself unless it is retired.
    #[must_use]
    pub fn current_name<'a>(&'a self, name: &'a str) -> &'a str {
        self.retired_plugins.get(name).map_or(name, String::as_str)
    }

    /// Whether the catalog knows this plugin name, current or retired.
    #[must_use]
    pub fn knows(&self, name: &str) -> bool {
        self.retired_plugins.contains_key(name)
            || self.base.iter().any(|base| base == name)
            || self.product_of(name).is_some()
    }

    /// Whether this marketplace name is retired.
    #[must_use]
    pub fn retired_marketplace(&self, name: &str) -> bool {
        self.retired_marketplaces
            .iter()
            .any(|retired| retired == name)
    }

    /// The binary with this name, and its product.
    #[must_use]
    pub fn binary(&self, name: &str) -> Option<(&Product, &Binary)> {
        self.products.iter().find_map(|product| {
            product
                .binaries
                .iter()
                .find(|binary| binary.name == name)
                .map(|binary| (product, binary))
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_binary_limited_to_linux_runs_only_on_linux_targets() {
        let mut binary = super::Binary {
            name: "b10x-harness".to_owned(),
            optional: true,
            platforms: vec!["linux".to_owned()],
            install: super::Install {
                archive: None,
                cargo: None,
            },
        };
        assert!(binary.runs_on(Some("x86_64-unknown-linux-gnu")));
        assert!(binary.runs_on(Some("aarch64-unknown-linux-gnu")));
        assert!(!binary.runs_on(Some("aarch64-apple-darwin")));
        assert!(!binary.runs_on(None));
        binary.platforms.clear();
        assert!(binary.runs_on(None));
    }

    use super::*;

    #[test]
    fn the_embedded_catalog_parses_and_names_no_version() {
        let catalog = Catalog::embedded();
        assert_eq!(catalog.marketplace.name, "b10x");
        assert!(catalog.product("ess").is_some());
        let digit_dot_digit = EMBEDDED
            .as_bytes()
            .windows(3)
            .any(|w| w[0].is_ascii_digit() && w[1] == b'.' && w[2].is_ascii_digit());
        assert!(!digit_dot_digit, "the catalog must not pin a version");
    }

    #[test]
    fn retired_names_map_to_current_ones() {
        let catalog = Catalog::embedded();
        assert_eq!(catalog.current_name("workspace-hygiene"), "worktree");
        assert_eq!(catalog.current_name("aep-plan"), "aep");
        assert_eq!(catalog.current_name("aep"), "aep");
        assert!(catalog.knows("ess-specify"));
        assert!(catalog.retired_marketplace("beyond10x"));
        assert!(!catalog.retired_marketplace("b10x"));
    }
}
