use std::{fs::File, io::BufReader, path::PathBuf};

use crate::utils::BetterExpect;

#[inline]
pub fn ndjson_reader(path: &PathBuf, verbose: bool) -> BufReader<File> {
    let file = File::open(path).better_expect(
        format!(
            "ERROR: Failed to open input file [{}] for reading.",
            path.to_str().unwrap_or("[input.ndjson]")
        )
        .as_str(),
        verbose,
    );

    BufReader::with_capacity(256 * 1024, file)
}
