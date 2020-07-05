// The help string
// This will displayed when the option "--help"
pub const HELP_STRING: &str = "Usage:\n
autotex [-v] [-la] [-pdf, -plain, -xe, -lua] FILENAME\n
Options:\n
-pdf : Execute pdfTeX (-la -pdf : Execute pdfLaTeX)
-xe : Execute XeTeX (-la -xe : Execute XeLaTeX)
-lua : Execute LuaTeX (-la -lua : Execute LuaLaTeX)
-plain : Execute TeX (-la -plain : Execute LaTeX)
-v : Open pdf and return the continuous mode
";
