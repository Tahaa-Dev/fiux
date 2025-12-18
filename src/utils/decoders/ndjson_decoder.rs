use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::from_fn,
};

use crate::utils::{BetterExpect, DataTypes, WriterStreams};

pub fn ndjson_decoder(
    mut reader: BufReader<File>,
    verbose: bool,
) -> WriterStreams<impl Iterator<Item = DataTypes>> {
    let mut buf = Vec::new();
    let mut line_no = 0usize;

    let iter = from_fn(move || {
        loop {
            line_no += 1;
            buf.clear();
            let bytes = reader.read_until(b'\n', &mut buf).better_expect(
                format!("ERROR: Failed to read line [{}] in input file.", line_no).as_str(),
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

                return Some(DataTypes::Json(
                    serde_json::from_slice(buf.as_slice()).better_expect(
                        format!("ERROR: Invalid NDJSON at line [{}] in input file.", line_no)
                            .as_str(),
                        verbose,
                    ),
                ));
            }
        }
    });

    WriterStreams::Values { iter }
}
