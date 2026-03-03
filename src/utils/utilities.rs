use csv::ByteRecord;

use serde::Serialize;

use crate::utils::{CtxResult, CtxResultExt};

pub enum WriterStreams<I>
where
    I: Iterator<Item = CtxResult<DataTypes>>,
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

#[inline(always)]
pub fn into_byte_record(brec: CtxResult<DataTypes>) -> CtxResult<ByteRecord> {
    let rec = brec.context("Failed to unwrap record")?;

    match rec {
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

#[inline(always)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_quote() {
        let mut out = Vec::new();
        escape(b'"', &mut out);
        assert_eq!(out, b"\\\"");
    }

    #[test]
    fn test_escape_newline() {
        let mut out = Vec::new();
        escape(b'\n', &mut out);
        assert_eq!(out, b"\\n");
    }

    #[test]
    fn test_escape_backslash() {
        let mut out = Vec::new();
        escape(b'\\', &mut out);
        assert_eq!(out, b"\\\\");
    }

    #[test]
    fn test_escape_tab() {
        let mut out = Vec::new();
        escape(b'\t', &mut out);
        assert_eq!(out, b"\\t");
    }

    #[test]
    fn test_escape_carriage_return() {
        let mut out = Vec::new();
        escape(b'\r', &mut out);
        assert_eq!(out, b"\\r");
    }

    #[test]
    fn test_no_escape_needed() {
        let mut out = Vec::new();
        escape(b'a', &mut out);
        assert_eq!(out, b"a");

        out.clear();
        escape(b'0', &mut out);
        assert_eq!(out, b"0");
    }

    #[test]
    fn test_needs_escape_table() {
        // Should escape these
        assert!(NEEDS_ESCAPE[b'"' as usize]);
        assert!(NEEDS_ESCAPE[b'\n' as usize]);
        assert!(NEEDS_ESCAPE[b'\r' as usize]);
        assert!(NEEDS_ESCAPE[b'\t' as usize]);
        assert!(NEEDS_ESCAPE[b'\\' as usize]);

        // Should NOT escape these
        assert!(!NEEDS_ESCAPE[b'a' as usize]);
        assert!(!NEEDS_ESCAPE[b'0' as usize]);
        assert!(!NEEDS_ESCAPE[b' ' as usize]);
        assert!(!NEEDS_ESCAPE[b',' as usize]);
    }
}
