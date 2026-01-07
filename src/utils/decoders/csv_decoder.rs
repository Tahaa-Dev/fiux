use std::{
    fs::File,
    io::{BufReader, Error, ErrorKind as EK},
};

use resext::{CtxResult, ResExt};

use crate::utils::{DataTypes, WriterStreams};

#[inline]
pub(crate) fn csv_decoder(
    mut reader: csv::Reader<BufReader<File>>,
) -> CtxResult<WriterStreams<impl Iterator<Item = CtxResult<DataTypes, Error>>>, Error> {
    let headers = reader
        .headers()
        .map_err(|_| Error::new(EK::InvalidData, "Input CSV file headers are missing"))
        .context("Failed to read input file headers")?
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let iter = reader.into_byte_records().enumerate().map(move |(line_no, rec)| {
        let record = rec
            .map_err(|_| Error::new(EK::InvalidData, "Invalid CSV record"))
            .context("Failed to deserialize file")
            .with_context(|| format!("Invalid CSV data in input file at line: {}", line_no + 1));

        match record {
            Ok(ok) => Ok(DataTypes::Csv(ok)),
            Err(err) => Err(err),
        }
    });

    Ok(WriterStreams::Table { headers, iter })
}
