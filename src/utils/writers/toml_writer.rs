use std::path::PathBuf;

use crate::{utilities::UniversalData, utils::BetterExpect};
use toml::Value as toml_val;

pub fn toml_writer(data: &UniversalData, path: &PathBuf, verbose: bool) {
    // Check if input data is a table or struct-based (like JSON and TOML) data.
    if let UniversalData::Structured(non_toml) = data {
        // Serialize to TOML
        let toml_ser = toml_val::try_from(non_toml).better_expect(
            "ERROR: Couldn't serialize input file into TOML format.",
            verbose,
        );

        // First, check if the data has a top level array to which TOML doesn't support to handle it
        if let toml_val::Array(arr) = toml_ser {
            let mut output: String = String::new();

            arr.iter().for_each(|item| {
                if let toml_val::Table(obj) = item {
                    output.push_str("[[Array]]\n");
                    output.push_str(&toml::to_string_pretty(&obj).unwrap_or_default());
                    output.push('\n');
                }
            });

            // Write into the file.
            std::fs::write(path, output.trim_end())
                .better_expect("ERROR: Failed to write final file.", verbose);

        // If it doesn't have a top level Array, it will just write into the file.
        } else {
            std::fs::write(
                path,
                toml::to_string_pretty(&toml_ser).unwrap_or(toml_ser.to_string()),
            )
            .better_expect("ERROR: Failed to write into output file.", verbose);
        }

    // If table based, write into the file by making keys of the TOML tables (objects) the headers (column names) of the table.
    } else if let UniversalData::Table { headers, rows } = data {
        // Iterator chain for writing into the file by using the `.zip()` method on keys (table headers) and values.
        let new_headers: Vec<String> = headers
            .iter()
            .map(|h| h.replace('\\', "\\\\").replace('"', "\\\""))
            .collect();

        let mut toml_str: String = String::new();

        rows.iter().for_each(|row| {
            toml_str.push_str("[[Rows]]\n");
            new_headers.iter().zip(row.iter()).for_each(|(h, v)| {
                let mut v = v.replace('\\', "\\\\").replace('"', "\\\"");
                h.trim().to_string();

                if v.parse::<f64>().is_err()
                    && v.parse::<i64>().is_err()
                    && v != "true"
                    && v != "false"
                {
                    v = format!("\"{}\"", v);
                }

                if h.bytes()
                    .any(|c| !(c.is_ascii_alphanumeric() || c == b'_' || c == b'-'))
                {
                    toml_str.push_str(&format!("\"{}\" = {}\n", h, v));
                } else {
                    toml_str.push_str(&format!("{} = {}\n", h, v));
                }
            });

            toml_str.push('\n');
        });

        std::fs::write(path, toml_str.trim_end())
            .better_expect("ERROR: Failed to write into output file.", verbose);
    }
}
