//! Typst package metadata.

use std::collections::HashSet;
use std::path::PathBuf;

use semver::Version;
use serde::{Deserialize, Serialize};

use super::author::Author;
use super::categories::Category;
use super::disciplines::Discipline;
use super::ident::Ident;
use super::license::License;
use super::website::Website;

/// The `package` key in the manifest, storing a package's metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Package {
    /// The name of the package.
    pub name: Ident,

    /// The current verison of the packge.
    pub version: Version,

    /// The primary module of the package.
    pub entrypoint: PathBuf,

    /// The authors of the package.
    pub authors: HashSet<Author>,

    /// The license expression for the package.
    pub license: License,

    /// The description of the package.
    pub description: String,

    /// The homepage URL of the package.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<Website>,

    /// The repository URL of the package.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Website>,

    /// The keywords for the package.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    pub keywords: HashSet<String>,

    /// The categories for the package.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    pub categories: HashSet<Category>,

    /// The disciplines for the package.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    pub disciplines: HashSet<Discipline>,

    /// The minimum compiler version for the package.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiler: Option<Version>,

    /// The excluded paths of this package. These paths are ignored by the
    /// package manager's bundler.
    #[serde(default)]
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    pub exclude: HashSet<PathBuf>,
}
