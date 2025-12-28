use resext::{CtxResult, ResExt};
use serde::de::IgnoredAny;
use serde_json::Deserializer;
use std::{
    fs::File,
    io::{BufReader, Error, ErrorKind as EK},
    path::PathBuf,
};

pub fn validate_json(path: &PathBuf, _verbose: bool) -> CtxResult<(), Error> {
    let file: File = File::open(path)
        .context("Failed to validate file")
        .with_context(|| format!("Failed to open input file: {}", &path.to_string_lossy()))?;

    let reader = BufReader::with_capacity(256 * 1024, file);

    let file_stream = Deserializer::from_reader(reader).into_iter::<IgnoredAny>();

    for item in file_stream {
        item.map_err(|_| Error::new(EK::InvalidData, "Invalid JSON values"))
            .context("Input file is invalid")
            .with_context(|| {
                format!("Invalid JSON data in input file: {}", &path.to_string_lossy())
            })?;
    }

    println!("Input file: {} is valid", &path.to_string_lossy());

    Ok(())
}
