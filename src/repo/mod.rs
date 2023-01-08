mod helpers;
mod repository;

pub(crate) use self::repository::{ Repository };


/// Path to repositories relative to mm's data folder.
const MM_REPOS_SUBFOLDER: &str = "repos/";

/// Name of main repository.
const MM_MAIN_REPO_NAME: &str = "mm_main_local";
