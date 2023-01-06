use std::fs;
use std::path::Path;

use crate::error::{ Error, Result };


/// Creates a directory and its parents recursively
/// 
/// Actually just calls [`std::fs::create_dir_all`] and then converts
/// an error to [`crate::error::Error`] if necessary.
/// 
/// * `path` - path to create
pub(crate) fn create_folder_recursive<P: AsRef<Path>>(path: P) -> Result<()> {
    fs::create_dir_all(path)
        .map_err(Error::from_io_error)
}
