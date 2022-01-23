use std::process::Command;

pub fn install(pkgs: Vec<&str>) {
    Command::new("crystalstrap")
        .arg("/mnt")
        .args(pkgs)
        .output()
        .expect("Failed to install packages");
}
