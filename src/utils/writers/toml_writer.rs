use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::PathBuf,
};

use crate::utils::{BetterExpect, WriterStreams, into_byte_record};

pub fn toml_writer(data_stream: WriterStreams, path: &PathBuf, verbose: bool) {
    let file = OpenOptions::new().write(true).open(path).better_expect(
        format!(
            "ERROR: Failed to open output file [{}] for writing.",
            path.to_str().unwrap_or("[output.toml]")
        )
        .as_str(),
        verbose,
    );

    let mut buffered_writer = BufWriter::new(file);

    match data_stream {
        WriterStreams::Values { iter } => {
            iter.for_each(|rec| {
                let toml_object = toml::Value::try_from(rec)
                    .better_expect("ERROR: Failed to serialize value into TOML", verbose);

                buffered_writer.write(
                    toml::to_string_pretty(&toml_object)
                    .better_expect("INTERNAL ERROR: Failed to turn TOML into bytes for writing (possible OOM or deeply nested data)!", true)
                    .as_bytes())
                    .better_expect(
                    format!("ERROR: Failed to write TOML into output file [{}].", 
                            path
                                .to_str()
                                .unwrap_or("[output.toml]"))
                            .as_str(), 
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
                buffered_writer.write(b"\n[[Rows]]\n").better_expect(
                    format!(
                        "ERROR: Failed to write into output file [{}].",
                        path.to_str().unwrap_or("[output.toml]")
                    )
                    .as_str(),
                    verbose,
                );

                let record = into_byte_record(rec);

                headers.iter().zip(record.iter()).for_each(|(h, v)| {
                    esc_buf.clear();

                    if matches!(v, b"true" | b"false") {
                        esc_buf = v.to_vec();
                    } else {
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
                            b'\n' => {
                                esc_buf.push(b'\\');
                                esc_buf.push(b'n');
                            }
                            _ => esc_buf.push(*byte),
                        });
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
    }
}
