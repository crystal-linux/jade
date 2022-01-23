use crate::internal::exec::*;
use crate::internal::*;

pub fn new_user(username: &str, hasroot: bool, password: &str) {
    exec_eval(exec_chroot(
        "useradd",
        vec![
            String::from("-m"),
            String::from("-s"),
            String::from("/bin/bash"),
            String::from(username),
        ],
    ), format!("Create user {}", username).as_str());
    if hasroot {
        exec_eval(exec_chroot(
            "usermod",
            vec![
                String::from("-a"),
                String::from("-G"),
                String::from("wheel"),
                String::from(username),
            ],
        ), format!("Add user {} to wheel group", username).as_str());
    }
    exec_eval(exec_chroot(
        "usermod",
        vec![
            String::from("--password"),
            String::from("$(echo"),
            format!("${}", password),
            String::from("|"),
            String::from("openssl"),
            String::from("passwd"),
            String::from("-1"),
            String::from("-stdin)"),
            String::from(username),
        ],
    ), format!("Set password for user {}", username).as_str());
}

pub fn root_pass(root_pass: &str) {
    println!("Setting root password to '{}'", root_pass);
    exec_eval(exec_chroot(
        "usermod",
        vec![
            String::from("--password"),
            String::from("$(echo"),
            format!("${{{}}}", root_pass),
            String::from("|"),
            String::from("openssl"),
            String::from("passwd"),
            String::from("-1"),
            String::from("-stdin)"),
            String::from("root"),
        ],
    ), "set root password");
}
