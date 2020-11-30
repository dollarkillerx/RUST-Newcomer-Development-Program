#[macro_use]
extern crate lazy_static;


pub mod define;
pub mod core;
use std::error::Error;
use common::error::{CommonError::*};
use chrono::Utc;
