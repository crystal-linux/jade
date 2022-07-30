use crate::internal::*;
use crate::functions::partition::mount;
use crate::functions::partition::umount;
use std::process::Command;

pub fn install(pkgs: Vec<&str>) {
    exec_eval(
        Command::new("pacstrap").arg("/mnt").args(&pkgs).status(),
        format!("Install packages {}", pkgs.join(", ")).as_str(),
    );
    umount("/mnt/dev");
}
