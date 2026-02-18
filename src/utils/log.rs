use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Error, Write},
    sync::{LazyLock, Mutex},
};

use owo_colors::OwoColorize;

use resext::resext;

#[resext(buf_size = 80, alias = CtxResult, delimiter = " -> ", alloc = true, include_variant = true)]
pub enum FiuxErr {
    Json(serde_json::Error),
    TomlDeserialize(toml::de::Error),
    TomlSerialize(toml::ser::Error),
    Csv(csv::Error),
    IoError(Error),
}

enum Logger {
    Stdout(BufWriter<std::io::Stdout>),
    File(BufWriter<File>),
}

static LOGGER: LazyLock<Mutex<Logger>> = LazyLock::new(|| {
    let args = &*crate::ARGS;

    match args.log_file {
        Some(path) => {
            let res = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path);

            match res {
                Ok(file) => Mutex::new(Logger::File(BufWriter::with_capacity(64 * 1024, file))),
                Err(err) => {
                    eprintln!("{} {}", "[WARN]".yellow(), err);

                    Mutex::new(Logger::Stdout(BufWriter::with_capacity(64 * 1024, std::io::stdout())))
                }
            }
        }

        None => Mutex::new(Logger::Stdout(BufWriter::with_capacity(64 * 1024, std::io::stdout()))),
    }
});

impl std::io::Write for Logger {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Logger::Stdout(out) => out.write(buf),
            Logger::File(file) => file.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Logger::Stdout(out) => out.flush(),
            Logger::File(file) => file.flush(),
        }
    }
}

pub trait Log<T> {
    fn log(self, level: &str) -> CtxResult<Option<T>>;
}

impl<T> Log<T> for CtxResult<T> {
    fn log(self, level: &str) -> CtxResult<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(err) => {
                let mut wtr = LOGGER.lock().map_err(|e| std::io::Error::other(format!("{}", e))).context("Failed to lock logger")?;

                write!(&mut wtr, "{} {}", level.yellow(), err).context("Failed to write log")?;

                Ok(None)
            }
        }
    }
}

pub fn flush_logger(msg: &str) -> CtxResult<()> {
    let mut wtr = LOGGER.lock().map_err(|e| std::io::Error::other(format!("{}", e))).context("Failed to lock logger")?;

    write!(&mut wtr, "{}", msg).context("Failed to write status message")?;

    wtr.flush().context("Failed to flush logger")
}
