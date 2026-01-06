use resext::{CtxResult, ResExt};
use serde::de::IgnoredAny;
use serde_json::Deserializer;
use std::{fs::File, io::BufReader, path::PathBuf};

pub(crate) fn validate_json(path: &PathBuf) -> CtxResult<(), std::io::Error> {
    let file = File::open(path)
        .context("Failed to validate file")
        .with_context(|| format!("Failed to open input file: {}", &path.to_string_lossy()))?;

    let reader = BufReader::with_capacity(256 * 1024, file);

    let file_stream = Deserializer::from_reader(reader).into_iter::<IgnoredAny>();

    let mut res = Ok(());

    for item in file_stream {
        item.with_context(|| {
            format!("Invalid JSON data in input file: {}", &path.to_string_lossy())
        })
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
    }

    res
}
