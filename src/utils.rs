extern crate walkdir;

use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

use crate::error::{self, AutoTeXErr};

// A container of files info
#[derive(Debug)]
pub struct TeXFileInfo {
    pub filenames: Vec<PathBuf>,
    pub mainfile: OsString,
    pub current_dir: PathBuf,
    pub bibtex_exists: bool,
    pub mkindex_exists: bool,
}

// TeX relative extensions
const TEX_FILES_EXTENSIONS: [&str; 4] = ["tex", "bib", "idx", "toc"];

// Take all tex related files in the current directory
pub fn get_files_info(filepath: &PathBuf) -> error::Result<TeXFileInfo> {
    let mut filenames: Vec<PathBuf> = Vec::new();
    let mut bibtex_exists = false;
    let mut mkindex_exists = false;
    let mainfile = if let Some(file) = filepath.file_stem() {
        file.to_os_string()
    } else {
        return Err(AutoTeXErr::NoFilenameInputErr);
    };
    let file_dir = filepath.ancestors().nth(1);
    let default_dir = Path::new(".").to_path_buf();
    let current_dir = if file_dir == Some(Path::new("")) {
        default_dir
    } else if let Some(_) = file_dir {
        file_dir.unwrap().to_path_buf()
    } else {
        return Err(AutoTeXErr::NoFilenameInputErr);
    };
    let iter = walkdir::WalkDir::new(&current_dir).into_iter();
    for path in iter {
        match path {
            Ok(dir) => {
                // Filter out directories and files not related with TeX
                let file_ext = dir.path().extension();
                if let Some(ext) = file_ext {
                    if TEX_FILES_EXTENSIONS.iter().any(|x| OsStr::new(x) == ext) {
                        if ext == "bib" {
                            bibtex_exists = true;
                        }
                        if ext == "idx" {
                            mkindex_exists = true;
                        }
                        filenames.push(dir.into_path());
                    }
                }
            }
            Err(_) => return Err(AutoTeXErr::TakeFilesErr),
        }
    }
    filenames.sort();
    Ok(TeXFileInfo {
        filenames,
        mainfile,
        current_dir,
        bibtex_exists,
        mkindex_exists,
    })
}

pub fn take_time(file_info: &TeXFileInfo) -> error::Result<Vec<SystemTime>> {
    let mut output: Vec<SystemTime> = vec![];
    let path_lst = file_info.filenames.clone();
    for path in path_lst {
        output.push(path.metadata()?.modified()?);
    }
    Ok(output)
}

fn get_pdf_viewer() -> error::Result<PathBuf> {
    let mut home_dir = if let None = dirs::home_dir() {
        return Err(AutoTeXErr::NoneError);
    } else {
        dirs::home_dir().unwrap()
    };
    home_dir.push(".autotexrc");
    let mut contents = fs::read_to_string(home_dir)?;
    contents.pop();
    if contents.is_empty() {
        Ok(Path::new("xdg-open").to_path_buf())
    } else {
        Ok(Path::new(&contents).to_path_buf())
    }
}

pub fn run_pdf(tex: &TeXFileInfo) -> error::Result<()> {
    let pdf_engine = get_pdf_viewer()?;
    let mut pdf_name = tex.mainfile.clone();
    pdf_name.push(".pdf");
    Command::new(pdf_engine).arg(pdf_name).spawn()?;
    Ok(())
}
