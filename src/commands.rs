use crate::error::{self, AutoTeXErr};
use clap::{Arg, Command};
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use yaml_rust::YamlLoader;

// Default TeX Engine and its options
pub const TEX_ENGINES: [&str; 5] = ["pdftex", "xetex", "luatex", "tex", "plaintex"];
pub const LATEX_ENGINES: [&str; 5] = ["pdflatex", "xelatex", "lualatex", "latex", "plainlatex"];

// Make macros to define options and variables easily
// These macros use only defining tex engine related options and variables in here
macro_rules! define_tex_engine_var {
    ($varname: ident := $matches: expr, $name: expr, $engine: expr) => {
        let $varname = if $matches.get_count($name) > 0 {
            $engine
        } else {
            ""
        };
    };
}

macro_rules! define_tex_engine_option {
    ($optname: ident := $argname: expr, $short: expr, $conflit: expr, $help: expr) => {
        let $optname = Arg::new($argname)
            .short($short)
            .conflicts_with_all($conflit)
            .help($help);
    };
}

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
    Ok(main_engine.to_lowercase())
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
        Self::new_from(std::env::args_os())
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
        let app = Command::new("autotex")
            .version(env!("CARGO_PKG_VERSION"))
            .about("Compiles TeX or LaTeX continuously")
            .author("Sungbae Jeong");

        // Define the view command line option
        let view_option = Arg::new("view")
            .long("view")
            .short('v')
            .help("View pdf for given compiled TeX file");

        // Whether compile automatically
        let auto_compile = Arg::new("autoCompile")
            .long("conti")
            .short('c')
            .help("Compile TeX automatically");

        // Take filepath
        let input_filepath = Arg::new("INPUT")
            .required(true)
            .index(1)
            .help("Sets the input filename or filepath to use");

        // Declare which engines to compile
        let engine_option = Arg::new("ENGINE")
            .long("engine")
            .short('e')
            .conflicts_with_all(["pdftex", "xetex", "luatex", "tex", "latex"])
            .num_args(1)
            .number_of_values(1)
            .help("Declare the TeX engine to compile");

        define_tex_engine_option!(pdftex := "pdftex", 'p', &["xetex", "luatex", "tex"],
            "Compile with pdftex, can be combined with -L"
        );
        define_tex_engine_option!(xetex := "xetex", 'x', &["pdftex", "luatex", "tex"],
            "Compile with xetex, can be combined with -L"
        );
        define_tex_engine_option!(luatex := "luatex", 'l', &["pdftex", "xetex", "tex"],
            "Compile with luatex, can be combined with -L"
        );
        define_tex_engine_option!(tex := "tex", 't', &["pdftex", "xetex", "luatex", "latex"],
            "Compile with tex"
        );
        define_tex_engine_option!(latex := "latex", 'L', &["tex"],
            "Compile with latex"
        );

        // Extract the matches
        let matches = app
            .args(&[
                view_option,
                auto_compile,
                input_filepath,
                engine_option,
                pdftex,
                xetex,
                luatex,
                tex,
                latex,
            ])
            .get_matches_from(args);

        define_tex_engine_var!(use_pdftex := matches, "pdftex", "pdf");
        define_tex_engine_var!(use_xetex := matches, "xetex", "xe");
        define_tex_engine_var!(use_luatex := matches, "luatex", "lua");
        define_tex_engine_var!(use_latex := matches, "latex", "la");

        let file_path = PathBuf::from(matches.get_one::<&str>("INPUT").unwrap());
        let tex_engine = if matches.get_count("ENGINE") == 0 {
            let engine = use_pdftex.to_string() + use_xetex + use_luatex + use_latex + "tex";
            if matches.get_count("tex") == 0 && &engine == "tex" {
                default_engine
            } else {
                engine
            }
        } else {
            matches.get_one::<String>("ENGINE").unwrap().to_lowercase()
        };
        let is_conti_compile = matches.get_count("autoCompile") > 0;
        let is_view = matches.get_count("view") > 0;

        Ok(Self {
            file_path,
            tex_engine,
            is_conti_compile,
            is_view,
        })
    }
}
