use std::env;
use std::str::FromStr;

use crate::environment::Environment::{Development, Production, Staging};
use crate::err::CrateError;

const CONFIG_ENV: &str = "POEM_ENV";


#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

impl Environment {
    pub fn action() -> Result<Environment, CrateError> {
        match env::var(CONFIG_ENV) {
            Ok(s) => s.parse().map_err(|_| CrateError::BadEnv(s)),
            _ => Ok(Development),

            #[cfg(not(debug_assertions))]
            _ => Ok(Production),
        }
    }
}

impl FromStr for Environment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let env = match s {
            "d" | "dev" | "devel" | "development" => Development,
            "s" | "stage" | "staging" => Staging,
            "p" | "prod" | "production" => Production,
            _ => return Err(()),
        };

        Ok(env)
    }
}