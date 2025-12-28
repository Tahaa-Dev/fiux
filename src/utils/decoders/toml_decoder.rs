use resext::CtxResult;

use crate::utils::{DataTypes, WriterStreams};

#[inline]
pub fn toml_decoder(
    content: toml::Value,
) -> CtxResult<
    WriterStreams<impl Iterator<Item = CtxResult<DataTypes, std::io::Error>>>,
    std::io::Error,
> {
    let iter = [content].into_iter().map(|c| Ok(DataTypes::Toml(c)));
    Ok(WriterStreams::Values { iter })
}
