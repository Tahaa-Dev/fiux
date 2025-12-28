use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind as EK},
    path::PathBuf,
};

use resext::{CtxResult, ResExt};
use serde::de::IgnoredAny;

pub fn validate_ndjson(path: &PathBuf, verbose: bool) -> CtxResult<(), Error> {
    let file = File::open(path)
        .context("Failed to validate file")
        .with_context(|| format!("Failed to open input file: {}", &path.to_string_lossy()))?;

    let mut reader = BufReader::with_capacity(256 * 1024, file);

    // read lines one by one and deserialize them to check for errors
    let mut buf: Vec<u8> = Vec::new();
    let mut idx: usize = 1;

    loop {
        // check for line reading errors
        let n = reader
            .read_until(b'\n', &mut buf)
            .with_context(|| format!("Failed to read line: {} in input file", idx))?;

        // check for EOF
        if n == 0 {
            break;
        };

        // check line validity
        serde_json::from_slice::<IgnoredAny>(&buf)
            .map_err(|_| Error::new(EK::InvalidData, "Invalid NDJSON data"))
            .context("Input file is invalid")
            .with_context(|| {
                format!(
                    "Invalid NDJSON values in input file: {} at line: {}",
                    &path.to_string_lossy(),
                    idx
                )
            })?;
        buf.clear();
        idx += 1;
    }

    if verbose {
        println!("File: {} is valid", &path.to_string_lossy())
    } else {
        println!("File is valid")
    }

    Ok(())
}
