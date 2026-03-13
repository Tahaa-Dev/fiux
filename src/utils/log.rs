use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Error, Write},
    sync::{LazyLock, Mutex},
};

use owo_colors::OwoColorize;

use resext::resext;

#[resext(buf_size = 28, alias = CtxResult, delimiter = " -> ", alloc = true, include_variant = true)]
pub enum FiuxErr {
    Json(serde_json::Error),
    TomlDeserialize(toml::de::Error),
    TomlSerialize(toml::ser::Error),
    Csv(csv::Error),
    IoError(Error),
    Custom(String),
}

static LOGGER: LazyLock<Mutex<BufWriter<File>>> = LazyLock::new(|| {
    let args = &*crate::ARGS;
    let mut open = OpenOptions::new();

    open.create(true).append(true);

    match &args.log_file {
        Some(path) => {
            let res = open.open(path);

            match res {
                Ok(file) => Mutex::new(BufWriter::with_capacity(64 * 1024, file)),
                Err(err) => {
                    eprintln!("{} {}", "[WARN]".yellow(), err);

                    Mutex::new(BufWriter::with_capacity(
                        64 * 1024,
                        open.open("fiux.log")
                            .inspect_err(|e| {
                                eprintln!(
                                    "{} Failed to open log file: ./fiux.log\nError: {}",
                                    "[FATAL]".red().bold(),
                                    e
                                );
                                std::process::exit(1);
                            })
                            .unwrap(),
                    ))
                }
            }
        }

        None => Mutex::new(BufWriter::with_capacity(
            64 * 1024,
            open.open("fiux.log")
                .inspect_err(|e| {
                    eprintln!(
                        "{} Failed to open log file: ./fiux.log\nError: {}",
                        "[FATAL]".red().bold(),
                        e
                    );
                    std::process::exit(1);
                })
                .unwrap(),
        )),
    }
});

pub trait Log<T> {
    fn log(self, level: &str) -> Option<T>;
}

impl<T> Log<T> for CtxResult<T> {
    #[inline]
    fn log(self, level: &str) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                let wtr = LOGGER.lock();

                let mut wtr = match wtr {
                    Ok(ok) => ok,
                    Err(err) => {
                        eprintln!("{}", err);
                        return None;
                    }
                };

                let res =
                    writeln!(&mut wtr, "{} {}", level.yellow(), err).context("Failed to write log");

                let _ = writeln!(&mut wtr);

                match res {
                    Ok(_) => {}
                    Err(err) => eprintln!("{}", err),
                }

                None
            }
        }
    }
}

#[inline]
pub fn flush_logger(msg: &str) -> CtxResult<()> {
    let mut wtr = LOGGER
        .lock()
        .map_err(|e| std::io::Error::other(format!("{}", e)))
        .context("Failed to lock logger")?;

    wtr.write_all(msg.as_bytes()).context("Failed to write status message")?;

    wtr.flush().context("Failed to flush logger")
}
