use std::io::{BufWriter, Write};

use toml::map::Map;

use crate::utils::{BetterExpect, DataTypes, WriterStreams, into_byte_record};

#[inline]
pub fn toml_writer(
    data_stream: WriterStreams<impl Iterator<Item = DataTypes>>,
    file: std::fs::File,
    verbose: bool,
    parse_numbers: bool,
) {
    let mut buffered_writer = BufWriter::new(file);

    match data_stream {
        WriterStreams::Values { iter } => {
            iter.for_each(|rec| {
                let toml_object = toml::Value::try_from(rec)
                    .better_expect("ERROR: Failed to serialize value into TOML", verbose);
                let valid_object = match toml_object {
                    toml::Value::Array(_) => {
                        let mut map = Map::new();
                        map.insert("Array".to_string(), toml_object);
                        toml::Value::Table(map)
                    }
                    _ => toml_object,
                };

                buffered_writer.write(
                    toml::to_string_pretty(&valid_object)
                    .better_expect("INTERNAL ERROR: Failed to turn TOML into bytes for writing (possible OOM or deeply nested data)!", true)
                    .as_bytes())
                    .better_expect(
                    "ERROR: Failed to write TOML into output file.", 
                         verbose
                    );
            });
        }
        WriterStreams::Table { headers, iter } => {
            let mut esc_buf: Vec<u8> = Vec::with_capacity(10);

            let headers: Vec<String> = headers
                .iter()
                .map(|h| {
                    let needs_quotes =
                        !h.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_');

                    if needs_quotes {
                        let escaped = h
                            .replace('\\', "\\\\")
                            .replace('"', "\\\"")
                            .replace('\n', "\\n")
                            .replace('\r', "\\r")
                            .replace('\t', "\\t");
                        format!("\"{}\"", escaped)
                    } else {
                        h.to_string()
                    }
                })
                .collect();

            iter.for_each(|rec| {
                buffered_writer
                    .write(b"\n[[Rows]]\n")
                    .better_expect("ERROR: Failed to write into output file.", verbose);

                let record = into_byte_record(rec);

                headers.iter().zip(record.iter()).for_each(|(h, v)| {
                    esc_buf.clear();

                    if matches!(v, b"true" | b"false")
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
                    buffered_writer.write(h.as_bytes()).better_expect(
                        "ERROR: Failed to write key for key-value pair into output file.",
                        verbose,
                    );

                    buffered_writer
                        .write(b" = ")
                        .better_expect("ERROR: Failed to write '=' into output file.", verbose);

                    buffered_writer.write(esc_buf.as_slice()).better_expect(
                        "ERROR: Failed to write value for key-value pair into output file.",
                        verbose,
                    );

                    buffered_writer.write(b"\n").better_expect(
                        "ERROR: Failed to write a newline into the output file.",
                        verbose,
                    );
                });
            });
        }

        _ => {}
    }
}
