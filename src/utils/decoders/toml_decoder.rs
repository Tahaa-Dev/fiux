use crate::utils::{DataTypes, WriterStreams};

#[inline]
pub fn toml_decoder(content: toml::Value) -> WriterStreams<impl Iterator<Item = DataTypes>> {
    let iter = [content].into_iter().map(DataTypes::Toml);
    WriterStreams::Values { iter }
}
