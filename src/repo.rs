use std::path::PathBuf;

use git2;

use crate::data;
use crate::misc;
use crate::error::{ Error, Result, ErrorCategory };


/// Path to repositories relative to mm's data folder.
const MM_REPOS_SUBFOLDER: &str = "repos/";

/// Name of main repository.
const MM_MAIN_REPO_NAME: &str = "mm_main_local";


/// Get full repositories folder path.
fn get_repos_folder() -> Option<PathBuf> {
    data::get_data_folder()
        .map(|path| path.join(MM_REPOS_SUBFOLDER))
}


/// Check if repositories folder exists.
fn is_repos_folder_present() -> bool {
    //
    // Well, let's assume, that inaccessible path is inexistent
    //

    get_repos_folder()
        .map_or(false, |path| path.exists())
}


/// Compose full repository path by its name.
/// 
/// * `repo_name` - a name of repository (or `None` for a main repository)
fn get_repo_path(repo_name: &Option<&str>) -> Option<PathBuf> {
    get_repos_folder()
        .map(|path| path.join(repo_name.unwrap_or(MM_MAIN_REPO_NAME)))
}


/// Open or create a git repository by its path.
/// 
/// * `path` - path to the repository's directory
fn open_or_create_repository(path: PathBuf) -> Result<git2::Repository> {
    git2::Repository::open(path.to_owned())
        .or_else(|_error| git2::Repository::init(path))
        .map_err(Error::from_git_error)
}


/// A structure, that describes a repository for notes.
pub(crate) struct Repository {
    /// Internal git repository, that manages version control
    internal_repo: git2::Repository,

    /// Name of the repository
    name: String,

    /// Optional list of remotes. `None` if repository has no remotes
    remotes: Option<git2::string_array::StringArray>,
}


impl Repository {
    /// Returns a repository ready to use.
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
    /// internal [`git2::Repository`] instance.
    /// 
    /// * `repo` - git repository instance to wrap
    /// * `repo_name` - a name of repository to open (pass `None` to open a main repository)
    fn from_git_repository(repo: git2::Repository, repo_name: Option<&str>) -> Result<Repository> {
        let remotes = repo
            .remotes()
            .ok();

        Ok(Repository { 
            internal_repo: repo, 

            name: repo_name
                .unwrap_or(MM_MAIN_REPO_NAME)
                .to_owned(), 

            remotes: remotes,
        })
    }
    

    /// Adds a note to repository. Returns a writable handle to created file.
    /// 
    /// * `note` - name of a note to add
    /// * `folder` - optional folder to add note to (pass `None` to add note to root folder)
    pub(crate) fn add_note(self, name: &str, folder: Option<&str>) -> Result<PathBuf> {
        let workdir = self.internal_repo
            .workdir()
            .ok_or(Error::from_string("cannot get working directory", ErrorCategory::Git))?;

        //
        // Firstly check a folder for existence. If it does not exist,
        // then it must be created.
        //

        let folder_path = workdir.join(folder.unwrap_or("" /* append nothing for root */));
        if !folder_path.exists() {
            misc::create_folder(&folder_path)?;
        }

        //
        // Now let's try to create a new file. This call should fail, if it exists.
        //

        let note_path = folder_path.join(name);
        misc::touch_new_file(&note_path)?;

        //
        // Now let's add the new file to repository and return its path
        //

        let relative_note_path = note_path
            .strip_prefix(workdir)
            .unwrap();

        self.internal_repo
            .index()
            .and_then(|mut index| index.add_path(relative_note_path))
            .map_err(Error::from_git_error)?;

        Ok(note_path)
    }
}
