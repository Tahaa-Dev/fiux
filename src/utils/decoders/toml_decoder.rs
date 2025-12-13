use crate::utils::{DataTypes, WriterStreams};

pub fn toml_decoder(content: toml::Value) -> WriterStreams {
    WriterStreams::Values { iter: Box::new(std::iter::once(DataTypes::Toml(content))) }
}
