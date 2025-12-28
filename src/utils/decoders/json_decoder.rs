use std::{
    fs::File,
    io::{BufReader, Error},
};

use resext::{CtxResult, ResExt};

use crate::utils::{DataTypes, WriterStreams};

#[inline]
pub fn json_decoder(
    reader: serde_json::Deserializer<serde_json::de::IoRead<BufReader<File>>>,
    verbose: bool,
) -> CtxResult<WriterStreams<impl Iterator<Item = CtxResult<DataTypes, Error>>>, Error> {
    let iter = reader.into_iter::<serde_json::Value>().map(move |obj| {
        let obj = obj.context("Invalid JSON data in input file")?;
        Ok(DataTypes::Json(obj))
    });

    Ok(WriterStreams::Values { iter })
}
