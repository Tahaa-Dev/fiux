use std::{
    fs::File,
    io::{BufReader, Error, ErrorKind as EK},
};

use resext::{CtxResult, ResExt};

use crate::utils::{DataTypes, WriterStreams};

#[inline]
pub(crate) fn json_decoder(
    reader: serde_json::Deserializer<serde_json::de::IoRead<BufReader<File>>>,
) -> CtxResult<WriterStreams<impl Iterator<Item = CtxResult<DataTypes, Error>>>, Error> {
    let iter = reader.into_iter::<serde_json::Value>().map(move |obj| {
        let obj = obj
            .map_err(|_| Error::new(EK::InvalidData, "Invalid JSON"))
            .context("Failed to deserialize file")
            .context("Invalid JSON data in input file");
        match obj {
            Ok(ok) => Ok(DataTypes::Json(ok)),
            Err(err) => Err(err),
        }
    });

    Ok(WriterStreams::Values { iter })
}
