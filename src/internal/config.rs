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

pub fn read_config() {
    let data =
        std::fs::read_to_string("example_config.json").expect("Unable to read example_config.json");
    let config: Config = serde_json::from_str(&data).expect("Unable to parse example_config.json");
    println!("---------Partition---------");
    println!("{}", config.partition.device);
    println!("{}", config.partition.mode);
    println!("{}", config.partition.efi);
    println!("---------Bootloader---------");
    println!("{}", config.bootloader.r#type);
    println!("{}", config.bootloader.location);
    println!("---------Locale---------");
    println!("{:?}", config.locale.locale);
    println!("{}", config.locale.keymap);
    println!("{}", config.locale.timezone);
    println!("---------Networking---------");
    println!("{}", config.networking.hostname);
    println!("{}", config.networking.ipv6);
    println!("---------Users---------");
    println!("---------");
    for i in 0..config.users.len() {
        println!("{}", config.users[i].name);
        println!("{}", config.users[i].password);
        println!("{}", config.users[i].hasroot);
        println!("---------");
    }
    println!("---------Rootpass---------");
    println!("{}", config.rootpass);
    println!("---------Desktop---------");
    println!("{}", config.desktop);
    println!("---------Timeshift---------");
    println!("{}", config.timeshift);
    println!("---------Extra packages---------");
    println!("{:?}", config.extra_packages);
}
