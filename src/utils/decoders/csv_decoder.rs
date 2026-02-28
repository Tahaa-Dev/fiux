use std::{fs::File, io::BufReader};

use crate::utils::{CtxResult, CtxResultExt, DataTypes, WriterStreams};

#[inline]
pub(crate) fn csv_decoder(
    mut reader: csv::Reader<BufReader<File>>,
) -> CtxResult<WriterStreams<impl Iterator<Item = CtxResult<DataTypes>>>> {
    let headers = reader
        .headers()
        .context("Failed to read input file headers")?
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let iter = reader.into_byte_records().enumerate().map(move |(line_no, rec)| {
        let record = rec.context(format_args!("Invalid CSV data at line: {}", line_no + 1));

        match record {
            Ok(ok) => Ok(DataTypes::Csv(ok)),
            Err(err) => Err(err),
        }
    });

    Ok(WriterStreams::Table { headers, iter })
}
