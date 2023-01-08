use std::path::PathBuf;

use dirs;

use super::{ MM_DATA_FOLDER };


/// Get full path to a folder with mm's data
pub(crate) fn get_data_folder() -> Option<PathBuf> {
    dirs::home_dir()
        .and_then(|path| Some(path.join(MM_DATA_FOLDER)))
}