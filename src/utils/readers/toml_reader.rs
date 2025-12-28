use std::path::PathBuf;

use resext::ResExt;

#[inline]
pub fn toml_reader(path: &PathBuf, verbose: bool) -> toml::Value {
    let file_bytes = std::fs::read(path).dyn_expect(
        || format!("Failed to read input file: {}", path.to_str().unwrap_or("[input.toml]")),
        1,
        verbose,
    );

    toml::from_slice::<toml::Value>(&file_bytes).dyn_expect(
        || format!("Invalid TOML data in input file: {}", path.to_str().unwrap_or("[input.toml]")),
        1,
        verbose,
    )
}
