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
    ServerTypeNone,
    ServerUpdateError,
    ServerNotFund(String),
    MutexError,
    RegisterError(String),
    PasswordErrorOrUserNofFound,
    Unauthorized,
    AccountNotFound,
    ParameterError,
    InsufficientBalance(String),
    DiscoverError(String),
}

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            IOError => write!(f, "io error"),
            SocketBindErr => write!(f, "Socket Bind Error"),
            UndefinedBehavior => write!(f, "Undefined Behavior"),
            ServerTypeNone => write!(f, "server type none"),
            ServerUpdateError => write!(f, "server update error"),
            ServerNotFund(ref s) => write!(f, "server: {}  not found", s),
            MutexError => write!(f, "MutexError"),
            RegisterError(ref s) => write!(f, "register error: {}", s),
            PasswordErrorOrUserNofFound => write!(f, "PasswordErrorOrUserNotFound"),
            Unauthorized => write!(f, "Unauthorized"),
            AccountNotFound => write!(f, "Account Not Found"),
            ParameterError => write!(f, "ParameterError"),
            InsufficientBalance(ref s) => write!(f, "{}", s),
            DiscoverError(ref s) => write!(f, "DiscoverError: {}", s),
        }
    }
}

impl Error for CommonError {
    fn description(&self) -> &str {
        match *self {
            IOError => "io error",
            SocketBindErr => "socket bind error",
            UndefinedBehavior => "Undefined Behavior",
            ServerTypeNone => "server type is none",
            ServerUpdateError => "server update error",
            ServerNotFund(..) => "server not found",
            MutexError => "MutexError",
            RegisterError(..) => "register error",
            PasswordErrorOrUserNofFound => "PasswordErrorOrUserNotFound",
            Unauthorized => "Unauthorized",
            AccountNotFound => "Account Not Found",
            ParameterError => "ParameterError",
            InsufficientBalance(..) => "InsufficientBalance",
            DiscoverError(..) => "DiscoverError",
        }
    }
}
