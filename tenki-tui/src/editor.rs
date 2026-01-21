use std::env;
use std::io;
use std::path::Path;
use std::process::Command;

/// Opens the given file in the user's preferred editor.
/// Falls back to common editors if $EDITOR is not set.
pub fn open_in_editor(path: &Path) -> io::Result<()> {
    let editor = env::var("EDITOR")
        .or_else(|_| env::var("VISUAL"))
        .unwrap_or_else(|_| detect_editor());

    let status = Command::new(&editor).arg(path).status()?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Editor '{}' exited with non-zero status", editor),
        ));
    }

    Ok(())
}

/// Try to detect an available editor on the system.
fn detect_editor() -> String {
    let editors = ["nvim", "vim", "vi", "nano", "code", "emacs"];

    for editor in editors {
        if Command::new("which")
            .arg(editor)
            .output()
            .is_ok_and(|o| o.status.success())
        {
            return editor.to_string();
        }
    }

    // Last resort fallback
    "vi".to_string()
}
