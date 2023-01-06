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
fn get_repo_path(repo_name: &Option<&str>) -> Option<PathBuf> {
    get_repos_folder()
        .and_then(|path| Some(path.join(repo_name.unwrap_or(MM_MAIN_REPO_NAME))))
}


/// Open or create a repository by its path
fn open_or_create_repository(path: PathBuf) -> Result<git2::Repository> {
    git2::Repository::open(path.clone())
        .or_else(|_error| git2::Repository::init(path))
        .map_err(Error::from_git_error)
}


/// A structure, that describes a repository for notes
pub(crate) struct Repository {
    internal_repo: git2::Repository,
    name: String,
    remotes: Option<git2::string_array::StringArray>,
}


impl Repository {
    /// Returns a repository ready to use
    /// 
    /// Supports opening a repository by its name or a main repo if no name given.
    /// 
    /// * `repo_name` - a name of repository to open (pass `None` to open a main repository)
    pub(crate) fn open_or_create(repo_name: Option<&str>) -> Result<Self> {
        //
        // Firstly we need to ensure, that we have repositories folder.
        // App may be run for the first time or data may be erased, so
        // we need to create the folder if necessary
        //

        if !is_repos_folder_present() {
            //
            // No path is present, let's try to create it
            //
    
            get_repos_folder()
                .ok_or(Error::from_string("cannot get repositories folder", ErrorCategory::Os))
                .and_then(misc::create_folder_recursive)?;
        }

        //
        // Now let's try to open an internal git repository.
        // If it doesn't exists, it is neessary to create it.
        //

        let internal_repo = get_repo_path(&repo_name)
            .ok_or(Error::from_string("cannot get repository path", ErrorCategory::Os))
            .and_then(open_or_create_repository)?;

        Repository::from_git_repository(internal_repo, repo_name)
    }

    
    /// Internal constructor, that constructs a repository instance from 
    /// internal [`git2::Repository`] instance
    fn from_git_repository(repo: git2::Repository, repo_name: Option<&str>) -> Result<Repository> {
        let remotes = repo
            .remotes()
            .ok();

        Ok(Repository { 
            internal_repo: repo, 
            name: repo_name
                .unwrap_or(MM_MAIN_REPO_NAME)
                .to_owned(), 
            remotes: remotes 
        })
    }
    
}