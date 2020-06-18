{-# LANGUAGE LambdaCase #-}

module Main where

import Control.Concurrent (threadDelay)
import Control.Concurrent.MVar (MVar, newEmptyMVar, putMVar, tryTakeMVar)
import Control.Monad (void, when)
import qualified Data.List as L
import Data.Maybe (fromMaybe)
import Data.Time.Clock (UTCTime)
import Safe (headMay)
import System.Directory (doesFileExist, getCurrentDirectory, getHomeDirectory)
import System.Environment (getArgs)
import System.Exit
import System.FilePath.Posix ((-<.>))
import System.Posix.Signals (Handler(Catch), installHandler, sigINT)
import System.Process (callCommand)

import AutoTeX.Help
import AutoTeX.TeXEngines
import AutoTeX.Utils

-- ==============================================================================
--                                   Options
--
--         Option lists and filtering the option part and filename part
-- ==============================================================================
-- Check whether given input is option
type Options = [String]

isOption :: String -> Bool
isOption str = or $ optionLst <*> [str]

optionLst :: [String -> Bool]
optionLst =
    flip elem engineOptions : (L.isSubsequenceOf <$> ["--texop", "-cd", "-v"])

isHelpOption = L.isSubsequenceOf "--help"

-- Take --texop={blabla} to blabla.
makeInputOp' :: Int -> (String -> Bool) -> [String] -> [String]
makeInputOp' n fnt = (map . drop $ n) . filter fnt

makeTeXInputOp = makeInputOp' 8 $ optionLst !! 1

-- =================================================================================
--                                   Main Part
--
--                     This is the main part of this program
-- =================================================================================
-- Check whether the option "-cd" is enabled
isCd :: Options -> LocationOP
isCd = toEnum . fromEnum . elem "-cd"

-- Check whether the option "-v" is enabled
isView :: Options -> ViewTF
isView = toEnum . fromEnum . elem "-v"

-- Run the engine
runEngine :: Options -> FilePath -> IO ExitCode
runEngine args fp = runEngine'' (isCd args) engineUse fp texOptions
  where
    texOptions = makeTeXInputOp args
    engineOptions = filter (head optionLst) args
    engineUse = takeEngine engineOptions -- Yet we are using only tex options. So we can write this

getPdfViewer :: IO String
getPdfViewer = do
    home <- getHomeDirectory
    isExist <- doesFileExist $ home ++ ".autotexrc"
    if isExist
        then do
            fileLine' <- readFile $ home ++ ".autotexrc"
            let fileLine = headMay $ filter parseView (lines fileLine')
            return . dropPdfViewer . takePdfEngine $ fileLine
        else return "xdg-open"
  where
    dropPdfViewer str =
        case L.elemIndex ':' str of
            Just n -> drop (n + 1) str
            Nothing -> "xdg-open"
    parseView x =
        L.isSubsequenceOf "pdfview:" x || L.isSubsequenceOf "pdfview :" x
    takePdfEngine = fromMaybe "xdg-open"

runPdf :: String -> FilePath -> IO ()
runPdf str fp = callCommand progToExec
  where
    fnTMP = getFileName fp
    fn = fnTMP -<.> "pdf"
    progToExec = str ++ " " ++ fn ++ "&"

intHandler :: MVar () -> Handler
intHandler v = Catch $ putMVar v ()

runEngineFileModified :: [UTCTime] -> Options -> FilePath -> MVar () -> IO ()
runEngineFileModified time1 args fp v =
    threadDelay 1000000 >> getTime fp >>= \time2 ->
        tryTakeMVar v >>= \case
            Just _ -> void (putStrLn "\nQuitting")
            Nothing
                | time1 == time2 -> runEngineFileModified time1 args fp v
                | otherwise -> do
                    runEngine args fp
                    putStrLn "Press Ctrl+C to finish the program"
                    newtime <- getTime fp
                    runEngineFileModified newtime args fp v
  where
    getTime = getFileTimes (isCd args) . getDir

main :: IO ()
main = do
    args <- getArgs
    currentDir <- getCurrentDirectory
    let fileName' = head $ filter (not . isOption) args
        fileName = currentDir ++ "/" ++ fileName'
    -- run TeX once
    if "--help" `elem` args
        then putStrLn helpString >> return ExitSuccess
        else runEngine args fileName
    when (or (optionLst !! 3 <$> args) && (not . or $ isHelpOption <$> args)) $ do
        pdfviewer <- getPdfViewer
    -- run Pdf
        runPdf pdfviewer fileName
        putStrLn "Press Ctrl+C to finish the program"
    -- Check whether files are modified
        timeLst <- getFileTimes (isCd args) . getDir $ fileName
    -- Setting init handler
        v <- newEmptyMVar
        installHandler sigINT (intHandler v) Nothing
        runEngineFileModified timeLst args fileName v
