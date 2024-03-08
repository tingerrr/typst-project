use std::path::Path;
use std::{fmt, fs, io};

/// The name of the typst manifest file.
pub const MANIFEST_FILE: &str = "typst.toml";

/// The name of a possible entrypoint for a document.
pub const MAIN_FILE: &str = "main.typ";

/// The name of a possible entrypoint for a package.
pub const LIB_FILE: &str = "lib.typ";

/// All files which can be found in a typst project root.
pub const ROOT_FILES: &[(&str, Heuristic)] = &[
    ("main.typ", Heuristic::MainFile { src: false }),
    ("lib.typ", Heuristic::LibFile { src: false }),
    (MANIFEST_FILE, Heuristic::ManifestFile),
    #[cfg(feature = "heuristics-typstfmt")]
    ("typstfmt.toml", Heuristic::TypstfmtConfig),
];

/// A single heuristic, see [Heuristics] for bitflags to represent more than
/// one heuristic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Heuristic {
    /// A main.typ file was found.
    MainFile { src: bool },

    /// A lib.typ file was found.
    LibFile { src: bool },

    /// A typst.toml manifest file was found.
    ManifestFile,

    /// A typstfmt.toml config file was found.
    #[cfg(feature = "heuristics-typstfmt")]
    TypstfmtConfig,
}

impl From<Heuristic> for Heuristics {
    fn from(value: Heuristic) -> Self {
        match value {
            Heuristic::MainFile { src: false } => Heuristics::MAIN_FILE,
            Heuristic::LibFile { src: false } => Heuristics::LIB_FILE,
            Heuristic::MainFile { src: true } => Heuristics::MAIN_FILE | Heuristics::SRC_FOLDER,
            Heuristic::LibFile { src: true } => Heuristics::LIB_FILE | Heuristics::SRC_FOLDER,
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
        /// A heuristic to look for a main.typ source file.
        const MAIN_FILE = 1 << 0;

        /// A heuristic to look for a lib.typ source file.
        const LIB_FILE = 1 << 0;

        /// A heuristic to look for a main.typ or lib.typ source file in a src
        /// folder instead of the root folder.
        const SRC_FOLDER = 1 << 1;

        /// A heuristic to look for typst.toml manifest files.
        const MANIFEST_FILE = 1 << 2;

        /// A heuristic to look for typstfmt config files.
        #[cfg(feature = "heuristics-typstfmt")]
        const TYPSTFMT_CONFIG = 1 << 3;

        /// The recommended heuristics.
        #[cfg(not(feature = "heuristics-typstfmt"))]
        const RECOMMENDED = Self::MANIFEST_FILE.bits();

        /// The recommended heuristics.
        #[cfg(feature = "heuristics-typstfmt")]
        const RECOMMENDED = Self::MANIFEST_FILE.bits() | Self::TYPSTFMT_CONFIG.bits();
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
            if let Some(h) = potential_root_dir_entry(entry?, heuristics)? {
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

fn potential_root_dir_entry(
    entry: fs::DirEntry,
    heuristics: Heuristics,
) -> io::Result<Option<Heuristic>> {
    let typ = entry.file_type()?;
    let name = entry.file_name();

    if typ.is_dir() {
        if heuristics.contains(Heuristics::SRC_FOLDER) && name == "src" {
            for entry in fs::read_dir(entry.path().join("src"))? {
                let entry = entry?;
                let typ = entry.file_type()?;
                let name = entry.file_name();

                if !typ.is_file() {
                    return Ok(None);
                }

                if name == "main.typ" {
                    return Ok(Some(Heuristic::MainFile { src: true }));
                }

                if name == "lib.typ" {
                    return Ok(Some(Heuristic::LibFile { src: true }));
                }
            }
        }

        return Ok(None);
    }

    if !typ.is_file() {
        return Ok(None);
    }

    Ok(ROOT_FILES
        .iter()
        .copied()
        .filter(|&(_, h)| heuristics.contains(h.into()))
        .find_map(|(f, h)| (name == f).then_some(h)))
}
