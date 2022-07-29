use crate::internal::*;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

pub fn create_file(path: &str) {
    let returncode = File::create(path);
    match returncode {
        Ok(_) => {
            log::info!("Create {}", path);
        }
        Err(e) => {
            crash(format!("Create {}: Failed with error {}", path, e), 1);
        }
    }
}

pub fn copy_file(path: &str, destpath: &str) {
    let return_code = std::fs::copy(path, destpath);
    match return_code {
        Ok(_) => {
            log::info!("Copy {} to {}", path, destpath);
        }
        Err(e) => {
            crash(
                format!("Copy {} to {}: Failed with error {}", path, destpath, e),
                1,
            );
        }
    }
}

pub fn append_file(path: &str, content: &str) -> std::io::Result<()> {
    log::info!("Append '{}' to file {}", content.trim_end(), path);
    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write_all("\n".as_bytes())?;
    file.write_all(content.as_bytes())?;
    file.write("\n".as_bytes())?;
    Ok(())
}

pub fn create_directory(path: &str) -> std::io::Result<()> {
    std::fs::create_dir(path)
}
