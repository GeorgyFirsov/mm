use std::path::Path;

use libmm::error::Result;
use super::Editor;


/// A struct, that implements a built-in xi-based editor.
struct XiEditor {}


impl Editor for XiEditor {
    fn run(&self, note_path: &Path) -> Result<()> {
        // TODO
        Ok(())
    }
}
