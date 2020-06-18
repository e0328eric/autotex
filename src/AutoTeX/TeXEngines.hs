module AutoTeX.TeXEngines where

import Data.Char (toLower)
import Data.List (elemIndex)
import System.Directory (setCurrentDirectory)
import System.Exit
import System.FilePath.Posix ((-<.>))
import System.Process (rawSystem)

import AutoTeX.Utils

-- Engine Options
texOptions = ["-pdf", "-xe", "-lua", "-plain"]

latexOptions = ["-la"]

engineOptions = texOptions ++ latexOptions

options = engineOptions ++ ["-cd", "-v"]

-- =============================================================================
--  TeX Engine Functions
--
--  There are Eight possibilities to run this program
--  If -la option is added, we can use latex options
-- =============================================================================
data TeXEngine
    = PdfTeX
    | XeTeX
    | LuaTeX
    | TeX
    | PdfLaTeX
    | XeLaTeX
    | LuaLaTeX
    | LaTeX
    | BibTeX
    | MakeIndex
    deriving (Show, Eq, Ord, Enum)

compile :: TeXEngine -> [String] -> IO ExitCode
compile c = rawSystem $ toLower <$> show c

takeEngine :: [String] -> Either String TeXEngine
takeEngine [] = Right PdfTeX
takeEngine [en]
    | en == "-la" = Right PdfLaTeX
    | otherwise =
        case elemIndex en engineOptions of
            Nothing -> Left "No tex option is used!"
            Just n -> Right $ [PdfTeX .. LaTeX] !! n
takeEngine lst
    | length lst > 2 = Left "Too many options!"
    | "-la" `elem` lst =
        case elemIndex removeLa engineOptions of
            Nothing -> Left "No tex option is used"
            Just n -> Right $ [PdfTeX .. LaTeX] !! (n + 4)
    | otherwise = Left "Cannot use two distinct TeX options!"
  where
    removeLa = head $ filter (/= "-la") lst

runMakeIndex :: [FilePath] -> IO ExitCode
runMakeIndex topdirs =
    case topdirs of
        [] -> return ExitSuccess
        (f:xf) -> do
            setCurrentDirectory $ getDir f
            compile MakeIndex [getFileName f]
            runMakeIndex xf

runEngine' :: TeXEngine -> FilePath -> [String] -> IO ExitCode
runEngine' engine fp options = do
    successOrFail <- head execEngineLst
    bibindex <- isBibOrIndexExists fp
    if successOrFail /= ExitSuccess
        then return ExitSuccess
        else if engine < PdfLaTeX
                 then head execEngineLst
                 else case bibindex of
                          NoneUsed -> head execEngineLst
                          BibTeXUsed ->
                              sequence (execBibTeXLst ++ execEngineLst) >>=
                              quitIfFailed
                          MKIndUsed ->
                              fp ?</> ".idx" >>= \lst ->
                                  sequence (execMKIndex lst ++ execEngineLst) >>=
                                  quitIfFailed
                          BothUsed ->
                              fp ?</> ".idx" >>= \lst ->
                                  sequence
                                      (execBibTeXLst ++
                                       execMKIndex lst ++ execEngineLst) >>=
                                  quitIfFailed
  where
    texFileName = getFileName fp
    execEngineLst = replicate 2 $ compile engine $ texFileName : options
    execBibTeXLst = [compile BibTeX [texFileName -<.> ""]]
    execMKIndex lst = [compile MakeIndex lst]

runEngine'' ::
       LocationOP
    -> Either String TeXEngine
    -> FilePath
    -> [String]
    -> IO ExitCode
runEngine'' _ (Left str) _ _ = rawSystem "echo" [str]
runEngine'' CurrentRun (Right engine) fp options = runEngine' engine fp options
runEngine'' DiffLoctRun (Right engine) fp options =
    setCurrentDirectory fl >> runEngine' engine fp options
  where
    fl = getDir fp
    texfilename = getFileName fp
