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
        .map_err(Error::from)
}


/// Creates a directory (fails if one of its parents is absent)
/// 
/// Actually just calls [`std::fs::create_dir`] and then converts
/// an error to [`crate::error::Error`] if necessary.
/// 
/// * `path` - path to create
pub(crate) fn create_folder<P: AsRef<Path>>(path: P) -> Result<()> {
    fs::create_dir(path)
        .map_err(Error::from)
}


/// Creates a new file in filesystem. Fails if the file already exists.
/// 
/// Technically it is similar to `touch` command in *nix systems. 
/// 
/// * `path` - path to a file to create
pub(crate) fn touch_new_file<P: AsRef<Path>>(path: P) -> Result<()> {
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(Error::from)
        .map(|_file| ())
}
