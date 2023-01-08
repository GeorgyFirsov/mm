use std::path::{ Path, PathBuf };

use git2;

use super::helpers;
use super::defs;
use crate::misc;
use crate::error::{ Error, Result, ErrorCategory };


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

        if !helpers::is_repos_folder_present() {
            helpers::get_repos_folder()
                .ok_or(Error::from_string("cannot get repositories folder", ErrorCategory::Os))
                .and_then(misc::create_folder_recursive)?;
        }

        //
        // Now let's try to open an internal git repository.
        // If it doesn't exists, it is neessary to create it.
        //

        let internal_repo = helpers::get_repo_path(&repo_name)
            .ok_or(Error::from_string("cannot get repository path", ErrorCategory::Os))
            .and_then(helpers::open_or_create_repository)?;

        Repository::from_git_repository(internal_repo, repo_name)
    }
    

    /// Adds a note to repository.
    /// 
    /// * `note` - name of a note to add
    /// * `folder` - optional folder to add note to (pass `None` to add note to root folder)
    pub(crate) fn add_note(&self, name: &str, folder: Option<&str>) -> Result<PathBuf> {
        //
        // Folder name if any must be valid
        //

        folder.map_or(Ok(()), helpers::ensure_valid_folder)?;

        //
        // Firstly check a folder for existence. If it does not exist,
        // then it must be created.
        //

        let workdir = self.get_workdir()?;
        let folder_path = workdir.join(folder.unwrap_or("" /* append nothing for root */));
        if !folder_path.exists() {
            //
            // Workdir is already present, so I can unwrap folder parameter.
            // It cannot be None or empty here.
            //

            self.add_folder_internal(folder.unwrap(), workdir)?;
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
            .expect("workdir is not a prefix of path");

        self.add_note_internal(relative_note_path)?;

        Ok(note_path)
    }


    /// Adds a folder to repository. 
    /// 
    /// * `name` - name of a folder to add
    pub(crate) fn add_folder(&self, name: &str) -> Result<()> {
        //
        // Firstly we need to ensure, that we create a valid folder
        //

        helpers::ensure_valid_folder(name)?;

        //
        // Just create a directory. Nothing else is required.
        //

        self.get_workdir()
            .and_then(|workdir| self.add_folder_internal(name, workdir))
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
                .unwrap_or(defs::MM_MAIN_REPO_NAME)
                .to_owned(), 

            remotes: remotes,
        })
    }


    /// Adds a note to repository (internal implementation).
    /// 
    /// Calls `git2::Index::add_all` in order to take `.gitignore` into 
    /// account, because `git2::Index::add_path` forces files to be added.
    /// 
    /// * `path` - relative to working directory path to the note
    fn add_note_internal(&self, path: &Path) -> Result<()> {
        self.internal_repo
            .index()
            .and_then(|mut index| index.add_all([path].iter(), git2::IndexAddOption::DEFAULT, None))
            .map_err(Error::from)
    }


    /// Adds a folder to repository (internal implementation). 
    /// 
    /// Used for optimization: sometimes workdir is already known, so we can 
    /// skip its acquisition.
    /// 
    /// * `name` - name of a folder to add
    /// * `workdir` - repo's working directory
    fn add_folder_internal(&self, name: &str, workdir: &Path) -> Result<()> {
        misc::create_folder(workdir.join(name))
    }


    /// Obtains a working directory for current repository.
    fn get_workdir(&self) -> Result<&Path> {
        self.internal_repo
            .workdir()
            .ok_or(Error::from_string("cannot get working directory", ErrorCategory::Git))
    }
}
