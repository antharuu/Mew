//! File reader module
//!
//! Provides functionality for reading Mew source files safely.

use std::fs;
use std::path::Path;

/// Reads the content of a Mew source file.
///
/// # Arguments
/// * `file_path` - Path to the Mew source file
///
/// # Returns
/// * `Ok(String)` - The file content if successfully read
/// * `Err(String)` - A descriptive error message if the operation failed
pub fn read_mew_file(file_path: &str) -> Result<String, String> {
    let path = Path::new(file_path);

    if !path.exists() {
        return Err(format!("Error: file {} does not exist", file_path));
    }

    if !path.is_file() {
        return Err(format!("Error: {} is not a file", file_path));
    }

    fs::read_to_string(file_path)
        .map_err(|e| format!("Error reading file {}: {}", file_path, e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_read_valid_file() -> Result<(), String> {
        let temp = tempdir().unwrap();
        let test_file = temp.path().join("test.mew");
        let content = ".test { color: blue; }";
        fs::write(&test_file, content).unwrap();

        let read_content = read_mew_file(test_file.to_str().unwrap())?;
        assert_eq!(read_content, content);
        Ok(())
    }

    #[test]
    fn test_read_nonexistent_file() {
        let result = read_mew_file("nonexistent.mew");
        assert!(result.is_err());
    }
}