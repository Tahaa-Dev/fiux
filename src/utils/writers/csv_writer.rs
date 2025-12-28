use crate::utils::{BetterExpect, DataTypes, WriterStreams, into_byte_record};

use std::io::BufWriter;

#[inline]
pub fn csv_writer(
    data_stream: WriterStreams<impl Iterator<Item = DataTypes>>,
    file: std::fs::File,
    verbose: bool,
) {
    let buffered = BufWriter::new(file);
    let mut wtr = csv::Writer::from_writer(buffered);

    match data_stream {
        WriterStreams::Table { headers, iter } => {
            // write headers
            wtr.write_record(&headers)
                .better_expect("ERROR: Couldn't write file headers.", verbose);

            // write records
            iter.enumerate().for_each(|(line_no, line)| {
                wtr.write_record(&into_byte_record(line)).better_expect(
                    format!("ERROR: Couldn't write record [{}] into output file.", line_no,)
                        .as_str(),
                    verbose,
                );
            });

            // flush writer
            wtr.flush().better_expect(
                "ERROR: An error occurred while writing to the output file",
                verbose,
            );
        }
        _ => {
            eprintln!("ERROR: CSV only supports table-based formats with headers.");
            eprintln!("Support for non-table formats will be added soon.");
            std::process::exit(1);
        }
    }
}
