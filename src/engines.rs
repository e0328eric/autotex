use crate::compilable::Compilable;
use crate::error::{self, AutoTeXErr};
use crate::utils::TeXFileInfo;
use std::env;
use std::ffi::OsStr;

// Return a unit value if the tex engine is failed
macro_rules! quit_if_failed {
    ($e: expr; $($es: expr),*) => {
        if !$e.compile($($es,)*)? { return Ok(false); }
    }
}

// Store TeX engine and some bool
// so that the program detect whether compile engine is TeX or LaTeX based
#[derive(Debug)]
pub struct TeXEngine<E: Compilable> {
    engine: E,
    is_tex: bool,
}

impl<E> Compilable for TeXEngine<E>
where
    E: Compilable,
{
    fn compile<S: AsRef<OsStr>>(&self, filename: &S) -> error::Result<bool> {
        self.engine.compile(&filename)
    }
}

impl<E> TeXEngine<E>
where
    E: Compilable,
{
    fn new(engine: E, is_tex: bool) -> Self {
        Self { engine, is_tex }
    }

    // Main function of compiling TeX
    pub fn run_engine(&self, tex_info: &TeXFileInfo) -> error::Result<bool> {
        let mut mainfile = tex_info.mainfile.clone();
        mainfile.push(".tex");
        env::set_current_dir(&tex_info.current_dir)?;
        quit_if_failed!(self; &mainfile);
        if self.is_tex {
            quit_if_failed!(self; &mainfile);
        } else {
            match (tex_info.bibtex_exists, tex_info.mkindex_exists) {
                (false, false) => {
                    quit_if_failed!(self; &mainfile);
                }
                (true, false) => {
                    quit_if_failed!("bibtex"; &tex_info.mainfile);
                    quit_if_failed!(self; &mainfile);
                    quit_if_failed!(self; &mainfile);
                }
                (false, true) => {
                    quit_if_failed!(&tex_info; &"");
                    quit_if_failed!(self; &mainfile);
                    quit_if_failed!(self; &mainfile);
                }
                (true, true) => {
                    quit_if_failed!("bibtex"; &tex_info.mainfile);
                    quit_if_failed!(&tex_info; &"");
                    quit_if_failed!(self; &mainfile);
                    quit_if_failed!(self; &mainfile);
                }
            }
        }
        Ok(true)
    }
}

// Take an appropriate TeX engine from an option
pub fn take_engine(engine: &str) -> error::Result<TeXEngine<String>> {
    match engine {
        "pdftex" | "xetex" | "luatex" | "tex" | "plaintex" => {
            Ok(TeXEngine::new(engine.to_string(), true))
        }
        "pdflatex" | "xelatex" | "lualatex" | "latex" | "plainlatex" => {
            Ok(TeXEngine::new(engine.to_string(), false))
        }
        _ => Err(AutoTeXErr::InvalidOptionErr),
    }
}
