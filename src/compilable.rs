use crate::error::{self, AutoTeXErr};
use crate::texfile_info::TeXFileInfo;
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
        let only_idx = self
            .filenames
            .iter()
            .filter(|x| x.extension() == Some(OsStr::new("idx")))
            .map(|x| x.file_name());
        let idx_compiled = if only_idx.clone().count() == 0 {
            true
        } else {
            let mut status = true;
            for file in only_idx {
                match file {
                    None => return Err(AutoTeXErr::NoneError),
                    Some(ref f) => status = status && "makeindex".compile(f)?,
                }
            }
            status
        };

        let only_asy = self
            .filenames
            .iter()
            .filter(|x| x.extension() == Some(OsStr::new("asy")))
            .map(|x| x.file_name());
        let asy_compiled = if only_asy.clone().count() == 0 {
            true
        } else {
            let mut status = true;
            for file in only_asy {
                match file {
                    None => return Err(AutoTeXErr::NoneError),
                    Some(ref f) => status = status && "asy".compile(f)?,
                }
            }
            status
        };
        Ok(idx_compiled && asy_compiled)
    }
}
