use std::path::PathBuf;

use super::{ MM_REPOS_SUBFOLDER, MM_MAIN_REPO_NAME };
use crate::data;
use crate::error::{ Result, Error, ErrorCategory };


/// Get full repositories folder path.
pub(super) fn get_repos_folder() -> Option<PathBuf> {
    data::get_data_folder()
        .map(|path| path.join(MM_REPOS_SUBFOLDER))
}


/// Check if repositories folder exists.
pub(super) fn is_repos_folder_present() -> bool {
    //
    // Well, let's assume, that inaccessible path is inexistent
    //

    get_repos_folder()
        .map_or(false, |path| path.exists())
}


/// Compose full repository path by its name.
/// 
/// * `repo_name` - a name of repository (or `None` for a main repository)
pub(super) fn get_repo_path(repo_name: &Option<&str>) -> Option<PathBuf> {
    get_repos_folder()
        .map(|path| path.join(repo_name.unwrap_or(MM_MAIN_REPO_NAME)))
}


/// Open or create a git repository by its path.
/// 
/// * `path` - path to the repository's directory
pub(super) fn open_or_create_repository(path: PathBuf) -> Result<git2::Repository> {
    git2::Repository::open(path.to_owned())
        .or_else(|_error| git2::Repository::init(path))
        .map_err(Error::from)
}


/// Verifies, that `name` is a valid name for folder with notes.
/// 
/// Checks if it is not empty and contains only one level
/// of structure (names with slashes are considered invalid).
/// 
/// * `name` - name of folder to verify
pub(super) fn ensure_valid_folder(name: &str) -> Result<()> {
    let valid = 
        !name.is_empty() && 
        !name.contains("/") && 
        !name.contains("\\");

    valid
        .then_some(())
        .ok_or(Error::from_string(format!("invalid folder name: '{}'", name).as_str(), ErrorCategory::Repo))
}
