use std::fmt::{Debug, Display};
use std::path::Path;
use std::{fs, io};

use serde::{Deserialize, Serialize};
pub use toml::de::Error as DeserializeError;
pub use toml::ser::Error as SerializeError;
use toml::Table;

use self::package::Package;
use self::template::Template;
use self::tool::Tool;
use crate::heuristics;
use crate::heuristics::Heuristics;

pub mod author;
pub mod categories;
pub mod disciplines;
pub mod ident;
pub mod license;
pub mod package;
pub mod template;
pub mod tool;
pub mod website;

/// A typst.toml manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    /// The `package` key, storing a package's metadata.
    pub package: Package,

    /// The `template` key, storing a packages's template metadata.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<Template>,

    /// The `tool` key, storing 3rd-party configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<Tool>,
}

impl Manifest {
    pub fn package(package: Package) -> Manifest {
        Manifest {
            package,
            template: None,
            tool: None,
        }
    }

    pub fn template(package: Package, template: Template) -> Manifest {
        Manifest {
            package,
            template: Some(template),
            tool: None,
        }
    }

    /// Tries to find the manifest for the project containing `path`. If `path`
    /// is relative, then it may not discover the project root, if it lies above
    /// the relative root. See [heuristics::try_find_project_root] for more info
    /// on how the manifest is discovered.
    ///
    /// Returns `None` if no manifest could be found, returns an error if
    /// [heuristics::try_find_project_root] fails, or if a manifest was found
    /// but could not be parsed.
    ///
    /// # Examples
    /// ```no_run
    /// use typst_project::manifest::Manifest;
    /// use std::env::current_dir;
    ///
    /// let pwd = current_dir()?;
    /// match Manifest::try_find(pwd)? {
    ///     Some(manifest) => println!("Manifest found: {manifest:#?}"),
    ///     None => println!("No manifest found"),
    /// }
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// [heuristics::try_find_project_root]: crate::heuristics::try_find_project_root
    pub fn try_find<P: AsRef<Path>>(path: P) -> Result<Option<Manifest>, Error> {
        let Some((root, _)) =
            heuristics::try_find_project_root(path.as_ref(), Heuristics::MANIFEST_FILE, true)?
        else {
            return Ok(None);
        };

        let content = fs::read_to_string(root.join(heuristics::MANIFEST_FILE))?;
        let manifest = Manifest::from_str(&content)?;
        Ok(Some(manifest))
    }
}

impl Manifest {
    /// Deserializes a manifest from a [`Value`][toml::Value].
    ///
    /// Returns a error if deserialization fails.
    ///
    /// # Examples
    /// ```
    /// use typst_project::manifest::Manifest;
    /// use toml::{toml, Value};
    ///
    /// let toml = toml! {
    ///     [package]
    ///     name = "Foo"
    ///     version = "0.1.0"
    ///     entrypoint = "/src/lib.typ"
    ///     authors = ["tingerrr <me@tinger.dev>"]
    ///     license = "MIT"
    ///     description = "Bar"
    /// };
    ///
    /// let manifest = Manifest::from_value(toml)?;
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_value(toml: Table) -> Result<Self, DeserializeError> {
        Self::deserialize(toml)
    }

    /// Deserializes a manifest from the contents of a manifest file.
    ///
    /// Returns a error if deserialization fails.
    ///
    /// # Examples
    /// ```
    /// use typst_project::manifest::Manifest;
    ///
    /// let toml = r#"
    ///     [package]
    ///     name = "Foo"
    ///     version = "0.1.0"
    ///     entrypoint = "src/lib.typ"
    ///     authors = ["John Doe <john@doe.com>"]
    ///     license = "MIT"
    ///     description = "Bar"
    /// "#;
    ///
    /// let manifest = Manifest::from_str(toml)?;
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_str(toml: &str) -> Result<Self, DeserializeError> {
        toml::from_str(toml)
    }
}

/// An error that may occur during manifest discovery or parsing.
#[derive(Debug)]
pub enum Error {
    /// A generic I/O error occured.
    Io(io::Error),

    /// A serialization error occured.
    Ser(SerializeError),

    /// A deserialization error occured.
    De(DeserializeError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Io(_) => "an I/O error occured",
            Self::Ser(_) => "serialization failed",
            Self::De(_) => "deserialization failed",
        })
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            Error::Io(err) => err,
            Error::Ser(err) => err,
            Error::De(err) => err,
        })
    }
}

macro_rules! impl_from {
    ($err:ty => $var:ident) => {
        impl From<$err> for Error {
            fn from(err: $err) -> Self {
                Self::$var(err)
            }
        }
    };
}

impl_from!(io::Error => Io);
impl_from!(SerializeError => Ser);
impl_from!(DeserializeError => De);
