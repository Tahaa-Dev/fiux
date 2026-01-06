use std::path::PathBuf;

use resext::{CtxResult, ResExt};

/// Toml cannot be streamed so how validation for it works is by reading the whole file into memory
/// then trying to serialize it and if it hits an error, it prints an error message like all other
/// validators except for line numbers.
pub(crate) fn validate_toml(path: &PathBuf) -> CtxResult<(), std::io::Error> {
    let file_bytes = std::fs::read(path)
        .context("Failed to validate file")
        .with_context(|| format!("Failed to open input file: {}", &path.to_string_lossy()))?;

    let mut res = Ok(());

    toml::from_slice::<serde::de::IgnoredAny>(&file_bytes)
        .with_context(|| format!("Invalid TOML values in input file: {}", &path.to_string_lossy()))
        .unwrap_or_else(|e: resext::ErrCtx<toml::de::Error>| {
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
            serde::de::IgnoredAny
        });

    println!(
        "This file was not streamed due to TOML's limitations with streaming.\nIt was all loaded into memory as bytes then validated."
    );

    res
}
