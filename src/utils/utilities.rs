use std::io::Error;

use csv::ByteRecord;
use resext::{CtxResult, ResExt};
use serde::Serialize;

pub enum WriterStreams<I>
where
    I: Iterator<Item = CtxResult<DataTypes, Error>>,
{
    Values { iter: I },

    Table { headers: Vec<String>, iter: I },

    Ndjson { values: I },
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
            DataTypes::Csv(_) => unreachable!(),
        }
    }
}

pub fn into_byte_record(brec: CtxResult<DataTypes, Error>) -> CtxResult<ByteRecord, Error> {
    match brec.context("Failed to unwrap record")? {
        DataTypes::Csv(csv) => Ok(csv),
        _ => unreachable!(),
    }
}

const NEEDS_ESCAPE: [bool; 256] = {
    let mut table = [false; 256];
    table[b'\\' as usize] = true;
    table[b'"' as usize] = true;
    table[b'\n' as usize] = true;
    table[b'\r' as usize] = true;
    table[b'\t' as usize] = true;
    table
};

#[inline]
pub fn escape(byte: u8, output: &mut Vec<u8>) {
    if NEEDS_ESCAPE[byte as usize] {
        output.reserve_exact(2);
        output.push(b'\\');
        match byte {
            b'\\' => output.push(b'\\'),
            b'"' => output.push(b'"'),
            b'\n' => output.push(b'n'),
            b'\r' => output.push(b'r'),
            b'\t' => output.push(b't'),
            _ => unreachable!(),
        }
    } else {
        output.push(byte);
    }
}
