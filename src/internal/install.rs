use std::process::Command;

pub fn install(pkgs: Vec<&str>) {
    Command::new("pacman").arg("-S").args(pkgs).output().expect("Failed to install packages");
}