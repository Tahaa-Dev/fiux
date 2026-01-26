use std::{fs::File, io::BufReader, path::PathBuf};

use resext::{CtxResult, ResExt, panic_if};

pub(crate) fn validate_csv(path: &PathBuf, delimiter: char) -> CtxResult<(), std::io::Error> {
    let file = File::open(path)
        .context("Failed to validate file")
        .with_context(|| format!("Failed to open input file: {}", &path.to_string_lossy()))?;

    let buf = BufReader::with_capacity(256 * 1024, file);

    panic_if!(
        !delimiter.is_ascii(),
        || format!("Input delimiter: {} is not valid UTF-8", delimiter),
        1
    );

    let d = delimiter as u8;

    let mut reader = csv::ReaderBuilder::new().delimiter(d).from_reader(buf);

    let mut res = Ok(());

    reader
        .byte_headers()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{e}")))
        .with_context(|| format!("Input file: {} is invalid", &path.to_string_lossy()))
        .context("Failed to read input file headers")
        .context("CSV files are required to have valid headers for parsing and validation")?;

    for (idx, rec) in reader.byte_records().enumerate() {
        rec.with_context(|| format!("Invalid CSV data at record: {}", idx + 1)).unwrap_or_else(
            |e: resext::ErrCtx<csv::Error>| {
                crate::utils::log_err(&e).unwrap_or_else(|err| eprintln!("{}\n{}", err, &e));

                if res.is_ok() {
                    res = Err(resext::ErrCtx::new(
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Invalid CSV in input file",
                        ),
                        b"Input file is invalid".to_vec(),
                    ));
                }
                csv::ByteRecord::default()
            },
        );
    }

    res
}
