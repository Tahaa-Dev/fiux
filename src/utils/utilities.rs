use std::process::exit;

use colored::Colorize;

use csv::ByteRecord;
use serde::Serialize;

pub enum WriterStreams {
    Values { iter: Box<dyn Iterator<Item = DataTypes>> },

    Table { headers: Vec<String>, iter: Box<dyn Iterator<Item = DataTypes>> },
}

pub enum DataTypes {
    Json(serde_json::Value),

    Toml(toml::Value),

    Csv(ByteRecord),
}

impl Serialize for DataTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            DataTypes::Json(j) => j.serialize(serializer),
            DataTypes::Toml(t) => t.serialize(serializer),
            DataTypes::Csv(c) => c.as_slice().serialize(serializer),
        }
    }
}

pub fn into_byte_record(brecord: DataTypes) -> ByteRecord {
    if let DataTypes::Csv(brec) = brecord { brec } else { ByteRecord::new() }
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
