mod utils;
use clap::Parser;
use resext::*;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind as EK};
use std::path::{Path, PathBuf};
use std::process::exit;
use utils::*;

fn main() -> CtxResult<(), Error> {
    let args: FioxArgs = cli::FioxArgs::parse();

    match args.cmd {
        Commands::Convert { verbose, input, output, append, parse_numbers } => {
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

            let (json, toml, csv, ndjson, idx) = get_data_stream(input_ext, &input, verbose);

            match idx {
                0 => match_output(json, output_file, verbose, output_ext, parse_numbers),
                1 => match_output(toml, output_file, verbose, output_ext, parse_numbers),
                2 => match_output(csv, output_file, verbose, output_ext, parse_numbers),
                3 => match_output(ndjson, output_file, verbose, output_ext, parse_numbers),
                _ => unreachable!(),
            };

            println!(
                "Finished converting {} -> {} in {:?}",
                input.to_str().unwrap_or("input file"),
                output.to_str().unwrap_or("output file"),
                now.elapsed()
            );
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
                "json" => json_validator::validate_json(&input, verbose),
                "toml" => toml_validator::validate_toml(&input, verbose),
                "csv" => csv_validator::validate_csv(&input, verbose),
                "ndjson" => ndjson_validator::validate_ndjson(&input, verbose),
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

fn get_data_stream(
    input_ext: &str,
    input: &PathBuf,
    verbose: bool,
) -> (
    WriterStreams<impl Iterator<Item = CtxResult<DataTypes, Error>>>,
    WriterStreams<impl Iterator<Item = CtxResult<DataTypes, Error>>>,
    WriterStreams<impl Iterator<Item = CtxResult<DataTypes, Error>>>,
    WriterStreams<impl Iterator<Item = CtxResult<DataTypes, Error>>>,
    u8,
) {
    let mut data1 = WriterStreams::Temp {};
    let mut data2 = WriterStreams::Temp {};
    let mut data3 = WriterStreams::Temp {};
    let mut data4 = WriterStreams::Temp {};
    let num;
    match input_ext {
        "json" => {
            data1 = json_decoder::json_decoder(json_reader::json_reader(input, verbose), verbose)
                .unwrap();
            num = 0;
        }
        "toml" => {
            data2 = toml_decoder::toml_decoder(toml_reader::toml_reader(input, verbose)).unwrap();
            num = 1;
        }
        "csv" => {
            data3 =
                csv_decoder::csv_decoder(csv_reader::csv_reader(input, verbose), verbose).unwrap();
            num = 2
        }
        "ndjson" => {
            data4 = ndjson_decoder::ndjson_decoder(
                ndjson_reader::ndjson_reader(input, verbose),
                verbose,
            )
            .unwrap();
            num = 3
        }
        _ => {
            let repo_link = "https://github.com/Tahaa-Dev/fiox";
            eprintln!(
                "ERROR: Input extension \"{}\" is not supported currently.\n Open an issue at {}",
                input_ext, repo_link,
            );
            exit(1);
        }
    };
    (data1, data2, data3, data4, num)
}

fn match_output(
    data: WriterStreams<impl Iterator<Item = CtxResult<DataTypes, Error>>>,
    output: std::fs::File,
    verbose: bool,
    output_ext: &str,
    parse_numbers: bool,
) {
    match output_ext {
        "json" => write_json::write_json(data, output, verbose, parse_numbers),
        "toml" => toml_writer::toml_writer(data, output, verbose, parse_numbers),
        "csv" => csv_writer::csv_writer(data, output, verbose),
        "ndjson" => ndjson_writer::ndjson_writer(data, verbose, output, parse_numbers),
        _ => {
            let repo_link = "https://github.com/Tahaa-Dev/fiox";
            eprintln!(
                "ERROR: Output extension \"{}\" is not supported currently.\n Open an issue at {}",
                output_ext, repo_link,
            );
            exit(1);
        }
    };
}
