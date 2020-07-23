use std::fmt;
use std::io;

use yaml_rust::scanner::ScanError;

#[derive(Debug)]
pub enum AutoTeXErr {
    IOErr(io::Error),
    ScanErr(ScanError),
    NoneError,
    NoFilenameInputErr,
    ParsePdfErr,
    TakeFilesErr,
    InvalidOptionErr,
    TooManyOptionsErr,
    DistinctTeXOptErr,
}

impl fmt::Display for AutoTeXErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use AutoTeXErr::*;
        match *self {
            IOErr(ref e) => e.fmt(f),
            ScanErr(ref e) => e.fmt(f),
            NoneError => write!(f, "NoneError"),
            NoFilenameInputErr => write!(f, "There is no filename to compile!"),
            ParsePdfErr => write!(f, "Cannot take a pdf reader name!"),
            TakeFilesErr => write!(f, "Cannot make a list of tex relative files!"),
            InvalidOptionErr => write!(f, "No tex option is used!"),
            TooManyOptionsErr => write!(f, "Too many options!"),
            DistinctTeXOptErr => write!(f, "Cannot use two distinct TeX options!"),
        }
    }
}

impl From<io::Error> for AutoTeXErr {
    fn from(err: io::Error) -> AutoTeXErr {
        AutoTeXErr::IOErr(err)
    }
}

impl From<ScanError> for AutoTeXErr {
    fn from(err: ScanError) -> AutoTeXErr {
        AutoTeXErr::ScanErr(err)
    }
}

pub type Result<T> = std::result::Result<T, AutoTeXErr>;
