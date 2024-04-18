use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use crate::error;

const AUX_EXTENSIONS: [&str; 7] = ["aux", "log", "toc", "bbl", "blg", "lof", "out"];

pub fn remove_aux(filepath: &Path) -> error::Result<()> {
    let dir_entry = fs::read_dir(filepath)?;

    for entry in dir_entry {
        let entry = entry?;

        if AUX_EXTENSIONS
            .iter()
            .any(|s| entry.path().extension().and_then(OsStr::to_str) == Some(s))
        {
            fs::remove_file(entry.path())?;
        }
    }

    Ok(())
}
