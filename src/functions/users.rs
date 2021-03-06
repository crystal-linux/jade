use crate::internal::exec::*;
use crate::internal::*;
use std::process::Command;

pub fn new_user(username: &str, hasroot: bool, password: &str, do_hash_pass: bool) {
    if do_hash_pass {
        let hashed_pass = &*hash_pass(password).stdout;
        let _password = match std::str::from_utf8(hashed_pass) {
            Ok(v) => v,
            Err(e) => panic!("Failed to hash password, invalid UTF-8 sequence {}", e),
        };
    }
    exec_eval(
        exec_chroot(
            "useradd",
            vec![
                String::from("-m"),
                String::from("-s"),
                String::from("/bin/bash"),
                String::from("-p"),
                String::from(password).replace('\n', ""),
                String::from(username),
            ],
        ),
        format!("Create user {}", username).as_str(),
    );
    if hasroot {
        exec_eval(
            exec_chroot(
                "usermod",
                vec![
                    String::from("-aG"),
                    String::from("wheel"),
                    String::from(username),
                ],
            ),
            format!("Add user {} to wheel group", username).as_str(),
        );
        files_eval(
            files::append_file("/mnt/etc/sudoers", "\n%wheel ALL=(ALL) ALL\n"),
            "Add wheel group to sudoers",
        );
        files_eval(
            files::append_file("/mnt/etc/sudoers", "\nDefaults pwfeedback\n"),
            "Add pwfeedback to sudoers",
        );
    }
}

pub fn hash_pass(password: &str) -> std::process::Output {
    let output = Command::new("openssl")
        .args(["passwd", "-1", password])
        .output()
        .expect("Failed to hash password");
    output
}

pub fn root_pass(root_pass: &str) {
    exec_eval(
        exec_chroot(
            "bash",
            vec![
                String::from("-c"),
                format!(r#"'usermod --password {root_pass} root'"#),
            ],
        ),
        "set root password",
    );
}
