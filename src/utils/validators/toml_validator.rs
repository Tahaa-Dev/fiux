use std::path::PathBuf;

use resext::{CtxResult, ResExt};

/// Toml cannot be streamed so how validation for it works is by reading the whole file into memory
/// then trying to serialize it and if it hits an error, it prints an error message like all other
/// validators except for line numbers.
pub fn validate_toml(path: &PathBuf, verbose: bool) -> CtxResult<(), std::io::Error> {
    let file_bytes = std::fs::read(path)
        .context("Failed to validate file")
        .with_context(|| format!("Failed to open input file: {}", &path.to_string_lossy()))?;

    toml::from_slice::<serde::de::IgnoredAny>(&file_bytes)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid TOML data"))
        .context("File is invalid")
        .with_context(|| {
            format!("Invalid TOML values in input file: {}", &path.to_string_lossy())
        })?;

    println!("Input file: {} is valid", &path.to_string_lossy());

    if verbose {
        println!(
            "This file was not streamed due to TOML's limitations with streaming.\ninstead it was all loaded into memory as raw bytes then validated for max efficiency."
        );
    }

    Ok(())
}
