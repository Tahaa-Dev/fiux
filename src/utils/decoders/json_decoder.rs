use std::{fs::File, io::BufReader};

use crate::utils::{BetterExpect, DataTypes, WriterStreams};

#[inline]
pub fn json_decoder(
    reader: serde_json::Deserializer<serde_json::de::IoRead<BufReader<File>>>,
    verbose: bool,
) -> WriterStreams<impl Iterator<Item = DataTypes>> {
    let iter = reader.into_iter::<serde_json::Value>().map(move |obj| {
        DataTypes::Json(
            obj.better_expect("ERROR: Invalid JSON values in input JSON file.", verbose),
        )
    });

    WriterStreams::Values { iter }
}
