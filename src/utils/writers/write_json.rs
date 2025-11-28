use std::path::PathBuf;

use crate::{utilities::UniversalData, utils::BetterExpect};
use serde_json::Value as JsonVal;

pub fn write_json(data: &UniversalData, path: &PathBuf, verbose: bool) {
    // Check if input data is struct, key-value based (like JSON and TOML) or table (like CSV)
    if let UniversalData::Structured(non_json) = data {
        let json_ser: JsonVal = serde_json::to_value(non_json).better_expect(
            "ERROR: Couldn't serialize input file to JSON format.",
            verbose,
        );

        std::fs::write(
            path,
            serde_json::to_string_pretty(&json_ser).unwrap_or_default(),
        )
        .better_expect("ERROR: Failed to write into output file.", verbose);

        // If table based, uses the `.zip()` method to bind table headers (column names) as keys to their values in the rows to form key-value pairs for serde_json to serialize
    } else if let UniversalData::Table { headers, rows } = data {
        let mut json_str = String::from("[");

        let new_headers: Vec<String> = headers
            .iter()
            .map(|h| h.replace('\\', "\\\\").replace('"', "\\\""))
            .collect();

        let mut first_row: bool = true;

        rows.iter().for_each(|row| {
            if !first_row {
                json_str.push(',');
            }
            first_row = false;
            json_str.push_str("\n  {\n");
            let mut first_pair: bool = true;

            new_headers.iter().zip(row.iter()).for_each(|(h, v)| {
                if !first_pair {
                    json_str.push_str(",\n");
                }
                first_pair = false;

                if v.parse::<i64>().is_ok()
                    || v.parse::<f64>().is_ok()
                    || v == "false"
                    || v == "true"
                {
                    json_str.push_str(&format!("    \"{}\": {}", h, v));
                } else if v.is_empty() {
                    json_str.push_str(&format!("    \"{}\": \"\"", h));
                } else {
                    let v = v.replace('\\', "\\\\").replace('"', "\\\"");
                    json_str.push_str(&format!("    \"{}\": \"{}\"", h, v));
                }
            });

            json_str.push_str("\n  }");
        });

        json_str.push_str("\n]");
        std::fs::write(path, json_str)
            .better_expect("ERROR: Failed to write into output file.", verbose);
    }
}
