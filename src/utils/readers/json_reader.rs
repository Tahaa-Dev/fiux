use std::{fs::File, io::BufReader, path::PathBuf};

#[inline]
pub fn json_reader(
    path: &PathBuf,
) -> serde_json::Deserializer<serde_json::de::IoRead<BufReader<File>>> {
    let file = File::open(path).unwrap_or_else(|e| {
        eprintln!("Failed to open input file\nError: {}", e);
        std::process::exit(1);
    });

    let buffered = BufReader::with_capacity(256 * 1024, file);

    serde_json::Deserializer::from_reader(buffered)
}
