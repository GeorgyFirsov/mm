use std::path::Path;
use std::ffi::OsString;

use super::external::ExternalEditor;


/// Dummy struct, that provides support a `vim` editor.
pub(crate) struct Vim {}


impl ExternalEditor for Vim {
    fn executable(&self) -> OsString {
        //
        // Just trying to make it as simple as possible
        //

        "vim".into()
    }
    
    fn make_args(&self, note_path: &Path) -> Vec<OsString> {
        //
        // Vim requires path to a file only
        //

        vec![note_path.into()]
    }
}