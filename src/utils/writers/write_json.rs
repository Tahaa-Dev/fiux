use std::io::{BufWriter, Error, Write};

use resext::{CtxResult, ErrCtx, ResExt};

use crate::utils::{DataTypes, WriterStreams, into_byte_record};

#[inline]
pub(crate) fn write_json(
    data_stream: WriterStreams<impl Iterator<Item = CtxResult<DataTypes, Error>>>,
    file: std::fs::File,
    parse_numbers: bool,
) -> CtxResult<(), Error> {
    let mut buffered_writer = BufWriter::new(&file);

    match data_stream {
        WriterStreams::Values { iter } => {
            for obj in iter {
                let valid_obj = obj
                    .context("Failed to re-serialize object for writing")
                    .unwrap_or_else(|e: ErrCtx<Error>| {
                        crate::utils::log_err(&e)
                            .unwrap_or_else(|err| eprintln!("{}\n{}", err, &e));
                        DataTypes::Json(serde_json::json!({}))
                    });

                serde_json::to_writer_pretty(&mut buffered_writer, &valid_obj)
                    .map_err(|_| Error::other("Failed to write into file"))
                    .context("Failed to write object into output JSON file")
                    .context("This might be caused by invalid JSON")?;

                writeln!(buffered_writer).context("Failed to write newline into output file")?;
            }

            buffered_writer.flush().context("Failed to flush final bytes into output file")?;
        }

        WriterStreams::Table { headers, iter } => {
            let mut esc_buf: Vec<u8> = Vec::with_capacity(10);

            buffered_writer
                .write_all(b"[\n")
                .context("Failed to write opening bracket into output file")?;

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
                    buffered_writer.write_all(b"  {\n").with_context(|| {
                        format!(
                            "Failed to write opening curly brace for record: {} into output file",
                            line
                        )
                    })?;

                    first_obj = false;
                } else {
                    buffered_writer.write_all(b",\n  {\n").with_context(|| {
                        format!(
                            "Failed to write opening curly brace for record: {} into output file",
                            line
                        )
                    })?;
                }

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
                            crate::utils::escape(*byte, &mut esc_buf);
                        });

                        esc_buf.push(b'"');
                    }
                    if first_value {
                        write!(&mut buffered_writer, "    \"{}\": ", &h)
                            .with_context(|| format!("Failed to write key in record: {}", line))?;

                        first_value = false;
                    } else {
                        write!(&mut buffered_writer, ",\n    \"{}\": ", &h)
                            .with_context(|| format!("Failed to write key in record: {}", line))?;
                    }

                    buffered_writer
                        .write_all(esc_buf.as_slice())
                        .with_context(|| format!("Failed to write value in record: {}", line))?;
                }

                buffered_writer.write_all(b"\n  }").with_context(|| {
                    format!(
                        "Failed to write closing curly brace for record: {} into output file",
                        line
                    )
                })?;
            }

            buffered_writer
                .write_all(b"\n]")
                .context("Failed to write closing bracket into output file")?;

            buffered_writer.flush().context("Failed to flush final bytes into output file")?;
        }

        WriterStreams::Ndjson { values } => {
            buffered_writer
                .write_all(b"[\n")
                .context("Failed to write opening bracket into output file")?;

            let mut first = true;

            for (idx, obj) in values.enumerate() {
                let idx = idx + 1;

                let obj = obj.context("Failed to re-serialize object for writing").unwrap_or_else(
                    |e: ErrCtx<Error>| {
                        crate::utils::log_err(&e)
                            .unwrap_or_else(|err| eprintln!("{}\n{}", err, &e));
                        DataTypes::Json(serde_json::json!({}))
                    },
                );

                if first {
                    serde_json::to_writer_pretty(&mut buffered_writer, &obj)
                        .map_err(|_| Error::other("Failed to write into file"))
                        .with_context(|| {
                            format!("Failed to write record: {} into output JSON file", idx)
                        })
                        .context("Error might be caused by invalid NDJSON values in input file")?;

                    first = false;
                } else {
                    buffered_writer.write_all(b",\n").with_context(|| {
                        format!("Failed to write comma after record: {} into output file", idx)
                    })?;

                    serde_json::to_writer_pretty(&mut buffered_writer, &obj)
                        .map_err(|_| Error::other("Failed to write into file"))
                        .with_context(|| {
                            format!("Failed to write record: {} into output JSON file", idx)
                        })
                        .context("Error might be caused by invalid NDJSON values in input file")?;
                }
            }

            buffered_writer
                .write_all(b"\n]")
                .context("Failed to write closing bracket into output file")?;
        }
    }

    Ok(())
}
