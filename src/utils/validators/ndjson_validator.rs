use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use serde::de::IgnoredAny;

use crate::utils::{CtxResult, CtxResultErr, CtxResultExt, Log};

pub fn validate_ndjson(path: &PathBuf) -> CtxResult<()> {
    let file = File::open(path)
        .context("Failed to validate file")
        .context(format_args!("Failed to open file: {}", &path.to_string_lossy()))?;

    let mut reader = BufReader::with_capacity(256 * 1024, file);

    // read lines one by one and deserialize them to check for errors
    let mut buf: Vec<u8> = Vec::new();
    let mut idx: usize = 1;
    let mut res = Ok(());

    loop {
        // check for line reading errors
        let n = reader
            .read_until(b'\n', &mut buf)
            .context(format_args!("Failed to read line: {}", idx))?;

        // check for EOF
        if n == 0 {
            break;
        };

        // check line validity
        let opt = serde_json::from_slice::<IgnoredAny>(&buf)
            .context(format_args!("Invalid NDJSON values at line: {}", idx))
            .log("[WARN]")
            .is_none();

        if opt && res.is_ok() {
            res = Err(CtxResultErr::new("Input file is invalid", String::from("Invalid NDJSON data")));
        }

        buf.clear();
        idx += 1;
    }

    res
}
