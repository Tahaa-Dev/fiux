use std::{
    fs::File,
    io::{BufWriter, Error, Write},
};

use resext::{CtxResult, ErrCtx, ResExt};
use serde_json::Value;

use crate::utils::{DataTypes, WriterStreams, escape, into_byte_record};

#[inline]
pub(crate) fn ndjson_writer(
    data_stream: WriterStreams<impl Iterator<Item = CtxResult<DataTypes, Error>>>,
    file: File,
    parse_numbers: bool,
) -> CtxResult<(), Error> {
    let mut writer = BufWriter::new(file);

    match data_stream {
        WriterStreams::Values { iter } => {
            for (line_no, item) in iter.enumerate() {
                let line_no = line_no + 1;
                let json = serde_json::to_value(
                    item.context("Failed to re-serialize record for writing").unwrap_or_else(
                        |e: ErrCtx<Error>| {
                            crate::utils::log_err(&e)
                                .unwrap_or_else(|err| eprintln!("{}\n{}", err, &e));

                            DataTypes::Json(serde_json::json!({}))
                        },
                    ),
                )
                .context("Failed to re-serialize record")
                .context("Invalid NDJSON values in input file")
                .unwrap_or_else(|e: ErrCtx<serde_json::Error>| {
                    crate::utils::log_err(&e).unwrap_or_else(|err| eprintln!("{}\n{}", err, &e));

                    serde_json::json!({})
                });

                if let Value::Array(arr) = json {
                    for (idx, obj) in arr.iter().enumerate() {
                        let idx = idx + 1;
                        serde_json::to_writer(&mut writer, obj)
                            .map_err(|_| {
                                Error::new(std::io::ErrorKind::WriteZero, "Failed to write")
                            })
                            .with_context(|| format!("Failed to write object: {}", idx))?;

                        writeln!(writer).with_context(|| {
                            format!("Failed to write newline delimiter after object: {}", idx)
                        })?;
                    }
                } else if let Value::Object(_) = json {
                    serde_json::to_writer(&mut writer, &json)
                        .map_err(|_| Error::new(std::io::ErrorKind::WriteZero, "Failed to write"))
                        .with_context(|| {
                            format!("Failed to write NDJSON object: {} into output file", line_no)
                        })?;

                    writeln!(writer).with_context(|| {
                        format!("Failed to write newline delimiter after object: {}", line_no)
                    })?;
                }
            }
        }

        WriterStreams::Table { headers, iter } => {
            let mut esc_buf: Vec<u8> = Vec::with_capacity(10);

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

            for (line_no, rec) in iter.enumerate() {
                let line_no = line_no + 1;
                writer.write(b"{").with_context(|| {
                    format!(
                        "Failed to write opening curly brace for object: {} into output file",
                        line_no
                    )
                })?;

                let mut first_value = true;

                let record = into_byte_record(rec)
                    .context("Failed to re-serialize object for writing")
                    .unwrap_or_else(|e: ErrCtx<Error>| {
                        crate::utils::log_err(&e)
                            .unwrap_or_else(|err| eprintln!("{}\n{}", err, &e));
                        csv::ByteRecord::with_capacity(0, 0)
                    });

                for (h, v) in headers.iter().zip(record.iter()) {
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
                        v.iter().for_each(|byte| {
                            escape(*byte, &mut esc_buf);
                        });
                        esc_buf.push(b'"');
                    }

                    if first_value {
                        write!(&mut writer, "\"{}\": ", &h).with_context(|| format!("Failed to write key in record: {}", line_no))?;

                        first_value = false;
                    } else {
                        write!(&mut writer, ", \"{}\": ", &h).with_context(|| format!("Failed to write key in record: {}", line_no))?;
                    }

                    writer.write_all(esc_buf.as_slice()).with_context(|| format!("Failed to write value in record: {}", line_no))?;
                }

                writer.write_all(b"}\n").with_context(|| format!(
                    "Failed to write closing curly brace for record: {}", line_no
                ))?;
            }

            writer.flush().context("Failed to flush final bytes into output file")?;
        }

        WriterStreams::Ndjson { values } => {
            for (line_no, item) in values.enumerate() {
                let json = item
                    .context("Failed to re-serialize record for writing")
                    .unwrap_or_else(|e: ErrCtx<Error>| {
                        crate::utils::log_err(&e)
                            .unwrap_or_else(|err| eprintln!("{}\n{}", err, &e));
                        DataTypes::Json(serde_json::json!({}))
                    });

                serde_json::to_writer(&mut writer, &json)
                    .map_err(|_| Error::new(std::io::ErrorKind::WriteZero, "Failed to write"))
                    .with_context(|| format!("Failed to write NDJSON object: {}", line_no + 1))?;
            }
        }
    }

    Ok(())
}
