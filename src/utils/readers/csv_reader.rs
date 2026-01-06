use std::{fs::File, io::BufReader, path::PathBuf};

use resext::{ResExt, throw_err_if};

#[inline]
pub(crate) fn csv_reader(path: &PathBuf, delimiter: char) -> csv::Reader<BufReader<File>> {
    let file = File::open(path).dyn_expect(
        || format!("FATAL: Couldn't open input file {}", path.to_str().unwrap_or("[input.csv]")),
        1,
        true,
    );

    let buffered_reader = BufReader::with_capacity(256 * 1024, file);

    throw_err_if!(
        !delimiter.is_ascii(),
        || format!("FATAL: Input delimiter: {} is not valid UTF-8", delimiter),
        1
    );

    let d = delimiter as u8;

    csv::ReaderBuilder::new().delimiter(d).from_reader(buffered_reader)
}
