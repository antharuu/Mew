//! File writer module
//!
//! Provides functionality for writing generated CSS files.

use std::fs;

/// Writes the generated CSS content to a file.
/// The output filename is derived from the input filename by replacing the
/// `.mew` extension with `.css`.
///
/// # Arguments
/// * `mew_file` - Path to the original Mew source file
/// * `content` - The CSS content to write
///
/// # Returns
/// * `Ok(())` - If the write operation was successful
/// * `Err(String)` - A descriptive error message if the operation failed
pub fn write_css_file(mew_file: &str, content: &str) -> Result<(), String> {
    let output_file = mew_file.replace(".mew", ".css");
    fs::write(&output_file, content)
        .map_err(|e| format!("Error writing file {}: {}", output_file, e))?;

    println!("Generated CSS file: {}", output_file);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_write_css_file() -> Result<(), String> {
        let temp = tempdir().unwrap();
        let mew_file = temp.path().join("test.mew");
        let css_content = ".test {\n    color: blue;\n}\n";

        fs::write(&mew_file, "").unwrap();
        write_css_file(mew_file.to_str().unwrap(), css_content)?;

        let css_file = temp.path().join("test.css");
        assert!(css_file.exists());
        assert_eq!(fs::read_to_string(css_file).unwrap(), css_content);
        Ok(())
    }
}