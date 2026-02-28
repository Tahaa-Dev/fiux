use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    iter::from_fn,
};

use crate::utils::{CtxResult, CtxResultExt, DataTypes, Log, WriterStreams};

pub(crate) fn ndjson_decoder(
    mut reader: BufReader<File>,
) -> CtxResult<WriterStreams<impl Iterator<Item = CtxResult<DataTypes>>>> {
    let mut buf = Vec::new();
    let mut line_no = 0usize;

    let iter = from_fn(move || {
        loop {
            line_no += 1;
            buf.clear();
            let bytes = reader
                .read_until(b'\n', &mut buf)
                .context(format_args!("Failed to read line: {}", line_no))
                .log("[WARN]")
                .unwrap_or(None);

            if bytes == Some(0) {
                return None;
            } else if bytes == None {
                continue;
            } else {
                while buf.last() == Some(&b'\n') || buf.last() == Some(&b'\r') {
                    buf.pop();
                }

                if buf.is_empty() {
                    continue;
                };

                let ndjson_obj = serde_json::from_slice(buf.as_slice())
                    .map_err(|_| Error::new(std::io::ErrorKind::InvalidData, "Invalid NDJSON"))
                    .context("Failed to deserialize file")
                    .context(format_args!(
                        "Invalid NDJSON values in input file at line: {}",
                        line_no
                    ));
                return match ndjson_obj {
                    Ok(ok) => Some(Ok(DataTypes::Json(ok))),
                    Err(err) => Some(Err(err)),
                };
            }
        }
    });

    Ok(WriterStreams::Ndjson { values: iter })
}
