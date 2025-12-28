use std::{fs::File, io::BufReader, path::PathBuf};

use resext::ResExt;

#[inline]
pub fn csv_reader(path: &PathBuf, verbose: bool) -> csv::Reader<BufReader<File>> {
    let file = File::open(path).dyn_expect(
        || format!("ERROR: Couldn't open input file [{}].", path.to_str().unwrap_or("[input.csv]")),
        1,
        verbose,
    );

    let buffered_reader = BufReader::with_capacity(256 * 1024, file);

    csv::Reader::from_reader(buffered_reader)
}
