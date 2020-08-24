use crate::error::{self, AutoTeXErr};
use clap::{App, Arg};
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use yaml_rust::YamlLoader;

// Default TeX Engine and its options
pub const TEX_ENGINES: [&str; 5] = ["pdftex", "xetex", "luatex", "tex", "plaintex"];
pub const LATEX_ENGINES: [&str; 5] = ["pdflatex", "xelatex", "lualatex", "latex", "plainlatex"];

// Read a config file and return the position of the given engine
// The config file must in at .config/autotex directory
// and its name is config.yaml
fn read_config() -> error::Result<String> {
    let mut dir = dirs::config_dir().unwrap();
    dir.push("autotex/config.yaml");
    let contents = fs::read_to_string(dir).unwrap_or_default();
    let docs = YamlLoader::load_from_str(&contents)?;
    let doc = docs.get(0);
    let main_engine = if let Some(d) = doc {
        if d["engine"]["main"].is_badvalue() {
            "pdftex"
        } else {
            d["engine"]["main"].as_str().unwrap()
        }
    } else {
        "pdftex"
    };
    Ok(main_engine.to_lowercase().to_string())
}

#[derive(Debug, PartialEq)]
pub struct AutoTeXCommand {
    pub file_path: PathBuf,
    pub tex_engine: String,
    pub is_conti_compile: bool,
    pub is_view: bool,
}

impl AutoTeXCommand {
    pub fn new() -> error::Result<Self> {
        Self::new_from(std::env::args_os().into_iter())
    }

    fn new_from<I, T>(args: I) -> error::Result<Self>
    where
        I: Iterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let default = read_config()?;
        let default_engine = if TEX_ENGINES.contains(&default.as_str())
            || LATEX_ENGINES.contains(&default.as_str())
        {
            default
        } else {
            return Err(AutoTeXErr::InvalidOptionErr);
        };

        // Basic app information
        let app = App::new("autotex")
            .version("1.0.0")
            .about("Compiles TeX or LaTeX continuously")
            .author("Sungbae Jeong");

        // Define the view command line option
        let view_option = Arg::with_name("view")
            .long("view")
            .short("v")
            .help("View pdf for given compiled TeX file");

        // Whether compile automatically
        let auto_compile = Arg::with_name("autoCompile")
            .long("conti")
            .short("c")
            .help("Compile TeX automatically");

        // Take filepath
        let input_filepath = Arg::with_name("INPUT")
            .required(true)
            .index(1)
            .help("Sets the input filename or filepath to use");

        // Declare which engines to compile
        let engine_option = Arg::with_name("engine")
            .long("engine")
            .short("e")
            .takes_value(true)
            .number_of_values(1)
            .default_value(&default_engine)
            .help("Declare the TeX engine to compile");

        // Extract the matches
        let matches = app
            .arg(view_option)
            .arg(auto_compile)
            .arg(input_filepath)
            .arg(engine_option)
            .get_matches_from(args);

        let file_path = PathBuf::from(matches.value_of("INPUT").unwrap());
        let tex_engine = String::from(matches.value_of("engine").unwrap().to_lowercase());
        let is_conti_compile = matches.occurrences_of("autoCompile") > 0;
        let is_view = matches.occurrences_of("view") > 0;

        Ok(Self {
            file_path,
            tex_engine,
            is_conti_compile,
            is_view,
        })
    }
}
