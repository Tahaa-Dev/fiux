use std::{fs::File, io::BufReader, path::PathBuf};

#[inline]
pub(crate) fn csv_reader(path: &PathBuf, delimiter: char) -> csv::Reader<BufReader<File>> {
    let file = File::open(path).unwrap_or_else(|e| {
        eprintln!("Failed to open input file\nError: {}", e);
        std::process::exit(1);
    });

    let buffered_reader = BufReader::with_capacity(256 * 1024, file);

    if !delimiter.is_ascii() {
        eprintln!("Input delimiter: {} is not valid UTF-8", delimiter);
        std::process::exit(1);
    }

    let d = delimiter as u8;

    csv::ReaderBuilder::new().delimiter(d).from_reader(buffered_reader)
}
