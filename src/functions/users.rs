use crate::internal::*;
use std::process::Command;

pub fn new_user(username: &str, hasroot: bool, password: &str) {
    let return_val = Command::new("useradd")
        .arg("-m")
        .arg("-s")
        .arg("/bin/bash")
        .arg(username)
        .output();
    match return_val {
        Ok(_) => {
            info(format!("Created user {}", username));
        }
        Err(e) => {
            crash(
                format!("Failed to create user {}, Error: {}", username, e),
                1,
            );
        }
    }
    if hasroot {
        let return_val = Command::new("usermod")
            .arg("-a")
            .arg("-G")
            .arg("wheel")
            .arg(username)
            .output();
        match return_val {
            Ok(_) => {
                info(format!("Added user {} to group wheel", username));
            }
            Err(e) => {
                crash(format!("Failed to add user {}, Error: {}", username, e), 1);
            }
        }
    }
    let return_val = Command::new("arch-chroot")
        .arg("/mnt")
        .arg("usermod")
        .arg("--password")
        .arg("$(echo")
        .arg(format!("${{{}}}", password))
        .arg("|")
        .arg("openssl")
        .arg("passwd")
        .arg("-1")
        .arg("-stdin)")
        .arg(username)
        .output();
    match return_val {
        Ok(_) => {
            info(format!("Set password for user {}", username));
        }
        Err(e) => {
            crash(
                format!("Failed to set password for user {}, Error: {}", username, e),
                1,
            );
        }
    }
}

pub fn root_pass(root_pass: &str) {
    println!("Setting root password to '{}'", root_pass);
    let return_val = Command::new("arch-chroot")
        .arg("/mnt")
        .arg("usermod")
        .arg("--password")
        .arg("$(echo")
        .arg(format!("${{{}}}", root_pass))
        .arg("|")
        .arg("openssl")
        .arg("passwd")
        .arg("-1")
        .arg("-stdin)")
        .arg("root")
        .output();
    match return_val {
        Ok(_) => {
            info("Set root password".to_string());
        }
        Err(e) => {
            crash(format!("Failed to set root password, Error: {}", e), 1);
        }
    }
}
