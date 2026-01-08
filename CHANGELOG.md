# Changelog

## [0.5.0] - 2026-01-08

### Added
- Comprehensive test suite (7 unit tests + 9 integration tests)
- GitHub Actions CI pipeline
- Tests run on Ubuntu, macOS, and Windows
- `--delimiter` flag for validators
- Error logging to Markdown files with `--log-file`
- Beautiful error messages with context chains and hints

### Changed
- All internal functions now `pub(crate)` for cleaner docs
- Exit codes are semantic (0 = success, 1 = validation failure)
- Removed all `unsafe` blocks (100% safe Rust)

### Fixed
- Improved error context chains
- Better integration with parsing crates' error reporting (rust-csv, serde_json, toml and serde)

---

## v0.4.0 - 2026-01-06

## Summary

- Added `--input-delimiter` / `--output-delimiter` flag
- Added `--log-file` flag
- Made arg parsing global with a static `LazyLock`
- Fixed issues with validators
- Completely removed `--verbose` / `-v` flag

---
### Details

#### 1. `--input-delimiter` / `--output-delimiter` flag

- Flags that make fiox ignore extension completely and treat the file as a CSV with the specified delimiter (e.g. PSV with `--input-delimiter '|'`)

**Example:**

```sh
# Input
fiox convert input.psv --input-delimiter '|' -o output.json

# Output
fiox convert input.csv -o output.ssv --output-delimiter ';'
```

---

#### 2. `--log-file` flag:

- Flag for logging recoverable errors into a file instead of printing to stderr.

**Example:**

```sh
# long flag
fiox convert input.ndjson -o output.toml --log-file err.md

# short flag
fiox validate input.csv -l csv_err.md
```

---

#### 3. Global arg parsing

 This is a change mainly for architecture, it won't affect fiox usage but will make development easier and will be more integrated into functions in the next version, for now it's just laying down the groundwork for less argument drilling in functions which will help in adding more flags in the future.
 
---

#### 4. Issues with validators

- Validators used to not give a lot of debugging information on errors, now they do!
- Fixed the double "input file is valid!" Bug
- Fixed the "input file is valid!" Even on error bug

---

#### 5. `--verbose` flag removal

This flag is now redundant since validate now gives full debugging information on error.
