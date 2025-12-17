use std::io::{BufWriter, Write};

use crate::utils::{BetterExpect, DataTypes, WriterStreams, into_byte_record};

#[inline]
pub fn write_json(
    data_stream: WriterStreams<impl Iterator<Item = DataTypes>>,
    file: std::fs::File,
    verbose: bool,
    parse_numbers: bool,
) {
    let mut buffered_writer = BufWriter::new(&file);

    match data_stream {
        WriterStreams::Values { iter } => {
            iter.for_each(|obj| {
                serde_json::to_writer_pretty(&mut buffered_writer, &obj)
                    .better_expect("ERROR: Failed to write JSON into output file [{}].", verbose);

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
                        .replace('\n', "\\n")
                })
                .collect();

            let mut first_obj = true;

            iter.for_each(|rec| {
                if first_obj {
                    buffered_writer.write_all(b"  {\n").better_expect(
                        "ERROR: Failed to write opening curly brace into output file.",
                        verbose,
                    );
                    first_obj = false;
                } else {
                    buffered_writer.write_all(b",\n  {\n").better_expect(
                        "ERROR: Failed to write opening curly brace into output file.",
                        verbose,
                    );
                }

                let mut first_value = true;

                let record = into_byte_record(rec);
                headers.iter().zip(record.iter()).for_each(|(h, v)| {
                    esc_buf.clear();
                    if matches!(v, b"true" | b"false" | b"null")
                        || (parse_numbers
                            && v.first()
                                .is_some_and(|b| *b == b'-' || *b == b'+' || b.is_ascii_digit())
                            && v.last().is_some_and(|b| b.is_ascii_digit())
                            && std::str::from_utf8(v).unwrap_or("").parse::<f64>().is_ok())
                    {
                        esc_buf.extend_from_slice(v);
                    } else {
                        esc_buf.push(b'"');
                        let mut start = 0usize;
                        v.iter().enumerate().for_each(|(idx, byte)| {
                            let chunk = &v[start..idx];
                            match *byte {
                                b'\\' => {
                                    esc_buf.extend_from_slice(chunk);
                                    esc_buf.extend_from_slice(b"\\\\");
                                    start = idx + 1;
                                }
                                b'\n' => {
                                    esc_buf.extend_from_slice(chunk);
                                    esc_buf.extend_from_slice(b"\\n");
                                    start = idx + 1;
                                }
                                b'"' => {
                                    esc_buf.extend_from_slice(chunk);
                                    esc_buf.extend_from_slice(b"\\\"");
                                    start = idx + 1;
                                }
                                b'\t' => {
                                    esc_buf.extend_from_slice(chunk);
                                    esc_buf.extend_from_slice(b"\\t");
                                    start = idx + 1;
                                }
                                b'\r' => {
                                    esc_buf.extend_from_slice(chunk);
                                    esc_buf.extend_from_slice(b"\\r");
                                    start = idx + 1;
                                }
                                _ => {}
                            }
                        });
                        if start < v.len() {
                            esc_buf.extend_from_slice(&v[start..]);
                        }
                        esc_buf.push(b'"');
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
                        .write_all(h.as_bytes())
                        .better_expect("ERROR: Failed to write key into output file.", verbose);

                    buffered_writer.write_all(b"\": ").better_expect(
                        "ERROR: Failed to write quotes for key into output file.",
                        verbose,
                    );

                    buffered_writer
                        .write_all(esc_buf.as_slice())
                        .better_expect("ERROR: Failed to write value into output file.", verbose);
                });
                buffered_writer.write_all(b"\n  }").better_expect(
                    "ERROR: Failed to write closing curly brace into output file.",
                    verbose,
                );
            });
            buffered_writer
                .write_all(b"\n]")
                .better_expect("ERROR: Failed to write closing bracket into output file.", verbose);

            buffered_writer
                .flush()
                .better_expect("ERROR: Failed to flush final writer bytes.", verbose);
        }

        _ => {}
    }
}
