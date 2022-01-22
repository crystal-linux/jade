use crate::internal::exec::*;
use crate::internal::*;

pub fn new_user(username: &str, hasroot: bool, password: &str) {
    let return_val = exec(
        "useradd",
        vec![
            String::from("-m"),
            String::from("-s"),
            String::from("/bin/bash"),
            String::from(username),
        ],
    );
    match return_val {
        Ok(_) => {
            log(format!("Created user {}", username));
        }
        Err(e) => {
            crash(
                format!("Failed to create user {}, Error: {}", username, e),
                1,
            );
        }
    }
    if hasroot {
        let return_val = exec(
            "usermod",
            vec![
                String::from("-a"),
                String::from("-G"),
                String::from("wheel"),
                String::from(username),
            ],
        );
        match return_val {
            Ok(_) => {
                log(format!("Added user {} to group wheel", username));
            }
            Err(e) => {
                crash(format!("Failed to add user {}, Error: {}", username, e), 1);
            }
        }
    }
    let return_val = exec(
        "arch-chroot",
        vec![
            String::from("/mnt"),
            String::from("usermod"),
            String::from("--password"),
            String::from("$(echo"),
            String::from(format!("${}", password)),
            String::from("|"),
            String::from("openssl"),
            String::from("passwd"),
            String::from("-1"),
            String::from("-stdin)"),
            String::from(username),
        ],
    );
    match return_val {
        Ok(_) => {
            log(format!("Set password for user {}", username));
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
    let return_val = exec(
        "arch-chroot",
        vec![
            String::from("/mnt"),
            String::from("usermod"),
            String::from("--password"),
            String::from("$(echo"),
            String::from(format!("${{{}}}", root_pass)),
            String::from("|"),
            String::from("openssl"),
            String::from("passwd"),
            String::from("-1"),
            String::from("-stdin)"),
            String::from("root"),
        ],
    );
    match return_val {
        Ok(_) => {
            log("Set root password".to_string());
        }
        Err(e) => {
            crash(format!("Failed to set root password, Error: {}", e), 1);
        }
    }
}
