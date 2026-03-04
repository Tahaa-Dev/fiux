use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueHint::FilePath};

static LONG_ABT: &str = r#"
fiux - The fastest streaming-first file conveter.

 -> Supports JSON, NDJSON, TOML, CSV, TSV, PSV and more!

 -> Formats are detected automatically based on file extension, except for custom 
   delimter CSV formats (e.g. TSV, PSV, etc.), which are detected with `--input-delimiter <DELIMITER>` and `--output-delimiter <DELIMITER>`.

 -> if there are any bugs or any features you want, open an issue at: `https://github.com/Tahaa-Dev/fiux`.


 ╭────────────────·Examples·─────────────────╮
 │                                       ••• │
 │ fiux convert data.json -o out.csv         │
 │ fiux validate broken.ndjson --delimiter   │
 │ fiux convert big.csv -o big.json --append │
 │                                           │
 ╰───────────────────────────────────────────╯
"#;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "The fastest streaming-first file conveter.",
    long_about = LONG_ABT
)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,

    /// Argument for setting a file to export error logs to.
    #[arg(short, long, value_hint = FilePath, global = true)]
    pub log_file: Option<PathBuf>,
}

/// fiux subcommands
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
        /// This flag makes fiux ignore the extension and instead treat the file as a CSV
        /// with the specified delimiter instead of commas.
        #[arg(long)]
        input_delimiter: Option<char>,

        /// Argument for specifying delimiters for CSV / CSV-like output formats (e.g. TSV, PSV, etc.).
        /// This flag makes fiux ignore the extension and instead treat the file as a CSV
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
        /// This flag makes fiux ignore the extension and instead treat the file as a CSV
        /// with the specified delimiter instead of commas.
        #[arg(short, long)]
        delimiter: Option<char>,
    },
}
