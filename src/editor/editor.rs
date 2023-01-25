use std::path::Path;

use libmm::error::Result;


/// A trait, that describes an editor.
/// 
/// Provides an interface to launch editor. Implemented by default for 
/// any struct, that implements [`crate::editor::ExternalEditor`].
pub(crate) trait Editor {
    /// Runs an editor for specific note.
    /// 
    /// Fails if there is no such note or editor. Fails also in case 
    /// of failure during editor launch or when editor finishes
    /// unsuccessfully.
    /// 
    /// * `note_path` - full path to a note to edit
    fn run(&self, note_path: &Path) -> Result<()>;
}


/// A function to launch an editor.
/// 
/// Just a proxy function, that accepts an editor and calls its 
/// `run` method.
/// 
/// * `note_path` - full path to a note to edit
/// * `editor` -  an editor instance
pub(crate) fn run<E: Editor>(note_path: &Path, editor: E) -> Result<()> {
    editor.run(note_path)
}
