use std::process::exit;

use colored::Colorize;
use serde::ser::{Serialize, Serializer};
use serde_json::Value as JsonVal;
use toml::Value as TomlVal;
// Main enums for a universal data type so all readers and writers can share one type

// enum for all serde_ext::Value with manual serialize impl to make the wrapper transparent

#[derive(Debug, PartialEq)]
pub enum Vals {
    Json(JsonVal),
    Toml(TomlVal),
}

// the impl

impl Serialize for Vals {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Vals::Json(v) => v.serialize(serializer),
            Vals::Toml(v) => v.serialize(serializer),
        }
    }
}

// main data type enum

#[derive(Debug, PartialEq)]
pub enum UniversalData {
    Table {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },
    Structured(Vals),
}

// Custom better expect trait for better error messages without duping code

pub trait BetterExpect<T> {
    fn better_expect(self, msg: &str, verbose: bool) -> T;
}

// impl for Result which matches the value to Ok to return the value or print the error msg in red if Err
impl<T, E: std::fmt::Display> BetterExpect<T> for Result<T, E> {
    fn better_expect(self, msg: &str, verbose: bool) -> T {
        match self {
            Ok(v) => v,
            Err(_) if !verbose => {
                eprintln!("{}", msg.red().bold());
                exit(1);
            }
            Err(e) => {
                eprintln!("{}\n{}", msg.red().bold(), e);
                exit(1);
            }
        }
    }
}

// impl for Option to match the value for Some to return the actual value and if None prints error msg in red

impl<T> BetterExpect<T> for Option<T> {
    fn better_expect(self, msg: &str, _verbose: bool) -> T {
        match self {
            Some(v) => v,
            None => {
                eprintln!("{}", msg.red().bold());
                exit(1);
            }
        }
    }
}
