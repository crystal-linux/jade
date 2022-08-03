use crate::internal::exec::*;
use crate::internal::*;
use std::process::Command;

pub fn new_user(username: &str, hasroot: bool, password: &str, do_hash_pass: bool, shell: &str) {
    let mut shell: &str = shell;
    if do_hash_pass {
        let hashed_pass = &*hash_pass(password).stdout;
        let _password = match std::str::from_utf8(hashed_pass) {
            Ok(v) => v,
            Err(e) => panic!("Failed to hash password, invalid UTF-8 sequence {}", e),
        };
    }
    if shell == "fish" {
         exec_eval(
            exec_chroot(
                "bash",
                vec![
                    String::from("pacman -S fish --noconfirm"),
                ],
            ),
            "installed fish",
        );
    }
    if shell == "zsh" {
        exec_eval(
            exec_chroot(
                "bash",
                vec![
                    String::from("pacman -S zsh --noconfirm"),
                ],
            ),
            "installed zsh",
        );
    }
    else if shell == "tcsh" || shell == "csh" {
        exec_eval(
            exec_chroot(
                "bash",
                vec![
                    String::from("pacman -S tcsh --noconfirm"),
                ],
            ),
            "installed tcsh and csh",
        );
    }
    else {
        exec_eval(
            exec_chroot(
                "bash",
                vec![
                    String::from("pacman -S fish --noconfirm"),
                ],
            ),
            "no shell or unknown shell specified, installed fish",
        );
        shell = "fish";
    }
    let shell_path = match shell {
        "bash" => "/bin/bash",
        "csh" => "/usr/bin/csh",
        "fish" => "/usr/bin/fish",
        "tcsh" => "/usr/bin/tcsh",
        "zsh" => "/usr/bin/zsh",
        &_ => "/usr/bin/fish",
    };
    exec_eval(
        exec_chroot(
            "useradd",
            vec![
                String::from("-m"),
                String::from("-s"),
                String::from(shell_path),
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
        .args(["passwd", "-6", password])
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

