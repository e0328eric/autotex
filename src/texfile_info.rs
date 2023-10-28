use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

use yaml_rust::YamlLoader;

use crate::error::{self, AutoTeXErr};

// A container of files info
#[derive(Debug)]
pub struct TeXFileInfo {
    pub filenames: Vec<PathBuf>,
    pub mainfile: OsString,
    pub current_dir: PathBuf,
    pub bibtex_exists: bool,
    pub mkindex_exists: bool,
    pub asymptote_exists: bool,
}

// TeX relative extensions
const TEX_FILES_EXTENSIONS: [&str; 4] = ["tex", "bib", "idx", "toc"];

// Implementation of TeXFileInfo
impl TeXFileInfo {
    fn new() -> Self {
        Self {
            filenames: vec![],
            mainfile: OsString::new(),
            current_dir: Path::new("").to_path_buf(),
            bibtex_exists: false,
            mkindex_exists: false,
            asymptote_exists: false,
        }
    }

    pub fn take_time(&self) -> error::Result<Vec<SystemTime>> {
        let mut output: Vec<SystemTime> = vec![];
        let path_lst = self.filenames.clone();
        for path in path_lst {
            output.push(path.metadata()?.modified()?);
        }
        Ok(output)
    }

    #[cfg(not(target_os = "windows"))]
    pub fn run_pdf(&self) -> error::Result<()> {
        let pdf_engine = get_pdf_viewer()?;
        let mut pdf_name = self.mainfile.clone();
        pdf_name.push(".pdf");
        Command::new(pdf_engine).arg(pdf_name).spawn()?;
        Ok(())
    }
    
    // TODO: Implement get_odf_viewer for windows
    #[cfg(target_os = "windows")]
    pub fn run_pdf(&self) -> error::Result<()> {
        Ok(())
    }

    pub fn get_main_tex_file(&self) -> String {
        [
            self.mainfile
                .to_str()
                .expect("Cannot take a filename to compile"),
            ".tex",
        ]
        .concat()
    }

    pub fn get_main_pdf_file(&self) -> String {
        [
            self.mainfile
                .to_str()
                .expect("Cannot take a filename to compile"),
            ".pdf",
        ]
        .concat()
    }
}

// Take all tex related files in the current directory
pub fn get_files_info(filepath: &Path) -> error::Result<TeXFileInfo> {
    let mut output = TeXFileInfo::new();
    output.mainfile = if let Some(file) = filepath.file_stem() {
        file.to_os_string()
    } else {
        return Err(AutoTeXErr::NoFilenameInputErr);
    };

    let file_dir = filepath.ancestors().nth(1);
    output.current_dir = if file_dir == Some(Path::new("")) {
        Path::new(".").to_path_buf()
    } else if file_dir.is_some() {
        file_dir.unwrap().to_path_buf()
    } else {
        return Err(AutoTeXErr::NoFilenameInputErr);
    };

    for path in walkdir::WalkDir::new(&output.current_dir) {
        match path {
            Ok(dir) => {
                // Filter out directories and files not related with TeX
                let file_ext = dir.path().extension();
                // Detect whether this is a file
                // If this is a directory, then continue the loop
                if let Some(ext) = file_ext {
                    if TEX_FILES_EXTENSIONS.iter().any(|x| OsStr::new(x) == ext) {
                        if !output.bibtex_exists {
                            output.bibtex_exists = ext == "bib";
                        }
                        if !output.mkindex_exists {
                            output.mkindex_exists = ext == "idx";
                        }
                        output.filenames.push(dir.into_path());
                    }
                }
            }
            Err(_) => return Err(AutoTeXErr::TakeFilesErr),
        }
    }
    output.filenames.sort();
    Ok(output)
}

// Get a pdf viewer from a config file
// The config file must in at .config/autotex directory
// and its name is config.yaml
#[cfg(target_os = "linux")]
const DEFAULT_PDF_VIEW: &str = "xdg-open";

#[cfg(target_os = "macos")]
const DEFAULT_PDF_VIEW: &str = "open";

#[cfg(not(target_os = "windows"))]
fn get_pdf_viewer() -> error::Result<PathBuf> {
    let mut config_dir = if dirs::config_dir().is_none() {
        return Err(AutoTeXErr::NoneError);
    } else {
        dirs::config_dir().unwrap()
    };
    config_dir.push("autotex/config.yaml");
    let contents = fs::read_to_string(config_dir).unwrap_or_default();
    let docs = YamlLoader::load_from_str(&contents)?;
    let doc = docs.get(0);
    if let Some(d) = doc {
        if d["pdf"].is_badvalue() {
            Ok(Path::new(DEFAULT_PDF_VIEW).to_path_buf())
        } else {
            let pdf_view = d["pdf"].as_str().unwrap();
            Ok(Path::new(pdf_view).to_path_buf())
        }
    } else {
        Ok(Path::new(DEFAULT_PDF_VIEW).to_path_buf())
    }
}

// TODO: Implement get_odf_viewer for windows
#[cfg(target_os = "windows")]
fn get_pdf_viewer() {
    eprintln!("[NOTE]: get_pdf_viewer is not supported yet in Windows");
}
