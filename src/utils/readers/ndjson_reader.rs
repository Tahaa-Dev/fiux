use std::{fs::File, io::BufReader, path::PathBuf};

use resext::ResExt;

#[inline]
pub(crate) fn ndjson_reader(path: &PathBuf) -> BufReader<File> {
    let file = File::open(path).dyn_expect(
        || format!("Failed to open input file: {}", path.to_str().unwrap_or("[input.ndjson]")),
        1,
        true,
    );

    BufReader::with_capacity(256 * 1024, file)
}
