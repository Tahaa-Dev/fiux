use crate::utils::{CtxResult, CtxResultExt, DataTypes, Log, WriterStreams, into_byte_record};

use std::io::BufWriter;

pub fn csv_writer(
    data_stream: WriterStreams<impl Iterator<Item = CtxResult<DataTypes>>>,
    file: std::fs::File,
    delimiter: char,
) -> CtxResult<()> {
    let buffered = BufWriter::new(file);

    if !delimiter.is_ascii() {
        eprintln!("Output delimiter: {} is not valid UTF-8", delimiter);
        std::process::exit(1);
    }

    let d = delimiter as u8;
    let mut wtr = csv::WriterBuilder::new().delimiter(d).from_writer(buffered);

    match data_stream {
        WriterStreams::Table { headers, iter } => {
            wtr.write_record(&headers).context("Failed to write headers into output file")?;

            for (line_no, line) in iter.enumerate() {
                let b =
                    into_byte_record(line).context("Failed to re-serialize object").log("[WARN]");

                let b = match b {
                    Some(b) => b,
                    None => continue,
                };

                wtr.write_record(&b)
                    .context(|| format!("Failed to write CSV record at: {}", line_no + 1))?;
            }

            wtr.flush().context("Failed to flush writer")?;
        }
        _ => {
            eprintln!("CSV only supports table-based formats with headers");
            eprintln!(" -> Support for non-table formats will be added soon");
            std::process::exit(1);
        }
    }

    Ok(())
}
