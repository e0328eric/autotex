use crate::error::{self, AutoTeXErr};
use crate::utils::TeXFileInfo;
use std::ffi::OsStr;
use std::process::Command;

// Every types that implemented this trait can be compiled
pub trait Compilable {
    //TODO: Later, I will implement indivisual tex options
    fn compile<S: AsRef<OsStr>>(&self, filename: &S) -> error::Result<bool>;
}

// Some types that are compilable
impl Compilable for &str {
    fn compile<S: AsRef<OsStr>>(&self, filename: &S) -> error::Result<bool> {
        Ok(Command::new(self).arg(filename).status()?.success())
    }
}

impl Compilable for String {
    fn compile<S: AsRef<OsStr>>(&self, filename: &S) -> error::Result<bool> {
        self.as_str().compile(&filename)
    }
}

impl Compilable for TeXFileInfo {
    fn compile<S: AsRef<OsStr>>(&self, _filename: &S) -> error::Result<bool> {
        let only_idx: Vec<Option<&OsStr>> = self
            .filenames
            .iter()
            .filter(|x| x.extension() == Some(OsStr::new("idx")))
            .map(|x| x.file_name())
            .collect();
        let idx_compiled = if only_idx.is_empty() {
            true
        } else {
            let mut status: Vec<bool> = vec![];
            for file in only_idx {
                match file {
                    None => return Err(AutoTeXErr::NoneError),
                    Some(ref f) => status.push("makeindex".compile(f)?),
                }
            }
            status.iter().all(|x| *x)
        };

        let only_asy: Vec<Option<&OsStr>> = self
            .filenames
            .iter()
            .filter(|x| x.extension() == Some(OsStr::new("asy")))
            .map(|x| x.file_name())
            .collect();
        let asy_compiled = if only_asy.is_empty() {
            true
        } else {
            let mut status: Vec<bool> = vec![];
            for file in only_asy {
                match file {
                    None => return Err(AutoTeXErr::NoneError),
                    Some(ref f) => status.push("asy".compile(f)?),
                }
            }
            status.iter().all(|x| *x)
        };
        Ok(idx_compiled && asy_compiled)
    }
}
