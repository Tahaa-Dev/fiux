use std::io::Error;

use resext::{CtxResult, ResExt};

use crate::utils::{DataTypes, WriterStreams};

#[inline]
pub(crate) fn toml_decoder(
    content: Vec<u8>,
) -> CtxResult<WriterStreams<impl Iterator<Item = CtxResult<DataTypes, Error>>>, Error> {
    let iter = [content].into_iter().map(move |c| {
        let toml_val = toml::from_slice(c.as_slice())
            .map_err(|_| Error::new(std::io::ErrorKind::InvalidData, "Invalid TOML"))
            .context("Failed to deserialize file")
            .context("Invalid TOML values in input file");

        match toml_val {
            Ok(ok) => Ok(DataTypes::Toml(ok)),
            Err(err) => Err(err),
        }
    });

    Ok(WriterStreams::Values { iter })
}
