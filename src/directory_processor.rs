//! Directory processing implementation
//!
//! Handles batch processing of Mew files within directories, including:
//! - Recursive directory traversal
//! - Structure preservation
//! - Multiple file extension support
//! - Output path configuration

use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use std::fmt;
use walkdir::WalkDir;
use crate::parser::parse;

/// Configuration for directory processing
#[derive(Debug)]
pub struct DirectoryConfig {
    /// Input directory path
    pub input_path: PathBuf,
    /// Optional custom output directory path
    pub output_path: Option<PathBuf>,
    /// Whether to preserve directory structure in output
    pub preserve_structure: bool,
    /// Whether to process directories recursively
    pub recursive: bool,
    /// List of supported file extensions (without dot)
    pub supported_extensions: Vec<String>,
}

impl DirectoryConfig {
    /// Creates a new directory configuration with default settings
    pub fn new<P: AsRef<Path>>(input_path: P) -> Self {
        Self {
            input_path: input_path.as_ref().to_path_buf(),
            output_path: None,
            preserve_structure: true,
            recursive: true,
            supported_extensions: vec!["mew".to_string()],
        }
    }

    /// Sets whether to preserve directory structure in output
    pub fn with_preserve_structure(mut self, preserve: bool) -> Self {
        self.preserve_structure = preserve;
        self
    }

    /// Sets whether to process directories recursively
    pub fn with_recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }

    /// Sets the list of supported file extensions
    pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
        self.supported_extensions = extensions;
        self
    }

    /// Sets the output directory path *(test purposes only)*
    #[cfg(test)]
    pub fn with_output_path<P: AsRef<Path>>(mut self, path: Option<P>) -> Self {
        self.output_path = path.map(|p| p.as_ref().to_path_buf());
        self
    }

    /// Checks if a given path has a supported file extension
    fn is_supported_extension(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| self.supported_extensions.iter().any(|supported| supported == ext))
            .unwrap_or(false)
    }
}

/// Custom error type for directory processing operations
#[derive(Debug)]
pub struct ProcessingError {
    message: String,
}

impl ProcessingError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<io::Error> for ProcessingError {
    fn from(err: io::Error) -> Self {
        ProcessingError::new(err.to_string())
    }
}

/// Processes a directory of Mew files according to the provided configuration
///
/// # Arguments
/// * `config` - The directory processing configuration
///
/// # Returns
/// * `Ok(Vec<PathBuf>)` - List of generated CSS file paths
/// * `Err(ProcessingError)` - Error details if processing failed
pub fn process_directory(config: DirectoryConfig) -> Result<Vec<PathBuf>, ProcessingError> {
    let mut processed_files = Vec::new();

    if !config.input_path.exists() || !config.input_path.is_dir() {
        return Err(ProcessingError::new(
            "Input directory does not exist or is not a directory"
        ));
    }

    // Create output directory if specified
    if let Some(ref output_path) = config.output_path {
        fs::create_dir_all(output_path)?;
    }

    // Configure directory walker based on recursion setting
    let walker = if config.recursive {
        WalkDir::new(&config.input_path)
    } else {
        WalkDir::new(&config.input_path).max_depth(1)
    };

    // Process each file in the directory
    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if !path.is_file() || !config.is_supported_extension(path) {
            continue;
        }

        let output_path = determine_output_path(&config, path)?;

        match process_single_file(path, &output_path) {
            Ok(_) => {
                processed_files.push(output_path);
                println!("Successfully processed: {:?}", path);
            }
            Err(e) => {
                eprintln!("Error processing {:?}: {}", path, e);
            }
        }
    }

    Ok(processed_files)
}

/// Determines the output path for a processed file based on configuration
fn determine_output_path(
    config: &DirectoryConfig,
    input_file: &Path,
) -> Result<PathBuf, ProcessingError> {
    let relative_path = input_file.strip_prefix(&config.input_path)
        .map_err(|_| ProcessingError::new("Failed to determine relative path"))?;

    let mut output_path = if let Some(ref output_dir) = config.output_path {
        if config.preserve_structure {
            let parent = relative_path.parent().unwrap_or_else(|| Path::new(""));
            let output_subdir = output_dir.join(parent);
            fs::create_dir_all(&output_subdir)?;
            output_subdir
        } else {
            output_dir.clone()
        }
    } else {
        input_file.parent().unwrap_or_else(|| Path::new("")).to_path_buf()
    };

    let filename = input_file.file_name()
        .ok_or_else(|| ProcessingError::new("Invalid filename"))?
        .to_string_lossy()
        .into_owned();

    let css_filename = if let Some(last_dot) = filename.rfind('.') {
        format!("{}.css", &filename[..last_dot])
    } else {
        format!("{}.css", filename)
    };

    output_path.push(css_filename);
    Ok(output_path)
}

/// Processes a single Mew file
fn process_single_file(input_path: &Path, output_path: &Path) -> Result<(), ProcessingError> {
    let content = fs::read_to_string(input_path)?;
    let css = parse(&content);
    fs::write(output_path, css)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_directory_processing() -> Result<(), ProcessingError> {
        let temp = tempdir().unwrap();
        let input_dir = temp.path().join("input");
        let output_dir = temp.path().join("output");
        fs::create_dir(&input_dir).unwrap();

        // Create test file
        let test_file = input_dir.join("test.mew");
        fs::write(&test_file, ".test { color: blue; }").unwrap();

        let config = DirectoryConfig::new(&input_dir)
            .with_output_path(Some(output_dir.clone()));

        let results = process_directory(config)?;
        assert_eq!(results.len(), 1);

        Ok(())
    }

    #[test]
    fn test_invalid_directory() {
        let config = DirectoryConfig::new("nonexistent");
        assert!(process_directory(config).is_err());
    }
}