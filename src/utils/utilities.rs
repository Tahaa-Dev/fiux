use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Error, Write},
    sync::{LazyLock, Mutex},
};

use csv::ByteRecord;
use resext::{CtxResult, ErrCtx, ResExt};
use serde::Serialize;

static LOGGER: LazyLock<Option<Mutex<BufWriter<File>>>> = LazyLock::new(|| {
    let args = &*crate::ARGS;

    args.log_file.as_ref().map(|path| {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .better_expect("FATAL: Failed to open error logging file", 1, true);

        Mutex::new(BufWriter::with_capacity(256, file))
    })
});

pub(crate) enum WriterStreams<I>
where
    I: Iterator<Item = CtxResult<DataTypes, Error>>,
{
    Values { iter: I },

    Table { headers: Vec<String>, iter: I },

    Ndjson { values: I },
}

pub(crate) enum DataTypes {
    Json(serde_json::Value),

    Toml(toml::Value),

    Csv(ByteRecord),
}

impl Serialize for DataTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            DataTypes::Json(j) => j.serialize(serializer),
            DataTypes::Toml(t) => t.serialize(serializer),
            DataTypes::Csv(_) => unreachable!(),
        }
    }
}

pub(crate) fn into_byte_record(brec: CtxResult<DataTypes, Error>) -> CtxResult<ByteRecord, Error> {
    match brec.context("Failed to unwrap record")? {
        DataTypes::Csv(csv) => Ok(csv),
        _ => unreachable!(),
    }
}

static NEEDS_ESCAPE: [bool; 256] = {
    let mut table = [false; 256];
    table[b'\\' as usize] = true;
    table[b'"' as usize] = true;
    table[b'\n' as usize] = true;
    table[b'\r' as usize] = true;
    table[b'\t' as usize] = true;
    table
};

#[inline]
pub(crate) fn escape(byte: u8, output: &mut Vec<u8>) {
    if NEEDS_ESCAPE[byte as usize] {
        output.reserve_exact(2);
        output.push(b'\\');
        match byte {
            b'\\' => output.push(b'\\'),
            b'"' => output.push(b'"'),
            b'\n' => output.push(b'n'),
            b'\r' => output.push(b'r'),
            b'\t' => output.push(b't'),
            _ => unreachable!(),
        }
    } else {
        output.push(byte);
    }
}

pub(crate) fn log_err<E: std::error::Error>(err: &ErrCtx<E>) -> CtxResult<(), Error> {
    if let Some(wtr) = &*LOGGER {
        let mut wtr = wtr
            .lock()
            .map_err(|_| Error::other("Failed to lock"))
            .context("FATAL: Failed to lock log file")?;

        writeln!(wtr, "{}", err).context("FATAL: Failed to write error to log")?;

        writeln!(wtr, "---").context("FATAL: Failed to write divider")?;
    } else {
        eprintln!("{}", err);
    }

    Ok(())
}

pub(crate) fn flush_logger() -> CtxResult<(), Error> {
    if let Some(wtr) = &*LOGGER {
        wtr.lock()
            .map_err(|_| Error::other("Failed to lock"))
            .context("Failed to lock logger")?
            .flush()
            .context("Failed to flush logger")?;
    }
    Ok(())
}
