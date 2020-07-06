use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

use yaml_rust::YamlLoader;

use crate::error::{self, AutoTeXErr};

// ============================
// Structure
// ============================
// A container of files info
#[derive(Debug)]
pub struct TeXFileInfo {
    pub filenames: Vec<PathBuf>,
    pub mainfile: OsString,
    pub current_dir: PathBuf,
    pub bibtex_exists: bool,
    pub mkindex_exists: bool,
}

// ============================
// Constants
// ============================
// TeX relative extensions
const TEX_FILES_EXTENSIONS: [&str; 4] = ["tex", "bib", "idx", "toc"];

// ============================
// Implementation
// ============================
impl TeXFileInfo {
    pub fn take_time(&self) -> error::Result<Vec<SystemTime>> {
        let mut output: Vec<SystemTime> = vec![];
        let path_lst = self.filenames.clone();
        for path in path_lst {
            output.push(path.metadata()?.modified()?);
        }
        Ok(output)
    }

    pub fn run_pdf(&self) -> error::Result<()> {
        let pdf_engine = get_pdf_viewer()?;
        let mut pdf_name = self.mainfile.clone();
        pdf_name.push(".pdf");
        Command::new(pdf_engine).arg(pdf_name).spawn()?;
        Ok(())
    }
}

// ============================
// Functions
// ============================
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
    let current_dir = if file_dir == Some(Path::new("")) {
        Path::new(".").to_path_buf()
    } else if file_dir.is_some() {
        file_dir.unwrap().to_path_buf()
    } else {
        return Err(AutoTeXErr::NoFilenameInputErr);
    };
    for path in walkdir::WalkDir::new(&current_dir) {
        match path {
            Ok(dir) => {
                // Filter out directories and files not related with TeX
                let file_ext = dir.path().extension();
                // Detect whether this is a file
                // If this is a directory, then continue the loop
                if let Some(ext) = file_ext {
                    if TEX_FILES_EXTENSIONS.iter().any(|x| OsStr::new(x) == ext) {
                        bibtex_exists = ext == "bib";
                        mkindex_exists = ext == "idx";
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

fn get_pdf_viewer() -> error::Result<PathBuf> {
    let mut config_dir = if dirs::config_dir().is_none() {
        return Err(AutoTeXErr::NoneError);
    } else {
        dirs::config_dir().unwrap()
    };
    config_dir.push("autotex/config.yaml");
    let contents = fs::read_to_string(config_dir).unwrap_or(String::new());
    let docs = YamlLoader::load_from_str(&contents)?;
    let doc = docs.get(0);
    if doc.is_none() || doc.unwrap()["pdf"].is_badvalue() {
        Ok(Path::new("xdg-open").to_path_buf())
    } else {
        let pdf_view = doc.unwrap()["pdf"].as_str().unwrap();
        Ok(Path::new(pdf_view).to_path_buf())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_yaml_in_config() {
        let mut dir = dirs::config_dir().unwrap();
        dir.push("autotex/config.yaml");
        let cont = fs::read_to_string(dir).unwrap();
        let doc = &YamlLoader::load_from_str(&cont).unwrap()[0];
        let pdf_view = doc["pdf"].as_str().unwrap();
        println!("{:?}", pdf_view);
    }
}
