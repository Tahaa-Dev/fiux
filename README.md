<h1 align="center">fiux</h1>

[<img alt="crates.io" src="https://img.shields.io/crates/v/fiux.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/fiux)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-fiux-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/fiux)
[![CI](https://github.com/Tahaa-Dev/fiux/workflows/CI/badge.svg)](https://github.com/Tahaa-Dev/fiux/actions)
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
cargo install fiux
```

- And that's it! You've installed fiux!

---

## Usage

```bash
# conversion
fiux convert <INPUT> -o <OUTPUT>

# validation
fiux validate <INPUT>

# options (flags)
fiux validate <INPUT> --log-file err.md
fiux convert <INPUT> --output <OUTPUT> -a
```


---

### benchmarks

Benchmarks were done with a 100k line CSV converted to other formats for consistency across benchmarks

##### Example benchmark

- This is a sample of the command used to capture these benchmarks and its output:

```sh
$ hyperfine -w 5  "mlr --c2j cat test100k.csv > temp.json" "fiux convert test100k.csv -o temp.json"
Benchmark 1: mlr --c2j cat test100k.csv > temp.json
  Time (mean ± σ):     605.9 ms ±  38.0 ms    [User: 1223.0 ms, System: 271.8 ms]
  Range (min … max):   556.0 ms … 677.0 ms    50 runs

Benchmark 2: fiux convert test100k.csv -o temp.json
  Time (mean ± σ):     107.5 ms ±  12.5 ms    [User: 95.1 ms, System: 17.2 ms]
  Range (min … max):    83.2 ms … 123.9 ms    50 runs

Summary
  fiux convert test100k.csv -o temp.json ran
    5.24 ± 1.07 times faster than mlr --c2j cat test100k.csv > temp.json
```

##### Results

| **Benchmark**     | **fiux**     | **Node.js**     | **Miller / jq (C)**              |
| ------------- | -------- | ----------- | ---------------------------- |
| CSV → JSON    | ~95ms    | ~1.29s      | Miller: ~603ms               |
| CSV → TOML    | ~101ms   | ~1.6s       | No native TOML support       |
| JSON → TOML   | ~888ms   | ~8.7s       | No native TOML support       |
| TOML → JSON   | ~862ms   | ~9s         | No native TOML support       |
| CSV → NDJSON  | ~97ms    | ~1.2s       | Miller: ~2.85s               |
| JSON → NDJSON | ~750ms   | ~6s         | jq: ~2.77s \| Miller: ~2.92s |
| TOML → NDJSON | ~921ms   | ~12s        | No native TOML support       |
| NDJSON → JSON | ~310ms   | ~6.2s       | jq: ~2.65 \| Miller: ~2.88s  |
| NDJSON → TOML | ~820ms   | ~10.8s      | No native TOML support       |

As you can see from these benchmarks, ***fiux is much faster than most industry-standard file conversion tools***, fiux scales even better on better / server hardware! (using SSH)

**Note:** TOML conversions are generally slower than other formats since TOML is very limited when it comes to streaming and parsing is slower as it is more complicated than other formats.

---

### Notes

- fiux is licensed under the **MIT** license.
- For specifics about contributing to fiux, see <a href="CONTRIBUTING.md">CONTRIBUTING.md</a>.
- For changes, see <a href="CHANGELOG.md">CHANGELOG.md</a>.
