use std::path::PathBuf;

use crate::utils::BetterExpect;

/// Toml cannot be streamed so how validation for it works is by reading the whole file into memory
/// then trying to serialize it and if it hits an error, it prints an error message like all other
/// validators except for line numbers.
pub fn validate_toml(path: &PathBuf, verbose: bool) {
    let file_bytes = std::fs::read(path).better_expect(
        format!(
            "ERROR: Couldn't read input TOML file [{}].",
            path.to_str().unwrap_or("[input.toml]")
        )
        .as_str(),
        verbose,
    );

    toml::from_slice::<toml::Value>(&file_bytes).better_expect(
        format!(
            "ERROR: Serialization error in input TOML file [{}].",
            path.to_str().unwrap_or("[input.toml]")
        )
        .as_str(),
        verbose,
    );
}
