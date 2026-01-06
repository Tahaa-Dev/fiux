/*!
**fiox: The *fastest* multi-format file converter CLI tool**

fiox provides intuitive file conversion similar to `mlr`'s `cat` verb but with a few advantages over it:

1. Auto format detection through extension
2. Better error-handling with recoverable errors, logging options, comprehensive logs and a `validate` command for debugging
3. 5× - 6× faster than `mlr`'s `cat` verb

---

## Subcommands & flags

### 1. Convert:

##### Description

- Convert from input file to output file
- If any item in the input file is invalid, the item is skipped and fiox logs an error message
- Only panics on read / write failure
##### Usage

Basic usage:

```sh
fiox convert <INPUT> -o <OUTPUT>
```

##### Arguments

1. Input: Input file to be converted, will panic if it doesn't exist or if its extension is not supported.
2. Output: `--output` / `-o` flag, file to write output to, will panic only if its extension is not supported, will create the file if it doesn't exist.

##### Flags (options)

1. `--append` / `-a`: fiox overwrites existing data in the output file by default, this flag makes it append to it instead. **WARNING:** This flag can lead to corrupted output with some formats like JSON.
2. `--parse-numbers` / `-p`: Flag to make fiox parse numbers in output when converted from CSV.
3. `--input-delimiter` / `--output-delimiter`: Flags that make fiox ignore file extension and treat them as CSV with the specified delimiter

---

### 2. Validate

##### Description

- Test if input file is valid
- Logs a detailed error message if an item is invalid then continues to validate the rest of the items
- Only panics on read failure
##### Usage

Basic usage:

```sh
fiox validate <INPUT>
```

##### Arguments

Input: File to be validated, will panic if it doesn't exist

##### Flags (options)

`--delimiter` / `-d`: Flag that makes fiox ignore file extension and treat the file as a CSV with the specified delimiter

### 3. `--log-file` / `-l` global flag

Flag for specifying a file to write logs to instead of printing them to stderr, preferably a Markdown file.

##### Usage

```sh
fiox validate <BROKEN_FILE> -l err.md
```

---

## Examples

```sh
# Convert with input delimiter
fiox convert input.psv --input-delimiter '|' -o output.csv

# Convert with output delimiter
fiox convert input.csv -o output.ssv --output-delimiter ';'

# Convert with append and parse numbers in output
fiox convert input.csv -o output.ndjson -a -p

# Convert with log file
fiox convert broken.ndjson -o output.toml -l err.md

# Validate with log file
fiox validate broken.json -l err.md

# Validate with delimiter
fiox validate input.psv -d '|'
```
*/

mod utils;
use clap::Parser;
use resext::*;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind as EK};
use std::path::Path;
use std::process::exit;
use std::sync::LazyLock;
use utils::*;

pub(crate) const VERBOSE_HELP: &str = "Try to use `fiox validate <INPUT>` for more information";

pub(crate) static ARGS: LazyLock<FioxArgs> = LazyLock::new(FioxArgs::parse);

fn main() -> CtxResult<(), Error> {
    let args = &*ARGS;

    match &args.cmd {
        Commands::Convert {
            input,
            output,
            append,
            parse_numbers,
            input_delimiter,
            output_delimiter,
        } => {
            // Check if input exists
            throw_err_if!(
                !Path::new(&input).exists(),
                || format!(
                    "Error: Input file {} doesn't exist",
                    input.to_str().unwrap_or("input_file")
                ),
                1
            );

            let output_file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(*append)
                .open(output)
                .context("Failed to open output file.")?;

            let o_d: char;

            let now = std::time::Instant::now();

            let output_ext;
            if let Some(ch) = output_delimiter {
                output_ext = std::borrow::Cow::Borrowed("csv");
                o_d = *ch;
            } else {
                output_ext = output
                    .extension()
                    .ok_or_else(|| Error::new(EK::InvalidFilename, "Output file has no extension"))
                    .context("Failed to get output file's extension")?
                    .to_string_lossy();
                o_d = ',';
            }

            if let Some(ch) = input_delimiter {
                let data = csv_decoder::csv_decoder(csv_reader::csv_reader(input, *ch))
                    .context("FATAL: Deserialization failed")?;

                match_output(data, output_file, &output_ext, *parse_numbers, o_d)?;
            } else {
                let input_ext: &str = &input
                    .extension()
                    .ok_or_else(|| Error::new(EK::InvalidFilename, "Input file has no extension"))
                    .context("Failed to get input file's extension")?
                    .to_string_lossy();

                match input_ext {
                    "json" => {
                        let data = json_decoder::json_decoder(json_reader::json_reader(input))
                            .context("FATAL: Deserialization failed")?;

                        match_output(data, output_file, &output_ext, *parse_numbers, o_d)?;
                    }
                    "toml" => {
                        let data = toml_decoder::toml_decoder(toml_reader::toml_reader(input))
                            .context("FATAL: Deserialization failed")?;
                        match_output(data, output_file, &output_ext, *parse_numbers, o_d)?;
                    }
                    "csv" => {
                        let data = csv_decoder::csv_decoder(csv_reader::csv_reader(input, ','))
                            .context("FATAL: Deserialization failed")?;
                        match_output(data, output_file, &output_ext, *parse_numbers, o_d)?;
                    }
                    "ndjson" => {
                        let data =
                            ndjson_decoder::ndjson_decoder(ndjson_reader::ndjson_reader(input))
                                .context("FATAL: Deserialization failed")?;
                        match_output(data, output_file, &output_ext, *parse_numbers, o_d)?;
                    }
                    _ => {
                        let repo_link = "https://github.com/Tahaa-Dev/fiox";
                        eprintln!(
                            "FATAL: Input extension \"{}\" is not supported currently.\n Open an issue at {}",
                            input_ext, repo_link,
                        );
                        exit(1);
                    }
                };
            }

            println!("Finished in {:?}", now.elapsed());
        }

        Commands::Validate { input, delimiter } => {
            // Check if input exists
            if !Path::new(&input).exists() {
                eprintln!("ERROR: Input file doesn't exist for validation.");
                exit(1);
            }

            let i_d: char;

            let temp_ext;
            if let Some(ch) = delimiter {
                temp_ext = std::borrow::Cow::Borrowed("csv");
                i_d = *ch;
            } else {
                temp_ext = input
                    .extension()
                    .ok_or_else(|| Error::new(EK::InvalidFilename, "Output file has no extension"))
                    .context("Failed to get output file's extension")?
                    .to_string_lossy();
                i_d = ',';
            }

            let input_ext: &str = &temp_ext;

            match input_ext {
                "json" => json_validator::validate_json(input)?,
                "toml" => toml_validator::validate_toml(input)?,
                "csv" => csv_validator::validate_csv(input, i_d)?,
                "ndjson" => ndjson_validator::validate_ndjson(input)?,
                _ => {
                    let repo_link = "https://github.com/Tahaa-Dev/fiox";
                    eprintln!(
                        "ERROR: Input extension \"{}\" is not supported currently.\n Open an issue at {}",
                        input_ext, repo_link,
                    );
                    exit(1);
                }
            };

            println!("Input file [{}] is valid!", input.to_str().unwrap_or("input"));
        }
    }

    flush_logger()?;

    Ok(())
}

#[inline]
fn match_output(
    data: WriterStreams<impl Iterator<Item = CtxResult<DataTypes, Error>>>,
    output_file: std::fs::File,
    output_ext: &str,
    parse_numbers: bool,
    o_d: char,
) -> CtxResult<(), Error> {
    match output_ext {
        "json" => write_json::write_json(data, output_file, parse_numbers)
            .context("FATAL: Serialization failed")?,
        "toml" => toml_writer::toml_writer(data, output_file, parse_numbers)
            .context("FATAL: Serialization failed")?,
        "csv" => {
            csv_writer::csv_writer(data, output_file, o_d).context("FATAL: Serialization failed")?
        }
        "ndjson" => ndjson_writer::ndjson_writer(data, output_file, parse_numbers)
            .context("FATAL: Serialization failed")?,
        _ => {
            let repo_link = "https://github.com/Tahaa-Dev/fiox";
            eprintln!(
                "FATAL: Output extension \"{}\" is not supported currently.\n Open an issue at {}",
                output_ext, repo_link,
            );
            exit(1);
        }
    };

    Ok(())
}
