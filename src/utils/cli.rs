use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueHint::FilePath};

#[derive(Parser)]
#[command(
    author,
    version,
    about = "A blazingly fast utility for handling files with different formats.",
    long_about = r#"
Fiox, a blazingly fast, streaming-first file handler.

• It supports JSON, TOML, CSV, and NDJSON.
• Formats are detected automatically based on file extension.
• Streams files instead of loading them into memory all at once.
• Supports file conversions and file validation.
• if there are any bugs or features you want, open an issue at [`https://github.com/Tahaa-Dev/fiox`].

Examples:
   _________________________________________
  |                                         |
  | fiox convert data.json out.csv          |
  | fiox validate broken.ndjson --verbose   |
  | fiox convert big.csv big.json --verbose |
  |_________________________________________|
"#
)]
pub struct FioxArgs {
    #[command(subcommand)]
    pub cmd: Commands,
}

/// Fiox subcommands
#[derive(Subcommand)]
pub enum Commands {
    /// Convert command that takes two positional arguments for input and output, takes one
    /// flag [`verbose`] which enables extra logs and soon backtraces.
    Convert {
        /// flag for extra debug logging
        #[arg(short, long)]
        verbose: bool,

        /// Argument for input file path
        #[arg(required = true, value_hint = FilePath)]
        input: PathBuf,

        /// Argument for output file path
        #[arg(short, long, required = true, value_hint = FilePath)]
        output: PathBuf,

        /// Argument to append into output file instead of overwriting it (WARNING: This
        /// can lead to unexpected output on some formats)
        #[arg(short, long)]
        append: bool,

        /// Argument for actually parsing numbers in manual TOML / JSON writers
        #[arg(short, long)]
        parse_numbers: bool,
    },

    /// Validate command for file format validation with one positional argument for the file
    /// to be checked and a flag [`verbose`] which enables extra logs, backtraces (coming soon!) and exact
    /// line errors.
    Validate {
        /// path to the file to be validated
        #[arg(required = true, value_hint = FilePath)]
        input: PathBuf,

        /// flag for extra debug logging
        #[arg(short, long)]
        verbose: bool,
    },
}
