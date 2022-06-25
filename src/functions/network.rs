use crate::internal::*;

pub fn set_hostname(hostname: &str) {
    println!("Setting hostname to {}", hostname);
    files::create_file("/mnt/etc/hostname");
    files_eval(
        files::append_file("/mnt/etc/hostname", hostname),
        "set hostname",
    );
}

pub fn create_hosts() {
    files::create_file("/mnt/etc/hosts");
    files_eval(
        files::append_file("/mnt/etc/hosts", "127.0.0.1     localhost"),
        "create /etc/hosts",
    );
}

pub fn enable_ipv6() {
    files_eval(
        files::append_file("/mnt/etc/hosts", "::1 localhost"),
        "add ipv6 localhost",
    );
}
