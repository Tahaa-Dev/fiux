use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::PathBuf,
};

use crate::utils::{BetterExpect, WriterStreams, into_byte_record};

pub fn write_json(data_stream: WriterStreams, path: &PathBuf, verbose: bool) {
    let file = OpenOptions::new().write(true).open(path).better_expect(
        format!(
            "ERROR: Couldn't open output file [{}] for writing.",
            path.to_str().unwrap_or("[output.json]")
        )
        .as_str(),
        verbose,
    );

    let mut buffered_writer = BufWriter::new(&file);

    match data_stream {
        WriterStreams::Values { iter } => {
            iter.for_each(|obj| {
                serde_json::to_writer_pretty(&mut buffered_writer, &obj).better_expect(
                    format!(
                        "ERROR: Failed to write JSON into output file [{}].",
                        path.to_str().unwrap_or("[output.json]")
                    )
                    .as_str(),
                    verbose,
                );

                writeln!(buffered_writer).better_expect(
                    "ERROR: Failed to write a newline into the output file.",
                    verbose,
                );
            });

            buffered_writer
                .flush()
                .better_expect("ERROR: Failed to flush writer into output file.", verbose);
        }

        WriterStreams::Table { headers, iter } => {
            // buffer for escapijg values which will get cleared after each value and
            // reused instead of allocating a new `Vec<u8>` for every value
            let mut esc_buf: Vec<u8> = Vec::with_capacity(10);

            buffered_writer
                .write(b"[\n")
                .better_expect("ERROR: Failed to write opening bracket into output file.", verbose);

            let headers: Vec<String> = headers
                .iter()
                .map(|h| {
                    h.replace('\\', "\\\\")
                        .replace('"', "\\\"")
                        .replace('\t', "\\t")
                        .replace('\r', "\\r")
                })
                .collect();

            let mut first_obj = true;

            iter.for_each(|rec| {
                if first_obj {
                    buffered_writer.write(b"  {\n").better_expect(
                        "ERROR: Failed to write opening curly brace into output file.",
                        verbose,
                    );
                    first_obj = false;
                } else {
                    buffered_writer.write(b",\n  {\n").better_expect(
                        "ERROR: Failed to write opening curly brace into output file.",
                        verbose,
                    );
                }

                let mut first_value = true;

                let record = into_byte_record(rec);
                headers.iter().zip(record.iter()).for_each(|(h, v)| {
                    esc_buf.clear();
                    if !matches!(v, b"true" | b"false" | b"null") {
                        esc_buf.push(b'"');
                        v.iter().for_each(|byte| match *byte {
                            b'\\' => {
                                esc_buf.push(b'\\');
                                esc_buf.push(b'\\');
                            }
                            b'"' => {
                                esc_buf.push(b'\\');
                                esc_buf.push(b'"');
                            }
                            b'\r' => {
                                esc_buf.push(b'\\');
                                esc_buf.push(b'r');
                            }
                            b'\t' => {
                                esc_buf.push(b'\\');
                                esc_buf.push(b't');
                            }
                            _ => esc_buf.push(*byte),
                        });
                        esc_buf.push(b'"');
                    } else {
                        esc_buf = v.to_vec();
                    }
                    if first_value {
                        buffered_writer.write(b"    \"").better_expect(
                            "ERROR: Failed to write quotes for key into output file.",
                            verbose,
                        );
                        first_value = false;
                    } else {
                        buffered_writer.write(b",\n    \"").better_expect(
                            "ERROR: Failed to write quotes for key into output file.",
                            verbose,
                        );
                    }

                    buffered_writer
                        .write(h.as_bytes())
                        .better_expect("ERROR: Failed to write key into output file.", verbose);

                    buffered_writer.write(b"\": ").better_expect(
                        "ERROR: Failed to write quotes for key into output file.",
                        verbose,
                    );

                    buffered_writer
                        .write(esc_buf.as_slice())
                        .better_expect("ERROR: Failed to write value into output file.", verbose);
                });
                buffered_writer.write(b"\n  }").better_expect(
                    "ERROR: Failed to write closing curly brace into output file.",
                    verbose,
                );
            });
            buffered_writer
                .write(b"\n]")
                .better_expect("ERROR: Failed to write closing bracket into output file.", verbose);

            buffered_writer
                .flush()
                .better_expect("ERROR: Failed to flush final writer bytes.", verbose);
        }
    }
}
