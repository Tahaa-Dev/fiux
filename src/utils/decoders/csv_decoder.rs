use std::{
    fs::File,
    io::{BufReader, Error, ErrorKind as EK},
};

use resext::{CtxResult, ResExt};

use crate::utils::{DataTypes, WriterStreams};

#[inline]
pub fn csv_decoder(
    mut reader: csv::Reader<BufReader<File>>,
    verbose: bool,
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
            .with_context(|| format!("Invalid CSV data in input file at line: {}", line_no + 1))?;

        Ok(DataTypes::Csv(record))
    });

    if verbose {
        println!(
            "Input file is valid and was deserialized successfully.\nProcessing and writing will start."
        );
    }

    Ok(WriterStreams::Table { headers, iter })
}
