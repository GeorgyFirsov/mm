use std::path::Path;

use super::Editor;
use crate::error::Result;


/// A struct, that implements a built-in xi-based editor.
struct XiEditor {}


impl Editor for XiEditor {
    fn run(&self, note_path: &Path) -> Result<()> {
        // TODO
        Ok(())
    }
}
