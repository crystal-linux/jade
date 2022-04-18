use crate::functions::*;
use crate::internal::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    partition: Partition,
    bootloader: Bootloader,
    locale: Locale,
    networking: Networking,
    users: Vec<Users>,
    rootpass: String,
    desktop: String,
    timeshift: bool,
    extra_packages: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Partition {
    device: String,
    mode: String,
    efi: bool,
}

#[derive(Serialize, Deserialize)]
struct Bootloader {
    r#type: String,
    location: String,
}

#[derive(Serialize, Deserialize)]
struct Locale {
    locale: Vec<String>,
    keymap: String,
    timezone: String,
}

#[derive(Serialize, Deserialize)]
struct Networking {
    hostname: String,
    ipv6: bool,
}

#[derive(Serialize, Deserialize)]
struct Users {
    name: String,
    password: String,
    hasroot: bool,
}

pub fn read_config(configpath: &str) {
    let data = std::fs::read_to_string(configpath);
    match &data {
        Ok(_) => {
            log(format!(
                "[ \x1b[2;1;32mOK\x1b[0m ] {}",
                format!("Read config file {}", configpath).as_str()
            ));
        }
        Err(e) => {
            crash(
                format!(
                    "{}  ERROR: {}",
                    format!("Read config file {}", configpath).as_str(),
                    e
                ),
                e.raw_os_error().unwrap(),
            );
        }
    }
    let config: std::result::Result<Config, serde_json::Error> =
        serde_json::from_str(&data.unwrap());
    match &config {
        Ok(_) => {
            log(format!(
                "[ \x1b[2;1;32mOK\x1b[0m ] {}",
                format!("Parse config file {}", configpath).as_str()
            ));
        }
        Err(e) => {
            crash(
                format!(
                    "{}  ERROR: {}",
                    format!("Parse config file {}", configpath).as_str(),
                    e
                ),
                1,
            );
        }
    }
    let config: Config = config.unwrap();

    info(format!(
        "Block device to use : /dev/{}",
        config.partition.device
    ));
    info(format!("Partitioning mode : {}", config.partition.mode));
    info(format!("Partitioning for EFI : {}", config.partition.efi));
    partition::partition(
        format!("/dev/{}", config.partition.device).as_str(),
        config.partition.mode.as_str(),
        config.partition.efi,
    );
    base::install_base_packages();
    base::genfstab();
    println!();
    info(format!(
        "Installing bootloader : {}",
        config.bootloader.r#type
    ));
    info(format!(
        "Installing bootloader to : {}",
        config.bootloader.location
    ));
    if config.bootloader.r#type == "grub-efi" {
        base::install_bootloader_efi(config.bootloader.location.as_str());
    } else if config.bootloader.r#type == "grub-legacy" {
        base::install_bootloader_legacy(config.bootloader.location.as_str());
    }
    println!();
    info(format!("Adding Locales : {:?}", config.locale.locale));
    info(format!("Using keymap : {}", config.locale.keymap));
    info(format!("Setting timezone : {}", config.locale.timezone));
    locale::set_locale(config.locale.locale.join(" "));
    locale::set_keyboard(config.locale.keymap.as_str());
    locale::set_timezone(config.locale.timezone.as_str());
    println!();
    info(format!("Hostname : {}", config.networking.hostname));
    info(format!("Enabling ipv6 : {}", config.networking.ipv6));
    network::set_hostname(config.networking.hostname.as_str());
    network::create_hosts();
    if config.networking.ipv6 {
        network::enable_ipv6();
    }
    println!();
    println!("---------");
    for i in 0..config.users.len() {
        info(format!("Creating user : {}", config.users[i].name));
        info(format!(
            "Setting use password : {}",
            config.users[i].password
        ));
        info(format!(
            "Enabling root for user : {}",
            config.users[i].hasroot
        ));
        users::new_user(
            config.users[i].name.as_str(),
            config.users[i].hasroot,
            config.users[i].password.as_str(),
        );
        println!("---------");
    }
    println!();
    info(format!("Setting root password : {}", config.rootpass));
    users::root_pass(config.rootpass.as_str());
    println!();
    info(format!("Installing desktop : {}", config.desktop));
    if config.desktop == "none" || config.desktop.is_empty() {
        desktops::choose_pkgs(config.desktop.as_str());
    }
    println!();
    info(format!("Enabling timeshift : {}", config.timeshift));
    if config.timeshift {
        base::setup_timeshift();
    }
    info(format!("Extra packages : {:?}", config.extra_packages));
    let mut extra_packages: Vec<&str> = Vec::new();
    for i in 0..config.extra_packages.len() {
        extra_packages.push(config.extra_packages[i].as_str());
    }
    install(extra_packages);
    println!("Installation finished! You may reboot now!")
}
