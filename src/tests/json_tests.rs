use crate::utils::{UniversalData, Vals};

use std::path::PathBuf;

use serde_json::Value as JsonVal;

#[cfg(test)]
#[test]
fn json_test() -> Result<(), std::io::Error> {
    let json_str: &str = r#"
[
  {
    "NAME": "Joe",
    "AGE": 20,
    "ID": 2038
  },
  {
    "NAME": "John",
    "AGE": 27,
    "ID": 2927
  },
  {
    "NAME": "Jesse",
    "AGE": 30,
    "ID": 4986
  }
]
    "#;
    let json_ser = serde_json::from_str::<JsonVal>(json_str)?;

    let json_data = UniversalData::Structured(Vals::Json(json_ser));

    let temp_file = tempfile::Builder::new().suffix(".json").tempfile()?;
    let temp_path = PathBuf::from(temp_file.path());

    crate::write_json::write_json(&json_data, &temp_path, false);

    assert_eq!(
        crate::json_reader::json_reader(&temp_path, false),
        json_data
    );
    crate::json_validator::validate_json(&temp_path, false);

    let headers: Vec<String> = vec!["NAME".to_string(), "AGE".to_string(), "ID".to_string()];

    let rows: Vec<Vec<String>> = vec![
        vec!["Joe".to_string(), 20.to_string(), 2038.to_string()],
        vec!["John".to_string(), 27.to_string(), 2927.to_string()],
        vec!["Jesse".to_string(), 30.to_string(), 4986.to_string()],
    ];

    let csv_data = UniversalData::Table { headers, rows };

    crate::write_json::write_json(&csv_data, &temp_path, false);

    assert_eq!(
        crate::json_reader::json_reader(&temp_path, false),
        json_data
    );
    crate::json_validator::validate_json(&temp_path, false);

    Ok(())
}
