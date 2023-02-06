use std::process;
use std::path::Path;
use std::ffi::OsString;

use libmm::error::{Error, ErrorCategory, Result};
use super::editor::Editor;


/// A trait, that is used to support external editors. For each 
/// supported editor there should be a struct, that implements 
/// this trait.
pub(crate) trait ExternalEditor {
    /// Returns an executable path or name used for process startup.
    fn executable(&self) -> OsString;

    /// Creates a list of arguments from a path to a note to edit.
    /// 
    /// * `note_path` - full path to a note to edit
    fn make_args(&self, note_path: &Path) -> Vec<OsString>;
}


impl<E: ExternalEditor> Editor for E {
    fn run(&self, note_path: &Path) -> Result<()> {
        //
        // Just launch a child process and wait for its completion
        //

        process::Command::new(self.executable())
            .args(self.make_args(note_path))
            .spawn()
            .map_err(Error::from)
            .and_then(wait_editor)
    }
}


/// Waits for external editor process. 
/// 
/// Fails if editor finishes unsuccessfully.
/// 
/// * `child` - child process
fn wait_editor(mut child: process::Child) -> Result<()> {
    //
    // Well... Firstly we need to wait for process to end
    //

    let status = child.wait()?;
    
    //
    // And now we need to check exit status and convert it 
    // into crate::error::Result
    //
    
    let code = status
        .code()
        .map_or("terminated by signal".to_owned(), |code| code.to_string());

    status
        .success()
        .then_some(())
        .ok_or(Error::from_string(format!("editor exited with code '{:?}'", code), ErrorCategory::Editor))
}
