use assert_cmd::{assert::OutputAssertExt, cargo};
use resext::{CtxResult, ResExt};
use std::{fs, io::Error, process::Command};
use tempfile::Builder;

#[test]
fn test_csv_to_json_conversion() -> CtxResult<(), Error> {
    let input =
        Builder::new().suffix(".csv").tempfile().context("Failed to crate input TempFile")?;
    let output =
        Builder::new().suffix(".json").tempfile().context("Failed to create output TempFile")?;

    // Write simple CSV
    fs::write(input.path(), "name,age,city\nAlice,30,NYC\nBob,25,LA\n")
        .context("Failed to write input TempFile contents")?;

    // Run fiox convert
    Command::new(cargo::cargo_bin!("fiox"))
        .arg("convert")
        .arg(input.path())
        .arg("-o")
        .arg(output.path())
        .assert()
        .success();

    // Verify output
    let result = fs::read_to_string(output.path()).context("Failed to read output file")?;
    assert!(result.contains(r#""name": "Alice""#));
    assert!(result.contains(r#""age": "30""#));
    assert!(result.contains(r#""city": "NYC""#));

    Ok(())
}

#[test]
fn test_tsv_to_json_with_delimiter() -> CtxResult<(), Error> {
    let input =
        Builder::new().suffix(".tsv").tempfile().context("Failed to crate input TempFile")?;
    let output =
        Builder::new().suffix(".ndjson").tempfile().context("Failed to create output TempFile")?;

    // Write TSV
    fs::write(input.path(), "name\tage\tcity\nAlice\t30\tNYC\nBob\t25\tLA\n")
        .context("Failed to write input TempFile contents")?;

    Command::new(cargo::cargo_bin!("fiox"))
        .arg("convert")
        .arg(input.path())
        .arg("--input-delimiter")
        .arg("\t")
        .arg("-o")
        .arg(output.path())
        .assert()
        .success();

    let result = fs::read_to_string(output.path()).context("Failed to read output file")?;
    assert!(result.contains(r#""name": "Alice""#));
    assert!(result.contains(r#""age": "30""#));
    assert!(result.contains(r#""city": "NYC""#));
    Ok(())
}

#[test]
fn test_error_logging_to_file() -> CtxResult<(), Error> {
    let input =
        Builder::new().suffix(".csv").tempfile().context("Failed to crate input TempFile")?;
    let output =
        Builder::new().suffix(".toml").tempfile().context("Failed to create output TempFile")?;
    let log = tempfile::NamedTempFile::new().context("Failed to create log TempFile")?;

    // Write broken CSV (mismatched columns)
    fs::write(input.path(), "a,b,c\n1,2,3\ninvalid\n4,5,6\n")
        .context("Failed to write input TempFile contents")?;

    Command::new(cargo::cargo_bin!("fiox"))
        .arg("--log-file")
        .arg(log.path())
        .arg("convert")
        .arg(input.path())
        .arg("-o")
        .arg(output.path())
        .assert()
        .success(); // Should still succeed (graceful error handling)

    // Check log file has error content
    let log_content = fs::read_to_string(log.path()).context("Failed to read output file")?;
    assert!(!log_content.is_empty());
    assert!(log_content.contains("Invalid CSV"));
    assert!(log_content.contains("---")); // Separator

    Ok(())
}

#[test]
fn test_validation_pass() -> CtxResult<(), Error> {
    let input =
        Builder::new().suffix(".json").tempfile().context("Failed to create output TempFile")?;

    let valid_json = r#"
    {
      "a": 1,
      "b": 2,
      "c": {
        "arr": [1, 2, 3],
        "k": {
          "a": 10,
          "b": 2
        }
      }
    }
    "#;

    // Write valid JSON
    fs::write(input.path(), valid_json).context("Failed to write input TempFile contents")?;

    Command::new(cargo::cargo_bin!("fiox")).arg("validate").arg(input.path()).assert().success();

    Ok(())
}

#[test]
fn test_validation_fail() -> CtxResult<(), Error> {
    let input =
        Builder::new().suffix(".csv").tempfile().context("Failed to crate input TempFile")?;

    // Write invalid CSV
    fs::write(input.path(), "a,b,c\n1,2,3\ninvalid\n")
        .context("Failed to write input TempFile contents")?;

    Command::new(cargo::cargo_bin!("fiox")).arg("validate").arg(input.path()).assert().failure(); // Should exit with code 1

    Ok(())
}

#[test]
fn test_validation_with_delimiter() -> CtxResult<(), Error> {
    let input =
        Builder::new().suffix(".psv").tempfile().context("Failed to crate input TempFile")?;

    // Write PSV (pipe-separated)
    fs::write(input.path(), "a|b|c\n1|2|3\n").context("Failed to write input TempFile contents")?;

    Command::new(cargo::cargo_bin!("fiox"))
        .arg("validate")
        .arg(input.path())
        .arg("--delimiter")
        .arg("|")
        .assert()
        .success();

    Ok(())
}

#[test]
fn test_ndjson_to_json() -> CtxResult<(), Error> {
    let input =
        Builder::new().suffix(".ndjson").tempfile().context("Failed to crate input TempFile")?;
    let output =
        Builder::new().suffix(".json").tempfile().context("Failed to create output TempFile")?;

    // Write NDJSON
    fs::write(
        input.path(),
        r#"{"name":"Alice","age":30}
{"name":"Bob","age":25}
"#,
    )
    .context("Failed to write input TempFile contents")?;

    Command::new(cargo::cargo_bin!("fiox"))
        .arg("convert")
        .arg(input.path())
        .arg("-o")
        .arg(output.path())
        .assert()
        .success();

    let result = fs::read_to_string(output.path()).context("Failed to read output file")?;

    assert!(result.contains(r#""name": "Alice""#));

    Ok(())
}

#[test]
fn test_parse_numbers_flag() -> CtxResult<(), Error> {
    let input =
        Builder::new().suffix(".csv").tempfile().context("Failed to crate input TempFile")?;
    let output =
        Builder::new().suffix(".ndjson").tempfile().context("Failed to create output TempFile")?;

    fs::write(input.path(), "a,b\n1,2\n3,4\n")
        .context("Failed to write input TempFile contents")?;

    Command::new(cargo::cargo_bin!("fiox"))
        .arg("convert")
        .arg(input.path())
        .arg("-o")
        .arg(output.path())
        .arg("--parse-numbers")
        .assert()
        .success();

    let result = fs::read_to_string(output.path()).context("Failed to read output file")?;
    // With --parse-numbers, should be numeric not string
    assert!(result.contains(r#""a": 1"#));

    Ok(())
}

#[test]
fn test_toml_to_json() -> CtxResult<(), Error> {
    let input =
        Builder::new().suffix(".toml").tempfile().context("Failed to crate input TempFile")?;
    let output =
        Builder::new().suffix(".json").tempfile().context("Failed to create output TempFile")?;

    fs::write(
        input.path(),
        r#"
[package]
name = "fiox"
version = "0.4.0"
edition = "2024"
authors = ["Taha Mahmoud <tahamahmoud7097@gmail.com>"]
categories = ["filesystem", "parsing", "command-line-utilities"]
description = "The fastest multi-format file converter CLI tool"
keywords = ["filesystem", "file-conversion", "file-handling", "cli"]
license = "MIT"
repository = "https://github.com/Tahaa-Dev/fiox"
readme = "README.md"

[dependencies]
clap = { version = "4.5.53", features = ["derive"] }
serde = "1.0.228"
serde_json = "1.0.145"
toml = "0.9.8"
csv = "1.4.0"
resext = "0.6.2"

[dev-dependencies]
assert_cmd = "2.1.1"
predicates = "3.1.3"
tempfile = "3.24.0"
    "#,
    )
    .context("Failed to write input TempFile contents")?;

    Command::new(cargo::cargo_bin!("fiox"))
        .arg("convert")
        .arg(input.path())
        .arg("-o")
        .arg(output.path())
        .assert()
        .success();

    let result = fs::read_to_string(output.path()).context("Failed to read output file")?;
    assert!(result.contains(r#""name": "fiox""#) || result.contains(r#""name":"fiox""#));

    Ok(())
}
