module AutoTeX.Help where

helpString :: String
helpString = unlines
  [
    "Usage:"
  , ""
  , "autotex [-cd] [-v] [-la] [-pdf, -plain, -xe, -lua] FILE.tex"
  , ""
  , "Options:"
  , ""
  , "-pdf : Execute pdfTeX (-la -pdf : Execute pdfLaTeX)"
  , "-xe : Execute XeTeX (-la -xe : Execute XeLaTeX)"
  , "-lua : Execute LuaTeX (-la -lua : Execute LuaLaTeX)"
  , "-plain : Execute TeX (-la -plain : Execute LaTeX)"
  , "-v : Open pdf and return the continuous mode"
  , "-cd : Compile .tex file at the subdirectory"
  , ""
  , "Especially, -pdf option can be omitted. (-pdf is the default option)"
  ]
