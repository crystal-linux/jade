use crate::internal::*;
use std::process::Command;

pub fn install(pkgs: Vec<&str>) {
    exec_eval(
        Command::new("pacstrap")
            .arg("/mnt")
            .arg("-C")
            .arg("/etc/pacman.conf")
            .args(&pkgs)
            .status(),
        format!("Install packages {}", pkgs.join(", ")).as_str(),
    );
}
