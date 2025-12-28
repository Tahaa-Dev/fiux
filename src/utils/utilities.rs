use std::process::exit;

use csv::ByteRecord;
use resext::CtxResult;
use serde::Serialize;

pub enum WriterStreams<I>
where
    I: Iterator<Item = CtxResult<DataTypes, std::io::Error>>,
{
    Values { iter: I },

    Table { headers: Vec<String>, iter: I },

    Ndjson { values: I },

    Temp {},
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
