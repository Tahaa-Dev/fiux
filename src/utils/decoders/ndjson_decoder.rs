use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    iter::from_fn,
};

use resext::{CtxResult, ResExt};

use crate::utils::{DataTypes, WriterStreams};

pub fn ndjson_decoder(
    mut reader: BufReader<File>,
    verbose: bool,
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
                verbose,
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
                    .dyn_expect(|| 
                        format!("Failed to deserialize input file\nInvalid NDJSON values in input file at line: {}", line_no), 1, verbose
                    );

                return Some(Ok(DataTypes::Json(ndjson_obj)));
            }
        }
    });

    Ok(WriterStreams::Ndjson { values: iter })
}
