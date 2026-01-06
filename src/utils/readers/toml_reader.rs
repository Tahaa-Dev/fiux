use std::path::PathBuf;

use resext::ResExt;

#[inline]
pub(crate) fn toml_reader(path: &PathBuf) -> Vec<u8> {
    std::fs::read(path).dyn_expect(
        || format!("Failed to read input file: {}", path.to_str().unwrap_or("[input.toml]")),
        1,
        true,
    )
}
