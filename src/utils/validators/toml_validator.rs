use std::path::PathBuf;

use crate::utils::{CtxResult, CtxResultExt};

#[inline(always)]
pub fn validate_toml(path: &PathBuf) -> CtxResult<()> {
    let file_bytes = std::fs::read(path)
        .context("Failed to validate file")
        .context(|| format!("Failed to open file: {}", &path.to_string_lossy()))?;

    let res = toml::from_slice::<serde::de::IgnoredAny>(&file_bytes)
        .context(|| format!("Invalid TOML values: {}", &path.to_string_lossy()))
        .map(|_| ());

    println!(
        "This file was not streamed due to TOML's limitations with streaming.\nIt was all loaded into memory as bytes then validated."
    );

    res
}
