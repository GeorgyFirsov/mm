use std::path::PathBuf;

use git2;

use crate::data;
use crate::misc;
use crate::error::{ Error, Result, ErrorCategory };


/// Path to repositories relative to mm's data folder
const MM_REPOS_SUBFOLDER: &str = "repos/";

/// Name of main repository
const MM_MAIN_REPO_NAME: &str = "mm_main_local";


/// Get full repositories folder path
fn get_repos_folder() -> Option<PathBuf> {
    data::get_data_folder()
        .and_then(|path| Some(path.join(MM_REPOS_SUBFOLDER)))
}


/// Check if repositories folder exists
fn is_repos_folder_present() -> bool {
    //
    // Well, let's assume, that inaccessible path is inexistent
    //

    get_repos_folder()
        .map_or(false, |path| path.exists())
}


/// Compose full repository path by its name
/// 
/// * `repo_name` - a name of repository (or `None` for a main repository)
fn get_repo_path(repo_name: Option<&str>) -> Option<PathBuf> {
    get_repos_folder()
        .and_then(|path| Some(path.join(repo_name.unwrap_or(MM_MAIN_REPO_NAME))))
}


/// Open or create a repository by its path
fn open_or_create_repository(path: PathBuf) -> Result<git2::Repository> {
    git2::Repository::open(path.clone())
        .or_else(|_error| git2::Repository::init(path))
        .map_err(Error::from_git_error)
}


/// Returns a repository ready to use
/// 
/// Supports opening a repository by its name or a main repo if no name given.
/// 
/// * `repo_name` - a name of repository to open (pass `None` to open a main repository)
pub(crate) fn open_repo(repo_name: Option<&str>) -> Result<git2::Repository> {
    if !is_repos_folder_present() {
        //
        // No path is present, let's create it
        //

        get_repos_folder()
            .ok_or(Error::from_string("cannot get repositories folder", ErrorCategory::Os))
            .and_then(misc::create_folder_recursive)?;
    }

    //
    // Now let's try to open repository. If it is absent, we 
    // need to create it first
    //

    get_repo_path(repo_name)
        .ok_or(Error::from_string("cannot get repository path", ErrorCategory::Os))
        .and_then(open_or_create_repository)
}