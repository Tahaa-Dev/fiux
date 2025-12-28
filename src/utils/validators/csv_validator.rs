use std::{
    fs::File,
    io::{BufReader, Error},
    path::PathBuf,
};

use resext::{CtxResult, ResExt};

pub fn validate_csv(path: &PathBuf, verbose: bool) -> CtxResult<(), Error> {
    let file = File::open(path)
        .context("Failed to validate input file")
        .context("Failed to open input file")?;

    let buf = BufReader::with_capacity(256 * 1024, file);

    let mut reader = csv::Reader::from_reader(buf);

    let headers = reader
        .byte_headers()
        .map_err(|_| Error::new(std::io::ErrorKind::InvalidData, "CSV file headers missing"))
        .with_context(|| format!("Input file: {} is not valid", &path.to_string_lossy()))
        .context("Failed to read input file headers");

    if verbose {
        headers
            .context("CSV files are required to have valid headers for parsing and validation")?;
    } else {
        headers?;
    }

    for (idx, rec) in reader.byte_records().enumerate() {
        rec.map_err(|_| Error::new(std::io::ErrorKind::InvalidData, "Invalid CSV in input file"))
            .with_context(|| format!("Input file: {} is not valid", &path.to_string_lossy()))
            .with_context(|| format!("Invalid CSV data at record: {}", idx + 1))?;
    }

    println!("Input file: {} is valid", &path.to_string_lossy());

    Ok(())
}
