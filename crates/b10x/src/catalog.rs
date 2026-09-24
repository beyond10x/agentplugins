//! The catalog: which products exist, which plugins and binaries make each one, and which names
//! are retired. It carries no version of anything it points at — versions are read at run time from
//! the host, the marketplace and the product's newest release.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// The catalog this binary was built with, used when no marketplace copy can be read.
pub const EMBEDDED: &str = include_str!("../../../catalog.json");

/// The only catalog format this binary reads.
pub const FORMAT: &str = "b10x.catalog/1";

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
    /// One line for the selection prompt.
    pub summary: String,
    /// Plugin names in the marketplace.
    pub plugins: Vec<String>,
    /// Binaries the plugins run.
    pub binaries: Vec<Binary>,
}

/// A binary a product's plugins run.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Binary {
    /// Executable name on `PATH`.
    pub name: String,
    /// Reported but never installed unprompted.
    #[serde(default)]
    pub optional: bool,
    /// Which version the binary must have.
    pub bind: Bind,
    /// How to install it.
    pub install: Install,
}

/// Which version a binary must have.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Bind {
    /// The repository's newest release.
    Latest,
    /// The version of this installed plugin, whose text describes that exact binary.
    Plugin {
        /// Plugin name.
        plugin: String,
    },
}

/// How a binary is installed.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Install {
    /// `<name>-<version>-<target>.tar.gz` plus `SHA256SUMS` on the release.
    ReleaseArchive {
        /// `owner/repo`.
        repository: String,
    },
    /// `cargo install --git … --tag … <package>`.
    Cargo {
        /// `owner/repo`.
        repository: String,
        /// Cargo package that builds the binary.
        package: String,
    },
}

impl Install {
    /// The GitHub repository the binary is released from.
    #[must_use]
    pub fn repository(&self) -> &str {
        match self {
            Install::ReleaseArchive { repository } | Install::Cargo { repository, .. } => {
                repository
            }
        }
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
        assert_eq!(catalog.current_name("aep-plan"), "aep-plan");
        assert!(catalog.knows("ess-specify"));
        assert!(catalog.retired_marketplace("beyond10x"));
        assert!(!catalog.retired_marketplace("b10x"));
    }
}
