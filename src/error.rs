use std::fmt;
use std::io;
use yaml_rust::scanner::ScanError;

#[derive(Debug)]
pub enum AutoTeXErr {
    IOErr(io::Error),
    ScanErr(ScanError),
    CommandErr(clap::Error),
    NoneError,
    NoFilenameInputErr,
    TakeFilesErr,
    InvalidOptionErr,
    CannotShowPdfErr,
}

impl fmt::Display for AutoTeXErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use AutoTeXErr::*;
        match *self {
            IOErr(ref e) => e.fmt(f),
            ScanErr(ref e) => e.fmt(f),
            CommandErr(ref e) => e.fmt(f),
            NoneError => write!(f, "NoneError"),
            NoFilenameInputErr => write!(f, "There is no filename to compile"),
            TakeFilesErr => write!(f, "Cannot make a list of tex relative files"),
            InvalidOptionErr => write!(f, "No tex option is used"),
            CannotShowPdfErr => write!(f, "failed to show pdf using `-v` command"),
        }
    }
}

impl From<io::Error> for AutoTeXErr {
    fn from(err: io::Error) -> Self {
        Self::IOErr(err)
    }
}

impl From<ScanError> for AutoTeXErr {
    fn from(err: ScanError) -> Self {
        Self::ScanErr(err)
    }
}

impl From<clap::Error> for AutoTeXErr {
    fn from(err: clap::Error) -> Self {
        Self::CommandErr(err)
    }
}

pub type Result<T> = std::result::Result<T, AutoTeXErr>;
