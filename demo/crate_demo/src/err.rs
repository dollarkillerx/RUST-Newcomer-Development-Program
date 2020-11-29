use std::error::Error;
use std::fmt;
use std::path::PathBuf;

use serde::export::Formatter;

use self::CrateError::*;

#[derive(Debug)]
pub enum CrateError {
    PathError,
    IOError,
    NotFound,
    BadEnv(String),
    BadType(String, &'static str, &'static str, Option<PathBuf>),
    ParseError(String, PathBuf, String, Option<(usize, usize)>),
}

impl fmt::Display for CrateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            PathError => write!(f, "path error"),
            IOError => write!(f, "io error"),
            NotFound => write!(f, "not found"),
            BadEnv(ref e) => write!(f, "{:?} is not a valid `ROCKET_ENV` value", e),
            BadType(ref n, e, a, _) => {
                write!(f, "type mismatch for '{}'. expected {}, found {}", n, e, a)
            }
            ParseError(..) => write!(f, "this config file contains invalid TOML"),
        }
    }
}


impl Error for CrateError {
    fn description(&self) -> &str {
        match *self {
            PathError => "path error",
            IOError => "io error",
            NotFound => "not found",
            BadEnv(..) => "is not a valid `ROCKET_ENV` value",
            BadType(..) => "a key was specified with a value of the wrong type",
            ParseError(..) => "this config file contains invalid TOML",
        }
    }
}