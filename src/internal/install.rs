use crate::functions::partition::umount;
use crate::internal::*;
use std::process::Command;

pub fn install(pkgs: Vec<&str>) {
    exec_eval(
        Command::new("pacstrap").arg("/mnt").args(&pkgs).status(),
        format!("Install packages {}", pkgs.join(", ")).as_str(),
    );
    umount("/mnt/dev");
}
