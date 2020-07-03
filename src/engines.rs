use std::env;
use std::ffi::OsStr;
use std::fs;
use std::process::Command;

use yaml_rust::YamlLoader;

use crate::error::{self, AutoTeXErr};
use crate::utils::TeXFileInfo;

// TeX engine enum
// Convert engine options to this enum so that compile TeX properly
#[derive(Clone)]
pub struct TeXEngine {
    engine: String,
    is_tex: bool,
}

// ==================================
// Constants
// ==================================
// Default TeX Engine and its options
pub const ENGINE_OPTIONS: [&str; 5] = ["-pdf", "-xe", "-lua", "-plain", "-la"];
const ENGINES_LST: [&str; 8] = [
    "pdftex", "xetex", "luatex", "tex", "pdflatex", "xelatex", "lualatex", "latex",
];

// ==================================
// Macros
// ==================================
macro_rules! quit_if_failed {
    ($e: expr; $($es: expr),*) => {
        if !$e.compile($($es,)*)? { return Ok(()); }
    }
}

// ==================================
// Trait
// ==================================
trait Compilable {
    //TODO: Later, I will implement indivisual tex options
    fn compile<S: AsRef<OsStr>>(&self, filename: &S) -> error::Result<bool>;
}

// ==================================
// Implementations
// ==================================
// Implementation of Compilable trait for several types
impl Compilable for &str {
    fn compile<S: AsRef<OsStr>>(&self, filename: &S) -> error::Result<bool> {
        Ok(Command::new(self.to_string())
            .arg(filename)
            .status()?
            .success())
    }
}

impl Compilable for TeXEngine {
    fn compile<S: AsRef<OsStr>>(&self, filename: &S) -> error::Result<bool> {
        Ok(Command::new(&self.engine).arg(filename).status()?.success())
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

impl TeXEngine {
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

// ==================================
// Functions
// ==================================
// Read a config file
fn read_config() -> error::Result<(String, String)> {
    let mut dir = dirs::config_dir().unwrap();
    dir.push("autotex/config.yaml");
    let contents = fs::read_to_string(dir).unwrap_or(String::new());
    let docs = YamlLoader::load_from_str(&contents)?;
    let doc = docs.get(0);
    let main_engine = if doc.is_none() || doc.unwrap()["engine"]["main"].is_badvalue() {
        "pdftex"
    } else {
        doc.unwrap()["engine"]["main"].as_str().unwrap()
    };
    let main_latex_engine = if doc.is_none() || doc.unwrap()["engine"]["latex"].is_badvalue() {
        "pdflatex"
    } else {
        doc.unwrap()["engine"]["latex"].as_str().unwrap()
    };
    Ok((main_engine.to_string(), main_latex_engine.to_string()))
}

// Take an appropriate TeX engine from an option
pub fn take_engine(opts: &[&String]) -> error::Result<TeXEngine> {
    match opts.len() {
        0 => Ok(TeXEngine {
            engine: read_config()?.0,
            is_tex: true,
        }),
        1 => {
            let en = opts[0];
            if en == "-la" {
                Ok(TeXEngine {
                    engine: read_config()?.1,
                    is_tex: false,
                })
            } else {
                match ENGINE_OPTIONS.iter().position(|&x| x == en) {
                    Some(n) => Ok(TeXEngine {
                        engine: (&ENGINES_LST[n]).to_string(),
                        is_tex: true,
                    }),
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
                        Some(n) => Ok(TeXEngine {
                            engine: (&ENGINES_LST[n + 4]).to_string(),
                            is_tex: false,
                        }),
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

// Testing
#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn parse_yaml_in_config() {
        let mut dir = dirs::config_dir().unwrap();
        dir.push("autotex/config.yaml");
        let cont = fs::read_to_string(dir).unwrap();
        let doc = &YamlLoader::load_from_str(&cont).unwrap()[0];
        let main_engine = doc["engine"]["main"].as_str().unwrap();
        println!("{:?}", main_engine);
    }
}
