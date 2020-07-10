use std::env;
use std::ffi::OsStr;
use std::fs;
use std::process::Command;

use yaml_rust::YamlLoader;

use crate::error::{self, AutoTeXErr};
use crate::utils::TeXFileInfo;

// Store TeX engine and some bool
// so that the program detect whether compile engine is TeX or LaTeX based
#[derive(Debug)]
pub struct TeXEngine<E> {
    engine: E,
    is_tex: bool,
}

// Default TeX Engine and its options
pub const ENGINE_OPTIONS: [&str; 5] = ["-pdf", "-xe", "-lua", "-plain", "-la"];
const ENGINES_LST: [&str; 8] = [
    "pdftex", "xetex", "luatex", "tex", "pdflatex", "xelatex", "lualatex", "latex",
];

// Every types that implemented this trait can be compiled
pub trait Compilable {
    //TODO: Later, I will implement indivisual tex options
    fn compile<S: AsRef<OsStr>>(&self, filename: &S) -> error::Result<bool>;
}

impl Compilable for &str {
    fn compile<S: AsRef<OsStr>>(&self, filename: &S) -> error::Result<bool> {
        Ok(Command::new(self).arg(filename).status()?.success())
    }
}

impl<E> Compilable for TeXEngine<E>
where
    E: Compilable,
{
    fn compile<S: AsRef<OsStr>>(&self, filename: &S) -> error::Result<bool> {
        self.engine.compile(&filename)
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
        if only_idx.is_empty() {
            Ok(true)
        } else {
            let mut status: Vec<bool> = vec![];
            for file in only_idx {
                match file {
                    None => return Err(AutoTeXErr::NoneError),
                    Some(ref f) => status.push("makeindex".compile(f)?),
                }
            }
            Ok(status.iter().all(|x| *x))
        }
    }
}

// Return a unit value if the tex engine is failed
macro_rules! quit_if_failed {
    ($e: expr; $($es: expr),*) => {
        if !$e.compile($($es,)*)? { return Ok(()); }
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
    pub fn run_engine(&self, tex_info: &TeXFileInfo) -> error::Result<()> {
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
        Ok(())
    }
}

// Read a config file and return the position of the given engine
// The config file must in at .config/autotex directory
// and its name is config.yaml
macro_rules! read_config {
    ($doc: ident taking $name: expr, $val: expr) => {
        if let Some(d) = $doc {
            if d["engine"][$name].is_badvalue() {
                $val
            } else {
                let engine = d["engine"][$name].as_str().unwrap();
                ENGINES_LST.iter().position(|&x| x == engine).unwrap()
            }
        } else {
            $val
        }
    };
}

fn read_config() -> error::Result<(usize, usize)> {
    let mut dir = dirs::config_dir().unwrap();
    dir.push("autotex/config.yaml");
    let contents = fs::read_to_string(dir).unwrap_or_default();
    let docs = YamlLoader::load_from_str(&contents)?;
    let doc = docs.get(0);
    let main_engine = read_config!(doc taking "main", 0);
    let main_latex_engine = read_config!(doc taking "latex", 4);
    Ok((main_engine, main_latex_engine))
}

// Take an appropriate TeX engine from an option
pub fn take_engine<'a>(opts: &'a [&'a String]) -> error::Result<TeXEngine<&'a str>> {
    let default = read_config()?;
    match opts.len() {
        0 => Ok(TeXEngine::new(ENGINES_LST[default.0], default.0 < 4)),
        1 => {
            let en = opts[0];
            if en == "-la" {
                Ok(TeXEngine::new(ENGINES_LST[default.1], default.1 < 4))
            } else {
                match ENGINE_OPTIONS.iter().position(|&x| x == en) {
                    Some(n) => Ok(TeXEngine::new(ENGINES_LST[n], true)),
                    None => Err(AutoTeXErr::InvalidOptionErr),
                }
            }
        }
        _ => {
            if opts.len() > 2 {
                Err(AutoTeXErr::TooManyOptionsErr)
            } else if opts.contains(&&String::from("-la")) {
                match opts.iter().find(|&x| x != &"-la") {
                    Some(en) => match ENGINE_OPTIONS.iter().position(|x| x == en) {
                        Some(n) => Ok(TeXEngine::new(ENGINES_LST[n + 4], false)),
                        None => Err(AutoTeXErr::InvalidOptionErr),
                    },
                    None => Err(AutoTeXErr::NoneError),
                }
            } else {
                Err(AutoTeXErr::DistinctTeXOptErr)
            }
        }
    }
}
