use std::{fs::File, io::BufReader, path::PathBuf};

use resext::ResExt;

#[inline]
pub fn ndjson_reader(path: &PathBuf, verbose: bool) -> BufReader<File> {
    let file = File::open(path).dyn_expect(
        || format!("Failed to open input file: {}", path.to_str().unwrap_or("[input.ndjson]")),
        1,
        verbose,
    );

    BufReader::with_capacity(256 * 1024, file)
}
