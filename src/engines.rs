use std::convert::{TryFrom, TryInto};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::process::Command;

use yaml_rust::YamlLoader;

use crate::error::{self, AutoTeXErr};
use crate::utils::TeXFileInfo;

// TeX engine enum
// Convert engine options to this enum so that compile TeX properly
#[derive(PartialEq, PartialOrd, Clone)]
pub enum TeXEngine {
    PdfTeX,
    XeTeX,
    LuaTeX,
    TeX,
    PdfLaTeX,
    XeLaTeX,
    LuaLaTeX,
    LaTeX,
    BibTeX,
    MakeIndex,
}

// ==================================
// Constants
// ==================================
// Default TeX Engine and its options
pub const ENGINE_OPTIONS: [&str; 5] = ["-pdf", "-xe", "-lua", "-plain", "-la"];
const ENGINES_LST: [TeXEngine; 8] = [
    TeXEngine::PdfTeX,
    TeXEngine::XeTeX,
    TeXEngine::LuaTeX,
    TeXEngine::TeX,
    TeXEngine::PdfLaTeX,
    TeXEngine::XeLaTeX,
    TeXEngine::LuaLaTeX,
    TeXEngine::LaTeX,
];

// ==================================
// Macros
// ==================================

macro_rules! impl_string {
    ($s:expr => $($e: pat),*) => {
        match $s {
            $(
                $e => stringify!($e).to_lowercase(),
            )*
        }
    }
}

macro_rules! quit_if_failed {
    ($i: ident <- $($e: expr),*) => {
        if !$i($($e,)*)? { return Ok(()); }
    }
}

// ==================================
// Implementations
// ==================================
// Implement ToString trait on a TeXEngine enum
impl ToString for TeXEngine {
    fn to_string(&self) -> String {
        use TeXEngine::*;
        impl_string! {
            *self =>
                PdfTeX, XeTeX, LuaTeX, TeX, PdfLaTeX,
            XeLaTeX, LuaLaTeX, LaTeX, BibTeX, MakeIndex
        }
    }
}

impl TryFrom<&str> for TeXEngine {
    type Error = error::AutoTeXErr;
    fn try_from(val: &str) -> error::Result<Self> {
        use TeXEngine::*;
        match val {
            "pdftex" => Ok(PdfTeX),
            "xetex" => Ok(XeTeX),
            "luatex" => Ok(LuaTeX),
            "tex" => Ok(TeX),
            "pdflatex" => Ok(PdfLaTeX),
            "xelatex" => Ok(XeLaTeX),
            "lualatex" => Ok(LuaLaTeX),
            "latex" => Ok(LaTeX),
            "bibtex" => Ok(BibTeX),
            "makeindex" => Ok(MakeIndex),
            _ => Err(AutoTeXErr::ParseErr),
        }
    }
}

impl TeXEngine {
    // Main function of compiling TeX
    pub fn run_engine(&self, tex_info: &TeXFileInfo) -> error::Result<()> {
        let mut mainfile = tex_info.mainfile.clone();
        mainfile.push(".tex");
        env::set_current_dir(&tex_info.current_dir)?;
        quit_if_failed!(compile <- self, &mainfile);
        if *self < TeXEngine::PdfLaTeX {
            quit_if_failed!(compile <- self, &mainfile);
        } else {
            match (tex_info.bibtex_exists, tex_info.mkindex_exists) {
                (false, false) => {
                    quit_if_failed!(compile <- self, &mainfile);
                }
                (true, false) => {
                    quit_if_failed!(compile <- &TeXEngine::BibTeX, &tex_info.mainfile);
                    quit_if_failed!(compile <- self, &mainfile);
                    quit_if_failed!(compile <- self, &mainfile);
                }
                (false, true) => {
                    quit_if_failed!(run_mkindex <- &tex_info);
                    quit_if_failed!(compile <- self, &mainfile);
                    quit_if_failed!(compile <- self, &mainfile);
                }
                (true, true) => {
                    quit_if_failed!(compile <- &TeXEngine::BibTeX, &tex_info.mainfile);
                    quit_if_failed!(run_mkindex <- &tex_info);
                    quit_if_failed!(compile <- self, &mainfile);
                    quit_if_failed!(compile <- self, &mainfile);
                }
            }
        }
        Ok(())
    }
}

// ==================================
// Functions
// ==================================
//TODO: Later, I will implement indivisual tex options
fn compile<S: AsRef<OsStr>>(tex: &TeXEngine, filename: &S) -> error::Result<bool> {
    Ok(Command::new(tex.to_string())
        .arg(filename)
        .status()?
        .success())
}

// TODO: run_mkindex does not run what I expected.
// I will modify it later soon.
fn run_mkindex(files: &TeXFileInfo) -> error::Result<bool> {
    let only_idx: Vec<Option<&OsStr>> = files
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
                Some(ref f) => status.push(compile(&TeXEngine::MakeIndex, f)?),
            }
        }
        Ok(status.iter().all(|x| *x))
    }
}

// Take an appropriate TeX engine from an option
pub fn take_engine(opts: &[&String]) -> error::Result<TeXEngine> {
    let mut dir = dirs::config_dir().unwrap();
    dir.push("autotex/config.yaml");
    let contents = fs::read_to_string(dir).unwrap();
    let doc = &YamlLoader::load_from_str(&contents).unwrap()[0];
    let main_engine = if doc["engine"]["main"].is_badvalue() {
        "pdftex"
    } else {
        doc["engine"]["main"].as_str().unwrap()
    };
    let main_latex_engine = if doc["engine"]["latex"].is_badvalue() {
        "pdflatex"
    } else {
        doc["engine"]["latex"].as_str().unwrap()
    };
    match opts.len() {
        0 => Ok(main_engine.try_into()?),
        1 => {
            let en = opts[0];
            if en == "-la" {
                Ok(main_latex_engine.try_into()?)
            } else {
                match ENGINE_OPTIONS.iter().position(|x| x == en) {
                    Some(n) => Ok((&ENGINES_LST[n]).clone()),
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
                        Some(n) => Ok((&ENGINES_LST[n + 4]).clone()),
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
    fn tostring_texengine() {
        assert_eq!("pdftex", TeXEngine::PdfTeX.to_string());
        assert_eq!("latex", TeXEngine::LaTeX.to_string());
        assert_eq!("makeindex", TeXEngine::MakeIndex.to_string());
    }

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
