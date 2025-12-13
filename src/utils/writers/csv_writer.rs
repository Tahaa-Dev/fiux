use colored::Colorize;

use crate::utils::{BetterExpect, WriterStreams, into_byte_record};

use std::path::PathBuf;

pub fn csv_writer(data_stream: WriterStreams, path: &PathBuf, verbose: bool) {
    let mut wtr = csv::Writer::from_path(path).better_expect(
        format!(
            "ERROR: Couldn't open output file [{}] for writing.",
            path.to_str().unwrap_or("[output.csv]")
        )
        .as_str(),
        verbose,
    );

    match data_stream {
        WriterStreams::Table { headers, iter } => {
            // write headers
            wtr.write_record(&headers)
                .better_expect("ERROR: Couldn't write file headers.", verbose);

            // write records
            iter.enumerate().for_each(|(line_no, line)| {
                wtr.write_record(&into_byte_record(line)).better_expect(
                    format!(
                        "ERROR: Couldn't write record [{}] into output file [{}].",
                        line_no + 1,
                        path.to_str().unwrap_or("[output.csv]")
                    )
                    .as_str(),
                    verbose,
                );
            });

            // flush writer
            wtr.flush().better_expect(
                format!(
                    "ERROR: An error occurred while writing to the output file [{}]",
                    path.to_str().unwrap_or("[output.csv]")
                )
                .as_str(),
                verbose,
            );
        }
        WriterStreams::Values { iter } => {
            iter.last();
            eprintln!(
                "{}",
                "ERROR: CSV only supports table-based formats with headers.".red().bold()
            );
            eprintln!("Support for non-table formats will be added soon.");
            std::process::exit(1);
        }
    }
}
