//! Typst template metadata.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// The `tempalte` key in the manifest, storing a template's metadata. given the following folder
/// structure of a template package:
/// ```text
/// .
/// ├ typst.toml
/// ├ assets
/// │ └ thumbnail.png
/// └ template
///   ├ chapters
///   │ ├ chapter-1.typ
///   │ └ chapter-2.typ
///   └ main.typ
/// ```
///
/// The typst.toml would look like this:
/// ```toml
/// [package]
/// # ...
///
/// [template]
/// path = "template"
/// entrypoint = "chapters/chapter-1.typ"
/// thumbnail = "assets/thumbnail.png"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Template {
    /// A path _relative to the package's root_ which points to a directory that contains the files
    /// which should be copied into the user's new project directory.
    pub path: PathBuf,

    /// A path _relative to the template's path_ that points to the file serving
    /// as the compilation target.
    pub entrypoint: PathBuf,

    /// A path _relative to the package's root_ that points to a PNG or lossless
    /// WebP thumbnail for the template.
    pub thumbnail: PathBuf,
}
