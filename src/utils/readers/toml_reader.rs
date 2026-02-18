#[inline]
pub(crate) fn toml_reader(path: &std::path::PathBuf) -> Vec<u8> {
    std::fs::read(path).unwrap_or_else(|e| {
        eprintln!("Failed to read input file\nError: {}", e);
        std::process::exit(1);
    })
}
