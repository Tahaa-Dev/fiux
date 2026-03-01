use std::io::{BufWriter, Write};

use toml::{Value, map::Map};

use crate::utils::{CtxResult, CtxResultExt, DataTypes, Log, WriterStreams, escape, into_byte_record};

#[inline]
pub(crate) fn toml_writer(
    data_stream: WriterStreams<impl Iterator<Item = CtxResult<DataTypes>>>,
    file: std::fs::File,
    parse_numbers: bool,
) -> CtxResult<()> {

    let mut buffered_writer = BufWriter::new(file);

    match data_stream {

        WriterStreams::Values { iter } => {
            for item in iter {
                let obj = Value::try_from(
                    item.context("Failed to re-serialize object")
                    .log("[WARN]")
                    .unwrap_or_else(|| DataTypes::Json(serde_json::json!({})))
                )
                    .context("Failed to re-serialize object")
                    .log("[WARN]")
                    .unwrap_or_else(|| Value::Table(Map::new()));

                if let Value::Array(_) = obj {
                    let mut map = Map::with_capacity(1);

                    map.insert("Array".to_string(), obj);

                    buffered_writer.write_all(toml::to_string_pretty(&Value::Table(map))
                        .context("Failed to serialize TOML table")?
                        .as_bytes()
                    )
                        .context("Failed to write TOML table")?;

                } else {

                    buffered_writer.write_all(toml::to_string_pretty(&obj)
                        .context("Failed to serialize TOML table")?
                        .as_bytes()
                    )
                        .context("Failed to write TOML table")?;
                }
            }
            buffered_writer.flush().context("Failed to flush writer")?;
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

            let mut first_row = true;

            for (line_no, rec) in iter.enumerate() {
                let line_no = line_no + 1;
                if !first_row {
                    buffered_writer.write_all(b"\n[[Rows]]\n").context(
                        format_args!("Failed to write key for row: {}", line_no)
                    )?;

                } else {

                    buffered_writer.write_all(b"[[Rows]]\n").context(
                        format_args!("Failed to write key for row: {}", line_no)
                    )?;

                    first_row = false;
                }

                let record = into_byte_record(rec)
                    .context("Failed to re-serialize object")
                    .log("[WARN]")
                    .unwrap_or_default();

                for (h, v) in headers.iter().zip(record.iter()) {
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
                        v.iter().for_each(|byte| {
                            escape(*byte, &mut esc_buf);
                        });
                        esc_buf.push(b'"');
                    }

                    write!(&mut buffered_writer, "{} = ", &h)
                        .context(format_args!("Failed to write key in record: {}", line_no))?;

                    buffered_writer
                        .write_all(esc_buf.as_slice())
                        .context(format_args!("Failed to write value in record: {}", line_no))?;

                    writeln!(&mut buffered_writer).context("Failed to write newline")?;
                }
            }

            buffered_writer.flush().context("Failed to flush writer")?;
        }

        WriterStreams::Ndjson { values } => {
            let mut first = true;
            for (rec_no, rec) in values.enumerate() {
                let rec_no = rec_no + 1;

                let obj = Value::try_from(
                    rec.context("Failed to re-serialize object")
                    .log("[WARN]")
                    .unwrap_or_else(|| DataTypes::Json(serde_json::json!({})))
                )
                    .context("Failed to re-serialize object")
                    .log("[WARN]")
                    .unwrap_or_else(|| Value::Table(Map::new()));

                if first {
                    buffered_writer
                        .write_all(b"[[Array]]\n")
                        .context(format_args!("Failed to write array key: {}", rec_no))?;

                    first = false;
                } else {
                    buffered_writer
                        .write_all(b"\n[[Array]]\n")
                        .context(format_args!("Failed to write array key: {}", rec_no))?;
                }

                if let Value::Array(_) = obj {
                    let mut map = Map::with_capacity(1);
                    map.insert("Array".to_string(), obj);

                    buffered_writer.write_all(toml::to_string_pretty(&Value::Table(map))
                        .context("Failed to serialize TOML table")?
                        .as_bytes()
                    )
                        .context("Failed to write TOML table")?;
                } else {
                    buffered_writer.write_all(toml::to_string_pretty(&obj)
                        .context("Failed to serialize valid TOML table")?
                        .as_bytes()
                    )
                        .context("Failed to write TOML table")?;
                }

                buffered_writer.flush().context("Failed to flush writer")?;
            }
        }
    }

    Ok(())
}
