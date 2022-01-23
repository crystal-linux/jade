use std::process::Command;
use crate::internal::*;

pub fn install(pkgs: Vec<&str>) {
    exec_eval(Command::new("crystalstrap")
        .arg("/mnt")
        .args(&pkgs)
        .status(), format!("Install packages {}", pkgs.join(", ")).as_str());
}
