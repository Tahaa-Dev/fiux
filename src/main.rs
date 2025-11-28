// MiMalloc setup because MiMalloc is much faster than the default rust allocator
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod tests;
mod utils;
use clap::Parser;
use colored::Colorize;
use std::path::Path;
use std::process::exit;
use utils::*;

fn main() {
    let args: FioxArgs = cli::FioxArgs::parse();

    let repo_link = "https://github.com/tahamahmoud7097-wq/fiox"
        .truecolor(16, 101, 230)
        .bold();

    match args.cmd {
        Commands::Convert {
            verbose,
            input,
            output,
        } => {
            // Check if input exists
            if !Path::new(&input).exists() {
                eprintln!("{}", "ERROR: Input file doesn't exist.".red().bold());
                exit(1);
            }

            // Check if output exists, if it doesn't then create it
            if !Path::new(&output).exists() {
                println!(
                    "{}",
                    format!(
                        "Output file [{}] doesn't exist, creating a new file for it...",
                        output.to_str().unwrap_or("output file")
                    )
                    .truecolor(179, 245, 216)
                    .italic()
                );
                std::fs::File::create(&output)
                    .better_expect("ERROR: Couldn't create output file.", verbose);
            }

            let input_ext = input
                .extension()
                .better_expect("ERROR: Input file has no valid extension.", verbose)
                .to_str()
                .better_expect("ERROR: Input file has no valid extension.", verbose);
            let output_ext = output
                .extension()
                .better_expect("ERROR: Output file has no valid extension.", verbose)
                .to_str()
                .better_expect("ERROR: Output file has no valid extension.", verbose);

            let now = std::time::Instant::now();

            let data: UniversalData = match input_ext {
                "json" => json_reader::json_reader(&input, verbose),
                "toml" => toml_reader::toml_reader(&input, verbose),
                "csv" => csv_reader::csv_reader(&input, verbose),
                _ => {
                    eprintln!(
                        "{} \n Open an issue at {}",
                        format!(
                            "ERROR: Intput extension \"{}\" is not supported currently.",
                            input_ext
                        )
                        .red()
                        .bold(),
                        repo_link
                    );
                    exit(1);
                }
            };
            match output_ext {
                "json" => write_json::write_json(&data, &output, verbose),
                "toml" => toml_writer::toml_writer(&data, &output, verbose),
                "csv" => csv_writer::csv_writer(&data, &output, verbose),
                _ => {
                    eprintln!(
                        "{} \n Open an issue at {}",
                        format!(
                            "ERROR: Output extension \"{}\" is not supported currently.",
                            output_ext
                        )
                        .red()
                        .bold(),
                        repo_link
                    );
                    exit(1);
                }
            };
            println!(
                "Finished converting {} -> {} in {:?}",
                input.to_str().unwrap_or("input file").bright_green().bold(),
                output
                    .to_str()
                    .unwrap_or("output file")
                    .bright_green()
                    .bold(),
                now.elapsed()
            );
        }

        Commands::Validate { input, verbose } => {
            // Check if input exists
            if !Path::new(&input).exists() {
                eprintln!(
                    "{}",
                    "ERROR: Input file doesn't exist for validation."
                        .red()
                        .bold()
                );
                exit(1);
            }

            let input_ext = input
                .extension()
                .better_expect("ERROR: Input file has no valid extension.", verbose)
                .to_str()
                .better_expect("ERROR: Input file has no valid extension.", verbose);

            match input_ext {
                "json" => json_validator::validate_json(&input, verbose),
                "toml" => toml_validator::validate_toml(&input, verbose),
                "csv" => csv_validator::validate_csv(&input, verbose),
                "ndjson" => ndjson_validator::validate_ndjson(&input, verbose),
                _ => {
                    eprintln!(
                        "{} \n Open an issue at {}",
                        format!(
                            "ERROR: Input extension \"{}\" is not supported currently.",
                            input_ext
                        )
                        .red()
                        .bold(),
                        repo_link
                    );
                    exit(1);
                }
            };
            println!(
                "{}",
                format!(
                    "Input file [{}] is valid!",
                    input.to_str().unwrap_or("inputFile")
                )
                .green()
                .bold()
            );
        }
    }
}
