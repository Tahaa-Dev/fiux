use resext::ctx;
use std::io::{BufWriter, Write};

use crate::utils::{CtxResult, CtxResultExt, DataTypes, Log, WriterStreams, into_byte_record};

pub fn write_json(
    data_stream: WriterStreams<impl Iterator<Item = CtxResult<DataTypes>>>,
    file: std::fs::File,
    parse_numbers: bool,
) -> CtxResult<()> {
    let mut wtr = BufWriter::new(&file);

    match data_stream {
        WriterStreams::Values { iter } => {
            for obj in iter {
                let valid_obj = obj
                    .context("Failed to re-serialize object for writing")
                    .log("[WARN]")
                    .unwrap_or_else(|| DataTypes::Json(serde_json::json!({})));

                serde_json::to_writer_pretty(&mut wtr, &valid_obj)
                    .context("Failed to write object into output JSON file")?;

                writeln!(wtr).context("Failed to write newline")?;
            }
        }

        WriterStreams::Table { headers, iter } => {
            let mut esc_buf: Vec<u8> = Vec::with_capacity(10);

            wtr.write_all(b"[\n").context("Failed to write opening bracket")?;

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

            for (line_no, rec) in iter.enumerate() {
                let line = line_no + 1;
                if first_obj {
                    wtr.write_all(b"  {\n").context(ctx!(
                        "Failed to write opening curly brace for record: {}",
                        line
                    ))?;

                    first_obj = false;
                } else {
                    wtr.write_all(b",\n  {\n")
                        .context(ctx!("Failed to write bracket for record: {}", line))?;
                }

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
                            crate::utils::escape(*byte, &mut esc_buf);
                        });

                        esc_buf.push(b'"');
                    }

                    if first_value {
                        wtr.write_all(b"    \"")
                            .context(ctx!("Failed to write key in record: {}", line))?;

                        wtr.write_all(h.as_bytes())
                            .context(ctx!("Failed to write key in record: {}", line))?;

                        wtr.write_all(b"\": ")
                            .context(ctx!("Failed to write key in record: {}", line))?;

                        first_value = false;
                    } else {
                        wtr.write_all(b",\n    \"")
                            .context(ctx!("Failed to write key in record: {}", line))?;

                        wtr.write_all(h.as_bytes())
                            .context(ctx!("Failed to write key in record: {}", line))?;

                        wtr.write_all(b"\": ")
                            .context(ctx!("Failed to write key in record: {}", line))?;
                    }

                    wtr.write_all(esc_buf.as_slice())
                        .context(ctx!("Failed to write value in record: {}", line))?;
                }

                wtr.write_all(b"\n  }")
                    .context(ctx!("Failed to write closing curly brace for record: {}", line))?;
            }

            wtr.write_all(b"\n]").context("Failed to write closing bracket")?;
        }

        WriterStreams::Ndjson { values } => {
            wtr.write_all(b"[\n").context("Failed to write opening bracket")?;

            let mut first = true;

            for (idx, obj) in values.enumerate() {
                let idx = idx + 1;

                let obj = obj
                    .context("Failed to re-serialize object")
                    .log("[WARN]")
                    .unwrap_or_else(|| DataTypes::Json(serde_json::json!({})));

                if first {
                    serde_json::to_writer_pretty(&mut wtr, &obj)
                        .context(ctx!("Failed to write record: {}", idx))?;

                    first = false;
                } else {
                    wtr.write_all(b",\n")
                        .context(ctx!("Failed to write comma after record: {}", idx))?;

                    serde_json::to_writer_pretty(&mut wtr, &obj)
                        .context(ctx!("Failed to write record: {}", idx))?;
                }
            }

            wtr.write_all(b"\n]").context("Failed to write closing bracket")?;
        }
    }

    wtr.flush().context("Failed to flush final bytes")
}
