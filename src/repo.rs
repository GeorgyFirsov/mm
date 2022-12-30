use std::path::Path;
use std::fs;
use git2::Repository;


/// Path to repository relative to user's home folder
const MM_MAIN_REPO_FOLDER: &str = ".mm/repo/";

/// Name of main repository
const MM_MAIN_REPO_NAME: &str = "mm_repo";


/// Get full repository folder path
fn get_main_repo_folder() -> String {
    "~/".to_owned() + MM_MAIN_REPO_FOLDER
}


/// Check if repository folder exists
fn is_main_repo_folder_present() -> bool {
    let repo = get_main_repo_folder();
    Path::new(&repo).exists()
}


/// Compose full repository path
fn get_main_repo_path() -> String {
    get_main_repo_folder() + MM_MAIN_REPO_NAME
}


/// Returns a repository ready to use
pub(crate) fn open_repo() -> Result<Repository, git2::Error> {
    if !is_main_repo_folder_present() {
        //
        // No path is present, let's create it
        //

        fs::create_dir_all(get_main_repo_folder());
    }

    //
    // Now let's try to open repository.
    // If it is absent, we need to create 
    // it first
    //

    Repository::open(get_main_repo_path())

    // TODO: error handling
}