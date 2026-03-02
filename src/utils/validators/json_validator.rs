use serde::de::IgnoredAny;
use serde_json::Deserializer;
use std::{fs::File, io::BufReader, path::PathBuf};

use crate::utils::{CtxResult, CtxResultErr, CtxResultExt, Log};

pub fn validate_json(path: &PathBuf) -> CtxResult<()> {
    let file = File::open(path)
        .context("Failed to validate file")
        .context(format_args!("Failed to open file: {}", &path.to_string_lossy()))?;

    let reader = BufReader::with_capacity(256 * 1024, file);

    let file_stream = Deserializer::from_reader(reader).into_iter::<IgnoredAny>();

    let mut res = Ok(());

    for item in file_stream {
        let opt = item.context("Invalid JSON values").log("[WARN]").is_none();

        if opt && res.is_ok() {
            res =
                Err(CtxResultErr::new("Input file is invalid", String::from("Invalid JSON data")));
        }
    }

    res
}
