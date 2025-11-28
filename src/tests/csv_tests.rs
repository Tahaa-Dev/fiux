use crate::utilities::UniversalData;
use std::path::PathBuf;

#[cfg(test)]
#[test]
fn csv_test() -> Result<(), std::io::Error> {
    let headers: Vec<String> = vec!["NAME".to_string(), "AGE".to_string(), "ID".to_string()];

    let rows: Vec<Vec<String>> = vec![
        vec!["Joe".to_string(), 20.to_string(), 2038.to_string()],
        vec!["\"Joh,n\"".to_string(), 27.to_string(), 2927.to_string()],
        vec!["Je\"se".to_string(), 30.to_string(), 4986.to_string()],
    ];

    let data = UniversalData::Table { headers, rows };

    let temp_file = tempfile::Builder::new().suffix(".csv").tempfile()?;

    let temp_path = PathBuf::from(temp_file.path());

    crate::csv_writer::csv_writer(&data, &temp_path, false);

    assert_eq!(crate::csv_reader::csv_reader(&temp_path, false), data);
    crate::csv_validator::validate_csv(&temp_path, false);

    Ok(())
}
