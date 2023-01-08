use std::path::Path;

use super::xi;
use super::external;
use super::ExternalEditor;
use crate::error::Result;


/// Runs an editor for specific note.
/// 
/// Fails if there is no such note or editor. Fails also in case 
/// of failure during editor launch or when editor finishes
/// unsuccessfully.
/// 
/// * `note_path` - full path to a note to edit
/// * `editor` - optional editor wrapper (see [`crate::editor::ExternalEditor`]). 
///              Pass `None` to run built-in xi-based one.
pub(crate) fn run_editor<E: ExternalEditor>(note_path: &Path, editor: Option<E>) -> Result<()> {
    //
    // Launch an external editor if passed or run the built-in one
    //

    match editor {
        Some(editor) => external::run_editor(note_path, editor),
        None => xi::run_editor(note_path)
    }
}
