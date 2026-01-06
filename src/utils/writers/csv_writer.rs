use resext::{CtxResult, ErrCtx, ResExt, throw_err_if};

use crate::utils::{DataTypes, WriterStreams, into_byte_record};

use std::io::{BufWriter, Error};

#[inline]
pub(crate) fn csv_writer(
    data_stream: WriterStreams<impl Iterator<Item = CtxResult<DataTypes, Error>>>,
    file: std::fs::File,
    delimiter: char,
) -> CtxResult<(), Error> {
    let buffered = BufWriter::new(file);

    throw_err_if!(
        !delimiter.is_ascii(),
        || format!("FATAL: Output delimiter: {} is not valid UTF-8", delimiter),
        1
    );

    let d = delimiter as u8;
    let mut wtr = csv::WriterBuilder::new().delimiter(d).from_writer(buffered);

    match data_stream {
        WriterStreams::Table { headers, iter } => {
            // write headers
            wtr.write_record(&headers)
                .map_err(|_| Error::other("Failed to write headers"))
                .context("Failed to write headers into output file")?;

            // write records
            for (line_no, line) in iter.enumerate() {
                let b = into_byte_record(line)
                    .context("Failed to re-serialize object for writing")
                    .unwrap_or_else(|e: ErrCtx<Error>| {
                        crate::utils::log_err(&e)
                            .unwrap_or_else(|err| eprintln!("{}\n{}", err, &e));
                        csv::ByteRecord::from(vec![b""; headers.len()])
                    });
                wtr.write_record(&b)
                    .map_err(|_| Error::other("Failed to write CSV record"))
                    .with_context(|| {
                        format!("FATAL: Failed to write CSV record at: {}", line_no + 1)
                    })
                    .context(crate::VERBOSE_HELP)?;
            }

            // flush writer
            wtr.flush()
                .map_err(|_| Error::other("Failed to flush"))
                .context("FATAL: Failed to flush final bytes into output file")?;
        }
        _ => {
            eprintln!("CSV only supports table-based formats with headers");
            eprintln!("» Support for non-table formats will be added soon");
            std::process::exit(1);
        }
    }

    Ok(())
}
