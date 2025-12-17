use std::{fs::File, io::BufReader};

use crate::utils::{BetterExpect, DataTypes, WriterStreams};

#[inline]
pub fn csv_decoder(
    mut reader: csv::Reader<BufReader<File>>,
    verbose: bool,
) -> WriterStreams<impl Iterator<Item = DataTypes>> {
    let headers = reader
        .headers()
        .better_expect("ERROR: Failed to read input file headers.", verbose)
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let iter = reader.into_byte_records().enumerate().map(move |(line_no, rec)| {
        DataTypes::Csv(rec.better_expect(
            format!("ERROR: Serialization error in input file at record [{}].", line_no).as_str(),
            verbose,
        ))
    });

    WriterStreams::Table { headers, iter }
}
