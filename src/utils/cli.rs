use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueHint::FilePath};

#[derive(Parser)]
#[command(
    author,
    version,
    about = "The fastest utility for converting between file formats.",
    long_about = r#"
fiox: The fastest streaming-first file conveter.

  • Supports JSON, NDJSON, TOML, CSV, TSV, PSV and more!

  • Formats are detected automatically based on file extension, except for custom 
    delimter CSV formats (e.g. TSV, PSV, etc.), which are detected with `--input-delimiter <DELIMITER>` and `--output-delimiter <DELIMITER>`.

  • if there are any bugs or any features you want, open an issue at: `https://github.com/Tahaa-Dev/fiox`.


╭────────────────·Examples·────────────────╮
│                                      ••• │
│ fiox convert data.json out.csv           │
│ fiox validate broken.ndjson --verbose    │
│ fiox convert big.csv big.json --verbose  │
│                                          │
╰──────────────────────────────────────────╯
"#
)]
pub struct FioxArgs {
    #[command(subcommand)]
    pub cmd: Commands,

    /// Argument for setting a Markdown (MD) file to export error logs to.
    #[arg(short, long, value_hint = FilePath, global = true)]
    pub log_file: Option<PathBuf>,
}

/// fiox subcommands
#[derive(Subcommand)]
pub enum Commands {
    /// Convert command that takes two positional arguments for input and output, takes one
    Convert {
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

        /// Argument for parsing numbers in manual TOML / JSON writers
        #[arg(short, long)]
        parse_numbers: bool,

        /// Argument for specifying delimiters for CSV / CSV-like input formats (e.g. TSV, PSV, etc.).
        /// This flag makes fiox ignore the extension and instead treat the file as a CSV
        /// with the specified delimiter instead of commas.
        #[arg(long)]
        input_delimiter: Option<char>,

        /// Argument for specifying delimiters for CSV / CSV-like output formats (e.g. TSV, PSV, etc.).
        /// This flag makes fiox ignore the extension and instead treat the file as a CSV
        /// with the specified delimiter instead of commas.
        #[arg(long)]
        output_delimiter: Option<char>,
    },

    /// Validate command for file format validation with one positional argument for the file
    Validate {
        /// path to the file to be validated
        #[arg(required = true, value_hint = FilePath)]
        input: PathBuf,

        /// Argument for specifying delimiters for CSV / CSV-like input formats (e.g. TSV, PSV, etc.).
        /// This flag makes fiox ignore the extension and instead treat the file as a CSV
        /// with the specified delimiter instead of commas.
        #[arg(short, long)]
        delimiter: Option<char>,
    },
}
