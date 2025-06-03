use thiserror::Error;

pub fn load_json_file(
    file_path: &str,
) -> Result<serde_json::Value, FileLoadingError> {
    let json_str = std::fs::read_to_string(file_path)?;
    Ok(serde_json::from_str(&json_str)?)
}

#[derive(Debug, Error)]
pub enum FileLoadingError {
    #[error("Failed to read file: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to parse JSON: {0}")]
    ParseError(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_json_file() {
        let file_path = "test_data/sample_1/sample.json";
        let s = load_json_file(file_path).unwrap();
        dbg!(s);
    }
}
