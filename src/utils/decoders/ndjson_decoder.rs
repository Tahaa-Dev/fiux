use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    iter::from_fn,
};

use resext::{CtxResult, ResExt};

use crate::utils::{DataTypes, WriterStreams};

pub(crate) fn ndjson_decoder(
    mut reader: BufReader<File>,
) -> CtxResult<WriterStreams<impl Iterator<Item = CtxResult<DataTypes, Error>>>, Error> {
    let mut buf = Vec::new();
    let mut line_no = 0usize;

    let iter = from_fn(move || {
        loop {
            line_no += 1;
            buf.clear();
            let bytes = reader.read_until(b'\n', &mut buf).dyn_expect(
                || format!("Failed to read line: {} in input file", line_no),
                1,
                true,
            );

            if bytes == 0 {
                return None;
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
                    .with_context(|| {
                        format!("Invalid NDJSON values in input file at line: {}", line_no)
                    });
                if ndjson_obj.is_err() {
                    return Some(
                        Err(unsafe { ndjson_obj.unwrap_err_unchecked() })
                            .context(crate::VERBOSE_HELP),
                    );
                } else {
                    return Some(Ok(DataTypes::Json(unsafe { ndjson_obj.unwrap_unchecked() })));
                }
            }
        }
    });

    Ok(WriterStreams::Ndjson { values: iter })
}
