use crate::internal::*;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

pub fn create_file(path: &str) {
    let return_val = File::create(path);
    match return_val {
        Ok(_file) => {
            log(format!("Created file {}", path));
        }
        Err(e) => {
            crash(format!("Failed to create file {}, Error: {}", path, e), 1);
        }
    }
}

pub fn append_file(path: &str, content: &str) -> std::io::Result<()> {
    log(format!("Appending {} to file {}", content, path));
    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn create_directory(path: &str) {
    let return_val = std::fs::create_dir(path);
    match return_val {
        Ok(_) => {
            log(format!("Created directory {}", path));
        }
        Err(e) => {
            crash(
                format!("Failed to create directory {}, Error: {}", path, e),
                1,
            );
        }
    }
}

