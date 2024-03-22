use std::io;
use std::path::Path;

use heuristics::Heuristics;

pub mod heuristics;
pub mod manifest;

#[macro_use]
mod utils;

/// Recursively looks up the ancestors of `path` until it finds a project root
/// directory which matches any of the recommended heurisitcs. If `path` is
/// relative, then it may not discover the project root, if it lies above the
/// relative root. See [heuristics::try_find_project_root] for more info.
///
/// Returns `None` if no root can be found, returns an error if
/// [heuristics::try_find_project_root] fails.
///
/// # Examples
/// ```no_run
/// use typst_project::try_find_project_root;
/// use std::env::current_dir;
///
/// let pwd = current_dir()?;
/// match try_find_project_root(&pwd)? {
///     Some(root) => println!("Found project root: {root:?}"),
///     None => println!("No project root found"),
/// }
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// ```
pub fn try_find_project_root(path: &Path) -> io::Result<Option<&Path>> {
    heuristics::try_find_project_root(path, Heuristics::RECOMMENDED, true)
        .map(|r| r.map(|(p, _)| p))
}

/// Checks if a directory matches the any of the recommended heuristics. See
/// [heuristics::is_project_root] if you need to control which heuristics
/// should be matched.
///
/// Returns an error if [read_dir] fails.
///
/// # Examples
/// ```no_run
/// use typst_project::is_project_root;
/// use std::env::current_dir;
///
/// let pwd = current_dir()?;
/// if is_project_root(pwd)? {
///     println!("PWD is project root");
/// } else {
///     println!("PWD is not project root");
/// }
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// ```
///
/// [read_dir]: std::fs::read_dir
pub fn is_project_root<P: AsRef<Path>>(path: P) -> io::Result<bool> {
    heuristics::is_project_root(path, Heuristics::RECOMMENDED)
}
