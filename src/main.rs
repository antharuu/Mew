//! Mew - A modern CSS preprocessor
//!
//! This is the main entry point for the Mew CSS preprocessor.
//! It handles both single file and directory processing modes.

mod parsers;
mod parser;
mod file_reader;
mod file_writer;
mod directory_processor;

use parser::parse;
use std::env;
use std::path::PathBuf;
use crate::directory_processor::{DirectoryConfig, process_directory};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: mew <input.mew or directory>");
        eprintln!("Examples:");
        eprintln!("  mew styles.mew        # Process single file");
        eprintln!("  mew ./styles/         # Process directory");
        return;
    }

    let input_path = PathBuf::from(&args[1]);

    if input_path.is_dir() {
        // Directory processing configuration
        let config = DirectoryConfig::new(&input_path)
            .with_recursive(true)
            .with_preserve_structure(true)
            .with_extensions(vec!["mew".into()]);

        match process_directory(config) {
            Ok(processed_files) => {
                println!("\nProcessing completed!");
                println!("Files processed: {}", processed_files.len());
                for file in processed_files {
                    println!("Generated: {:?}", file);
                }
            }
            Err(e) => eprintln!("Error processing directory: {:?}", e),
        }
    } else {
        // Single file processing
        match file_reader::read_mew_file(&args[1]) {
            Ok(content) => {
                let css = parse(&content);
                if let Err(e) = file_writer::write_css_file(&args[1], &css) {
                    eprintln!("{}", e);
                }
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}