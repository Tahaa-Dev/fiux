use toml::Value as TomlVal;

use std::path::PathBuf;

use crate::utilities::{UniversalData, Vals};

#[cfg(test)]
#[test]
fn toml_tests() -> Result<(), std::io::Error> {
    let toml_str: &str = r#"
[[Rows]]
NAME = "Joe"
AGE = 20
ID = 2038

[[Rows]]
NAME = "John"
AGE = 27
ID = 2927

[[Rows]]
NAME = "Jesse"
AGE = 30
ID = 4986
    "#;

    let toml_ser = toml::from_str::<TomlVal>(toml_str).map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "ERROR: Couldn't serialize input toml_str",
        )
    })?;

    let toml_data = UniversalData::Structured(Vals::Toml(toml_ser));

    let temp_file = tempfile::Builder::new().suffix(".toml").tempfile()?;
    let temp_path = PathBuf::from(temp_file.path());

    crate::toml_writer::toml_writer(&toml_data, &temp_path, false);

    assert_eq!(
        crate::toml_reader::toml_reader(&temp_path, false),
        toml_data
    );

    crate::toml_validator::validate_toml(&temp_path, false);

    let headers: Vec<String> = vec!["NAME".to_string(), "AGE".to_string(), "ID".to_string()];

    let rows: Vec<Vec<String>> = vec![
        vec!["Joe".to_string(), 20.to_string(), 2038.to_string()],
        vec!["John".to_string(), 27.to_string(), 2927.to_string()],
        vec!["Jesse".to_string(), 30.to_string(), 4986.to_string()],
    ];

    let csv_data = UniversalData::Table { headers, rows };

    crate::toml_writer::toml_writer(&csv_data, &temp_path, false);

    assert_eq!(
        crate::toml_reader::toml_reader(&temp_path, false),
        toml_data
    );

    crate::toml_validator::validate_toml(&temp_path, false);

    Ok(())
}
