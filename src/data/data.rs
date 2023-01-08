use std::path::PathBuf;

use dirs;
use super::defs;


/// Get full path to a folder with mm's data
pub(crate) fn get_data_folder() -> Option<PathBuf> {
    dirs::home_dir()
        .and_then(|path| Some(path.join(defs::MM_DATA_FOLDER)))
}