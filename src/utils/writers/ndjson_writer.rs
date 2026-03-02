use std::{
    fs::File,
    io::{BufWriter, Write},
};

use serde_json::Value;

use crate::utils::{
    CtxResult, CtxResultExt, DataTypes, Log, WriterStreams, escape, into_byte_record,
};

#[inline]
pub fn ndjson_writer(
    data_stream: WriterStreams<impl Iterator<Item = CtxResult<DataTypes>>>,
    file: File,
    parse_numbers: bool,
) -> CtxResult<()> {
    let mut writer = BufWriter::new(file);

    match data_stream {
        WriterStreams::Values { iter } => {
            for (line_no, item) in iter.enumerate() {
                let line_no = line_no + 1;

                let json = serde_json::to_value(
                    item.context("Failed to re-serialize record")
                        .log("[WARN]")
                        .unwrap_or_else(|| DataTypes::Json(serde_json::json!({}))),
                )
                .context("Failed to re-serialize record")
                .log("[WARN]")
                .unwrap_or_default();

                if let Value::Array(arr) = json {
                    for (idx, obj) in arr.iter().enumerate() {
                        let idx = idx + 1;

                        serde_json::to_writer(&mut writer, obj)
                            .context(format_args!("Failed to write object: {}", idx))?;

                        writeln!(writer).context("Failed to write newline")?;
                    }
                } else if let Value::Object(_) = json {
                    serde_json::to_writer(&mut writer, &json)
                        .context(format_args!("Failed to write object: {}", line_no))?;

                    writeln!(writer).context("Failed to write newline")?;
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

                writer
                    .write(b"{")
                    .context(format_args!("Failed to write bracket for object: {}", line_no))?;

                let mut first_value = true;

                let record = into_byte_record(rec)
                    .context("Failed to re-serialize object")
                    .log("[WARN]")
                    .unwrap_or_default();

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
                        write!(&mut writer, "\"{}\": ", &h)
                            .context(format_args!("Failed to write key in record: {}", line_no))?;

                        first_value = false;
                    } else {
                        write!(&mut writer, ", \"{}\": ", &h)
                            .context(format_args!("Failed to write key in record: {}", line_no))?;
                    }

                    writer
                        .write_all(esc_buf.as_slice())
                        .context(format_args!("Failed to write value in record: {}", line_no))?;
                }

                writer.write_all(b"}\n").context(format_args!(
                    "Failed to write closing curly brace for record: {}",
                    line_no
                ))?;
            }

            writer.flush().context("Failed to flush writer")?;
        }

        WriterStreams::Ndjson { values } => {
            for (line_no, item) in values.enumerate() {
                let json = item
                    .context("Failed to re-serialize record")
                    .log("[WARN]")
                    .unwrap_or_else(|| DataTypes::Json(serde_json::json!({})));

                serde_json::to_writer(&mut writer, &json)
                    .context(format_args!("Failed to write object: {}", line_no + 1))?;
            }
        }
    }

    Ok(())
}
