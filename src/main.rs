mod utils;
use clap::Parser;
use resext::*;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind as EK};
use std::path::Path;
use std::process::exit;
use utils::*;

pub const VERBOSE_HELP: &str = "Try to use `fiox validate <INPUT> -v` for more information";

fn main() -> CtxResult<(), Error> {
    let args: FioxArgs = cli::FioxArgs::parse();

    match args.cmd {
        Commands::Convert { input, output, append, parse_numbers } => {
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
                .append(append)
                .open(&output)
                .context("Failed to open output file.")?;

            let input_ext: &str = &input
                .extension()
                .ok_or_else(|| Error::new(EK::InvalidFilename, "Input file has no extension"))
                .context("Failed to get input file's extension")?
                .to_string_lossy();

            let output_ext: &str = &output
                .extension()
                .ok_or_else(|| Error::new(EK::InvalidFilename, "Output file has no extension"))
                .context("Failed to get output file's extension")?
                .to_string_lossy();

            let now = std::time::Instant::now();

            match input_ext {
                "json" => {
                    let data = json_decoder::json_decoder(json_reader::json_reader(&input))
                        .context("FATAL: Deserialization failed")?;

                    match_output(data, output_file, output_ext, parse_numbers)?;
                }
                "toml" => {
                    let data = toml_decoder::toml_decoder(toml_reader::toml_reader(&input))
                        .context("FATAL: Deserialization failed")?;
                    match_output(data, output_file, output_ext, parse_numbers)?;
                }
                "csv" => {
                    let data = csv_decoder::csv_decoder(csv_reader::csv_reader(&input))
                        .context("FATAL: Deserialization failed")?;
                    match_output(data, output_file, output_ext, parse_numbers)?;
                }
                "ndjson" => {
                    let data = ndjson_decoder::ndjson_decoder(ndjson_reader::ndjson_reader(&input))
                        .context("FATAL: Deserialization failed")?;
                    match_output(data, output_file, output_ext, parse_numbers)?;
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

            println!("Finished in {:?}", now.elapsed());
        }

        Commands::Validate { input, verbose } => {
            // Check if input exists
            if !Path::new(&input).exists() {
                eprintln!("ERROR: Input file doesn't exist for validation.");
                exit(1);
            }

            let input_ext: &str = &input
                .extension()
                .ok_or_else(|| Error::new(EK::InvalidFilename, "Input file has no extension"))
                .context("Failed to get input file's extension")?
                .to_string_lossy();

            match input_ext {
                "json" => json_validator::validate_json(&input, verbose)?,
                "toml" => toml_validator::validate_toml(&input, verbose)?,
                "csv" => csv_validator::validate_csv(&input, verbose)?,
                "ndjson" => ndjson_validator::validate_ndjson(&input, verbose)?,
                _ => {
                    let repo_link = "https://github.com/Tahaa-Dev/fiox";
                    eprintln!(
                        "ERROR: Input extension \"{}\" is not supported currently.\n Open an issue at {}",
                        input_ext, repo_link,
                    );
                    exit(1);
                }
            };
            println!("Input file [{}] is valid!", input.to_str().unwrap_or("inputFile"));
        }
    }

    Ok(())
}

#[inline]
fn match_output(
    data: WriterStreams<impl Iterator<Item = CtxResult<DataTypes, Error>>>,
    output_file: std::fs::File,
    output_ext: &str,
    parse_numbers: bool,
) -> CtxResult<(), Error> {
    match output_ext {
        "json" => write_json::write_json(data, output_file, parse_numbers)
            .context("FATAL: Serialization failed")?,
        "toml" => toml_writer::toml_writer(data, output_file, parse_numbers)
            .context("FATAL: Serialization failed")?,
        "csv" => {
            csv_writer::csv_writer(data, output_file).context("FATAL: Serialization failed")?
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
