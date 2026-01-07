# fiox

**The *fastest* multi-format file handling CLI tool, built in *Rust***

- Supports **NDJSON**, **JSON**, **CSV**, **PSV**, **TSV**, **TOML** formats and more!

- Support for more formats will be added soon.

---

## Features

- Convert between NDJSON, JSON, TOML, CSV, TSV, PSV and more!
- Validate files quickly with detailed logs for debugging
- The fastest thanks to being written in highly optimized Rust
- Highly memory efficient for low resource environments through streaming architecture and optimized allocations.
- Intuitive and easy to use out of the box, built in 100% pure Rust which makes it easy to install.

---

## Installation

Clone the repository and build:

```bash
git clone https://github.com/Tahaa-Dev/fiox.git fiox

cd fiox

cargo build --release
# optional: move binary to ~/.local/bin for convenience
mv ~/fiox/target/release/fiox ~/.local/bin/fiox
```

**Note:** This installation is temporary until I publish as a binary crate on **crates.io**.

---

## Usage

```bash
# conversion
fiox convert <INPUT> -o <OUTPUT>

# validation
fiox validate <INPUT>

# options (flags)
fiox validate <INPUT> --log-file err.md
fiox convert <INPUT> --output <OUTPUT> -a
```


---

## Options (flags)

| **Option (flag)**                               | **Functionality**                                                                                                                                                                          |
| ----------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `-a`, `--append`                                | fiox overwrites existing data in files by default, this flag makes it append into the output file instead. **WARNING**: This flag can lead to unexpected output on some formats like JSON. |
| `-p`, `--parse-numbers`                         | This flag only affects CSV -> non-tabular format conversions only. It makes it so that fiox infers types and doesn't quote (stringify) numbers.                                            |
| `-o`, `--output`                                | Not an option, but is a flag you use to indicate which file is the output file.                                                                                                            |
| `-l`, `--log-file`                              | Flag to log recoverable errors in a file (preferably Markdown) instead of printing to stderr                                                                                               |
| `-i`, `--input-delimiter`, `--output-delimiter` | Flag to ignore extension and parse file / write output as CSV with the specified delimiter (e.g. PSV with '\|' as the delimiter)                                                           |

---
### Benchmarks

**Notes:** 
- All of these benchmarks were done using the same file but converted to other formats using fiox which is a 100k rows with 6 fields per row CSV file with 3 text columns, 2 number columns and a column of dates.
- All of these benchmarks were done using **hyperfine** on entry level hardware. (Ryzen 3 3100, 8GB of DDR3 RAM and a SATA SSD).

| **Benchmark** | **fiox** | **Node.js** | **Miller / jq (C)**          |
| ------------- | -------- | ----------- | ---------------------------- |
| CSV → JSON    | ~109ms   | ~1.29s      | Miller: ~603ms               |
| CSV → TOML    | ~111ms   | ~1.6s       | No native TOML support       |
| JSON → TOML   | ~1.1s    | ~8.7s       | No native TOML support       |
| TOML → JSON   | ~862ms   | ~9s         | No native TOML support       |
| CSV → NDJSON  | ~107ms   | ~1.2s       | Miller: ~2.85s               |
| JSON → NDJSON | ~750ms   | ~6s         | jq: ~2.77s \| Miller: ~2.92s |
| TOML → NDJSON | ~1.1s    | ~12s        | No native TOML support       |
| NDJSON → JSON | ~310ms   | ~6.2s       | jq: ~2.65 \| Miller: ~2.88s  |
| NDJSON → TOML | ~820ms   | ~10.8s      | No native TOML support       |

As you can see from these benchmarks, ***fiox is much faster than most industry-standard file conversion tools***, fiox scales even better on better / server hardware! (using SSH)

**Note:** TOML conversions are generally slower than other formats since TOML is very limited when it comes to streaming and parsing is slower as it is more complicated than other formats.

---
### Plans

- [x] Modularize readers and writers.
- [x] Add clap for help and subcommand / flag support.
- [x] Add a flag for better debugging.
- [x] Add a validation subcommand.
- [x] Implement streaming for all commands (except TOML).
- [ ] Add support for more formats (in progress).
- [ ] Add parallelization for processing using rayon.
- [ ] Add more subcommands for more flexibility.
- [ ] Publish as a binary crate on **crates.io**.

---

### Notes

- fiox is licensed under the **MIT** license.
- For specifics about contributing to fiox, see <a href="CONTRIBUTING.md">CONTRIBUTING.md</a>.
- For changes, see <a href="CHANGELOG.md">CHANGELOG.md</a>
- fiox is still unstable and in heavy development, it's recommended that you only use fiox after v0.5.0 release.
