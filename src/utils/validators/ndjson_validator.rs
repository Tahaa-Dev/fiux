use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use resext::{CtxResult, ResExt};
use serde::de::IgnoredAny;

pub(crate) fn validate_ndjson(path: &PathBuf) -> CtxResult<(), std::io::Error> {
    let file = File::open(path)
        .context("Failed to validate file")
        .with_context(|| format!("Failed to open input file: {}", &path.to_string_lossy()))?;

    let mut reader = BufReader::with_capacity(256 * 1024, file);

    // read lines one by one and deserialize them to check for errors
    let mut buf: Vec<u8> = Vec::new();
    let mut idx: usize = 1;
    let mut res = Ok(());

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
            .with_context(|| format!("Invalid NDJSON values in input file at line: {}", idx))
            .unwrap_or_else(|e: resext::ErrCtx<serde_json::Error>| {
                crate::utils::log_err(&e).unwrap_or_else(|err| eprintln!("{}\n{}", err, &e));

                if res.is_ok() {
                    res = Err(resext::ErrCtx::new(
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Invalid JSON in input file",
                        ),
                        b"Input file is invalid".to_vec(),
                    ));
                }
                IgnoredAny
            });
        buf.clear();
        idx += 1;
    }

    res
}
