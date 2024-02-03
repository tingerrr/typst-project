use std::path::Path;
use std::{fmt, fs, io};

/// The name of the typst manifest file.
pub const MANIFEST_FILE: &str = "typst.toml";

/// All files which can be found in a typst project.
pub const ROOT_FILES: &[(&str, Heuristic)] = &[
    (MANIFEST_FILE, Heuristic::ManifestFile),
    #[cfg(feature = "heuristics-typstfmt")]
    ("typstfmt.toml", Heuristic::TypstfmtConfig),
];

/// A single heuristic, see [Heuristics] for bitflags to represent more than
/// one heuristic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Heuristic {
    /// A typst.toml manifest file was found.
    ManifestFile,

    /// A typstfmt.toml config file was found.
    #[cfg(feature = "heuristics-typstfmt")]
    TypstfmtConfig,
}

impl From<Heuristic> for Heuristics {
    fn from(value: Heuristic) -> Self {
        match value {
            Heuristic::ManifestFile => Heuristics::MANIFEST_FILE,
            #[cfg(feature = "heuristics-typstfmt")]
            Heuristic::TypstfmtConfig => Heuristics::TYPSTFMT_CONFIG,
        }
    }
}

bitflags::bitflags! {
    /// A set of heuristics.
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Heuristics: u32 {
        /// A heuristic to look for typst.toml manifest files.
        const MANIFEST_FILE = 1 << 0;

        /// A heuristic to look for typstfmt config files.
        #[cfg(feature = "heuristics-typstfmt")]
        const TYPSTFMT_CONFIG = 1 << 1;
    }
}

impl fmt::Debug for Heuristics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        bitflags::parser::to_writer(self, f)
    }
}

/// Recursively looks up the ancestors of `path` until it finds a project root
/// directory which matches the given heurisitcs. If `path` is relative, then
/// it may not discover the project root, if it lies above the relative root.
/// See [project_root] for more info on when a directory is a packge root. If
/// `first` is `true`, the first matched heuristic will be returned without
/// looking for more.
///
/// Returns `None` if no root can be found, returns an error if
/// [project_root] fails.
///
/// # Examples
/// ```no_run
/// use typst_project::heuristics::{try_find_project_root, Heuristics};
/// use std::env::current_dir;
///
/// let pwd = current_dir()?;
/// match try_find_project_root(&pwd, Heuristics::all(), true)? {
///     Some((root, heuristics)) => {
///         println!("Found project root: {root:?}, {heuristics:?}");
///     }
///     None => println!("No project root found"),
/// }
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// ```
pub fn try_find_project_root(
    path: &Path,
    heuristics: Heuristics,
    any: bool,
) -> io::Result<Option<(&Path, Heuristics)>> {
    for ancestor in path.ancestors() {
        let returned = project_root(ancestor, heuristics, any)?;
        if !returned.is_empty() {
            return Ok(Some((ancestor, returned)));
        }
    }

    Ok(None)
}

/// Checks if a directory matches any of the given heuristics. See
/// [project_root] if you need to know which heuristics were matched.
///
/// Returns an error if [read_dir][fs::read_dir] fails.
///
/// # Examples
/// ```no_run
/// use typst_project::heuristics::{is_project_root, Heuristics};
/// use std::env::current_dir;
///
/// let pwd = current_dir()?;
/// if is_project_root(pwd, Heuristics::all())? {
///     println!("PWD is project root");
/// } else {
///     println!("PWD is not project root");
/// }
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// ```
pub fn is_project_root<P: AsRef<Path>>(path: P, heuristics: Heuristics) -> io::Result<bool> {
    project_root(path, heuristics, true).map(|hs| !hs.is_empty())
}

/// Checks if a directory matches any of the given heuristics. If `first` is
/// `true`, the first matched heuristic will be returned without looking for
/// more.
///
/// Returns an error if [read_dir][fs::read_dir] fails.
///
/// # Examples
/// ```no_run
/// use typst_project::heuristics::{project_root, Heuristics};
/// use std::env::current_dir;
///
/// let pwd = current_dir()?;
/// let heuristics = project_root(pwd, Heuristics::all(), true)?;
/// if heuristics.is_empty() {
///     println!("PWD is project root");
/// } else {
///     println!("PWD is not project root");
/// }
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// ```
pub fn project_root<P: AsRef<Path>>(
    path: P,
    heuristics: Heuristics,
    first: bool,
) -> io::Result<Heuristics> {
    fn inner(path: &Path, heuristics: Heuristics, first: bool) -> io::Result<Heuristics> {
        let mut res = Heuristics::empty();

        for entry in fs::read_dir(path)? {
            if let Some(h) = dir_entry(entry?, heuristics)? {
                res |= h.into();

                if first || res == heuristics {
                    break;
                }
            }
        }

        Ok(res)
    }

    inner(path.as_ref(), heuristics, first)
}

fn dir_entry(entry: fs::DirEntry, heuristics: Heuristics) -> io::Result<Option<Heuristic>> {
    let typ = entry.file_type()?;
    let name = entry.file_name();

    if !typ.is_file() {
        return Ok(None);
    }

    Ok(ROOT_FILES
        .iter()
        .copied()
        .filter(|&(_, h)| heuristics.contains(h.into()))
        .find_map(|(f, h)| (name == f).then_some(h)))
}
