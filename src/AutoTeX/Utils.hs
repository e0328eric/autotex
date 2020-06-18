module AutoTeX.Utils where

import Control.Monad (forM)
import qualified Data.List as L
import Data.Time.Clock (UTCTime)
import System.Directory
    ( doesDirectoryExist
    , getCurrentDirectory
    , getDirectoryContents
    , getModificationTime
    , setCurrentDirectory
    )
import System.Exit
import System.FilePath.Posix ((</>), splitFileName)

data LocationOP
    = CurrentRun
    | DiffLoctRun
    deriving (Eq, Ord, Enum)

data ViewTF
    = OnlyCompile
    | ViewAndCompile
    deriving (Eq, Ord, Enum)

data BibOrIndexUsed
    = NoneUsed
    | BibTeXUsed
    | MKIndUsed
    | BothUsed
    deriving (Eq, Ord, Enum)

-- basic but must need functions
takePureName = reverse . drop 4 . reverse

getDir = fst . splitFileName

getFileName = snd . splitFileName

-- Quit the program if Exitfail signal is detected
quitIfFailed :: [ExitCode] -> IO ExitCode
quitIfFailed [] = return ExitSuccess
quitIfFailed (x:xs)
    | x == ExitSuccess = quitIfFailed xs
    | otherwise = putStrLn "End Compiling" >> return ExitSuccess

-- ==============================================================================
--                                   File Manager
--
--              Check the file is Modified or Specific file exists
-- ==============================================================================
-- Take all files in current directory
getRecursiveContents' :: FilePath -> IO [FilePath]
getRecursiveContents' topdir = do
    names <- getDirectoryContents topdir
    let properNames = filter (`notElem` [".", ".."]) names
    paths <-
        forM properNames $ \name -> do
            let path = topdir </> name
            isDirectory <- doesDirectoryExist path
            if isDirectory
                then getRecursiveContents' path
                else return [path]
    return (concat paths)

getRecursiveContents = getRecursiveContents' . getDir

-- Get recent modified time for each files in the current location
getFileTimes :: LocationOP -> FilePath -> IO [UTCTime]
getFileTimes CurrentRun topdir =
    getRecursiveContents topdir >>= mapM getModificationTime
getFileTimes DiffLoctRun topdir = do
    fl <- setCurrentDirectory topdir >> getCurrentDirectory
    getRecursiveContents fl >>= mapM getModificationTime

-- Check whether .bib or .idx file exists
infix 6 ?</>

(?</>) :: FilePath -> String -> IO [FilePath]
topdir ?</> str = do
    tmplst <- getRecursiveContents topdir
    return $ filter (L.isSubsequenceOf str) tmplst

isBibOrIndexExists :: FilePath -> IO BibOrIndexUsed
isBibOrIndexExists topdir = do
    lst1 <- topdir ?</> ".bib"
    lst2 <- topdir ?</> ".idx"
    return $
        [NoneUsed .. BothUsed] !!
        (fromEnum (not . null $ lst1) + 2 * fromEnum (not . null $ lst2))
