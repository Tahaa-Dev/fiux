use std::{fs::File, io::BufReader, path::PathBuf};

use crate::utils::{CtxResult, CtxResultErr, CtxResultExt, Log};

pub(crate) fn validate_csv(path: &PathBuf, delimiter: char) -> CtxResult<()> {
    let file = File::open(path)
        .context("Failed to validate file")
        .context(format_args!("Failed to open file: {}", &path.to_string_lossy()))?;

    let buf = BufReader::with_capacity(256 * 1024, file);

    if !delimiter.is_ascii() {
        eprintln!("Input delimiter: {} is not valid UTF-8", delimiter);
        std::process::exit(1);
    }

    let d = delimiter as u8;

    let mut reader = csv::ReaderBuilder::new().delimiter(d).from_reader(buf);

    let mut res = Ok(());

    reader
        .byte_headers()
        .context(format_args!("Input file: {} is invalid", &path.to_string_lossy()))
        .context("Failed to read input file headers")?;

    for (idx, rec) in reader.byte_records().enumerate() {
        let opt = rec.context(format_args!("Invalid CSV data at record: {}", idx + 1))
            .log("[WARN]")
            .is_none();

        if opt && res.is_ok() {
            res = Err(CtxResultErr::new("Input file is invalid", String::from("Invalid CSV data")));
        }
    }

    res
}
