use std::{fs::File, io::BufReader, path::PathBuf};

#[inline]
pub(crate) fn ndjson_reader(path: &PathBuf) -> BufReader<File> {
    let file = File::open(path).unwrap_or_else(|e| {
        eprintln!("Failed to open input file\nError: {}", e);
        std::process::exit(1);
    });

    BufReader::with_capacity(256 * 1024, file)
}
