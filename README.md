# fiox

[![CI](https://github.com/Tahaa-Dev/fiox/workflows/CI/badge.svg)](https://github.com/Tahaa-Dev/fiox/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**The *fastest* multi-format file converter CLI tool.**

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

```sh
cargo install fiox
```
```
```

- And that's it! You've installed fiox!

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

### benchmarks

Benchmarks were done with a 100k line CSV converted to other formats for consistency across benchmarks

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
- For changes, see <a href="CHANGELOG.md">CHANGELOG.md</a>.
