use std::{fs::File, io::BufReader, path::PathBuf};

use resext::ResExt;

#[inline]
pub fn json_reader(
    path: &PathBuf,
    verbose: bool,
) -> serde_json::Deserializer<serde_json::de::IoRead<BufReader<File>>> {
    let file = File::open(path).dyn_expect(
        || format!("Failed to open input file: {}", path.to_str().unwrap_or("[input.json]")),
        1,
        verbose,
    );

    let buffered = BufReader::with_capacity(256 * 1024, file);

    serde_json::Deserializer::from_reader(buffered)
}
