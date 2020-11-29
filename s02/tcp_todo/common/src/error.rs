use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use self::CommonError::*;

// https://lotabout.me/2017/rust-error-handling/

#[derive(Debug)]
pub enum CommonError {
    IOError,
    SocketBindErr,
    UndefinedBehavior,
}

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            IOError => write!(f, "io error"),
            SocketBindErr => write!(f, "Socket Bind Error"),
            UndefinedBehavior => write!(f, "Undefined Behavior"),
        }
    }
}

impl Error for CommonError {
    fn description(&self) -> &str {
        match *self {
            IOError => "io error",
            SocketBindErr => "socket bind error",
            UndefinedBehavior => "Undefined Behavior",
        }
    }
}
