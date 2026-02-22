use std::{
    fs::File,
    io::BufReader,
};

use crate::utils::{CtxResult, CtxResultExt, DataTypes, WriterStreams};

#[inline]
pub(crate) fn json_decoder(
    reader: serde_json::Deserializer<serde_json::de::IoRead<BufReader<File>>>,
) -> CtxResult<WriterStreams<impl Iterator<Item = CtxResult<DataTypes>>>> {

    let iter = reader.into_iter::<serde_json::Value>().map(move |obj| {
        let obj = obj
            .context("Invalid JSON data in input file");

        match obj {
            Ok(ok) => Ok(DataTypes::Json(ok)),
            Err(err) => Err(err),
        }
    });

    Ok(WriterStreams::Values { iter })
}
