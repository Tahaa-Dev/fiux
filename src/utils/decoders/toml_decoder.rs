use crate::utils::{CtxResult, CtxResultExt, DataTypes, WriterStreams};


pub fn toml_decoder(
    content: Vec<u8>,
) -> CtxResult<WriterStreams<impl Iterator<Item = CtxResult<DataTypes>>>> {
    let iter = [content].into_iter().map(move |c| {
        let toml_val = toml::from_slice(c.as_slice()).context("Invalid TOML values in input file");

        match toml_val {
            Ok(ok) => Ok(DataTypes::Toml(ok)),
            Err(err) => Err(err),
        }
    });

    Ok(WriterStreams::Values { iter })
}
