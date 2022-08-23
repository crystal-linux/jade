use crate::args;
use crate::args::{DesktopSetup, PartitionMode};
use crate::functions::*;
use crate::internal::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
    flatpak: bool,
    zramd: bool,
    extra_packages: Vec<String>,
    unakite: Unakite,
    kernel: String,
}

#[derive(Serialize, Deserialize)]
struct Partition {
    device: String,
    mode: PartitionMode,
    efi: bool,
    partitions: Vec<String>,
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
    shell: String,
}

#[derive(Serialize, Deserialize)]
struct Unakite {
    enable: bool,
    root: String,
    oldroot: String,
    efidir: String,
    bootdev: String,
}

pub fn read_config(configpath: PathBuf) {
    let data = std::fs::read_to_string(&configpath);
    match &data {
        Ok(_) => {
            log::debug!("[ \x1b[2;1;32mOK\x1b[0m ] Read config file {configpath:?}");
        }
        Err(e) => {
            crash(
                format!("Read config file {configpath:?}  ERROR: {}", e),
                e.raw_os_error().unwrap(),
            );
        }
    }
    let config: std::result::Result<Config, serde_json::Error> =
        serde_json::from_str(&data.unwrap());
    match &config {
        Ok(_) => {
            log::debug!("[ \x1b[2;1;32mOK\x1b[0m ] Parse config file {configpath:?}",);
        }
        Err(e) => {
            crash(format!("Parse config file {configpath:?}  ERROR: {}", e), 1);
        }
    }
    let config: Config = config.unwrap();
    log::info!("Block device to use : /dev/{}", config.partition.device);
    log::info!("Partitioning mode : {:?}", config.partition.mode);
    log::info!("Partitioning for EFI : {}", config.partition.efi);
    let mut partitions: Vec<args::Partition> = Vec::new();
    for partition in config.partition.partitions {
        partitions.push(args::Partition::new(
            partition.split(':').collect::<Vec<&str>>()[0].to_string(),
            partition.split(':').collect::<Vec<&str>>()[1].to_string(),
            partition.split(':').collect::<Vec<&str>>()[2].to_string(),
        ));
    }
    let device = PathBuf::from("/dev/").join(config.partition.device.as_str());
    partition::partition(
        device,
        config.partition.mode,
        config.partition.efi,
        &mut partitions,
        config.unakite.enable,
    );
    base::install_base_packages(config.kernel);
    base::genfstab();
    println!();
    log::info!("Installing bootloader : {}", config.bootloader.r#type);
    log::info!("Installing bootloader to : {}", config.bootloader.location);
    if config.bootloader.r#type == "grub-efi" {
        base::install_bootloader_efi(PathBuf::from(config.bootloader.location));
    } else if config.bootloader.r#type == "grub-legacy" {
        base::install_bootloader_legacy(PathBuf::from(config.bootloader.location));
    }
    println!();
    log::info!("Adding Locales : {:?}", config.locale.locale);
    log::info!("Using keymap : {}", config.locale.keymap);
    log::info!("Setting timezone : {}", config.locale.timezone);
    locale::set_locale(config.locale.locale.join(" "));
    locale::set_keyboard(config.locale.keymap.as_str());
    locale::set_timezone(config.locale.timezone.as_str());
    println!();
    log::info!("Hostname : {}", config.networking.hostname);
    log::info!("Enabling ipv6 : {}", config.networking.ipv6);
    network::set_hostname(config.networking.hostname.as_str());
    network::create_hosts();
    if config.networking.ipv6 {
        network::enable_ipv6();
    }
    println!();
    println!("---------");
    log::info!("Enabling zramd : {}", config.zramd);
    if config.zramd {
        base::install_zram();
    }
    println!();
    println!("---------");
    for i in 0..config.users.len() {
        log::info!("Creating user : {}", config.users[i].name);
        log::info!("Setting use password : {}", config.users[i].password);
        log::info!("Enabling root for user : {}", config.users[i].hasroot);
        log::info!("Setting user shell : {}", config.users[i].shell);
        users::new_user(
            config.users[i].name.as_str(),
            config.users[i].hasroot,
            config.users[i].password.as_str(),
            false,
            config.users[i].shell.as_str(),
        );
        println!("---------");
    }
    println!();
    log::info!("Setting root password : {}", config.rootpass);
    users::root_pass(config.rootpass.as_str());
    println!();
    log::info!("Installing desktop : {:?}", config.desktop);
    /*if let Some(desktop) = &config.desktop {
        desktops::install_desktop_setup(*desktop);
    }*/
    match config.desktop.to_lowercase().as_str() {
        // "onyx" => desktops::install_desktop_setup(DesktopSetup::Onyx),
        "kde" => desktops::install_desktop_setup(DesktopSetup::Kde),
        "mate" => desktops::install_desktop_setup(DesktopSetup::Mate),
        "gnome" => desktops::install_desktop_setup(DesktopSetup::Gnome),
        "cinnamon" => desktops::install_desktop_setup(DesktopSetup::Cinnamon),
        "xfce" => desktops::install_desktop_setup(DesktopSetup::Xfce),
        "budgie" => desktops::install_desktop_setup(DesktopSetup::Budgie),
        "enlightenment" => desktops::install_desktop_setup(DesktopSetup::Enlightenment),
        "lxqt" => desktops::install_desktop_setup(DesktopSetup::Lxqt),
        "sway" => desktops::install_desktop_setup(DesktopSetup::Sway),
        "i3-gaps" => desktops::install_desktop_setup(DesktopSetup::I3gaps),
        "herbstluftwm" => desktops::install_desktop_setup(DesktopSetup::Herbstluftwm),
        "awesome" => desktops::install_desktop_setup(DesktopSetup::Awesome),
        "bspwm" => desktops::install_desktop_setup(DesktopSetup::Bspwm),
        "none/diy" => desktops::install_desktop_setup(DesktopSetup::None),
        _ => log::info!("No desktop setup selected!"),
    }
    println!();
    log::info!("Enabling timeshift : {}", config.timeshift);
    if config.timeshift {
        base::setup_timeshift();
    }
    println!();
    log::info!("Enabling flatpak : {}", config.flatpak);
    if config.flatpak {
        base::install_flatpak();
    }
    log::info!("Extra packages : {:?}", config.extra_packages);
    let mut extra_packages: Vec<&str> = Vec::new();
    for i in 0..config.extra_packages.len() {
        extra_packages.push(config.extra_packages[i].as_str());
    }
    install(extra_packages);
    log::info!("Setup unakite");
    if config.partition.mode == PartitionMode::Auto
        && !config.partition.efi
        && config.unakite.enable
        && !config.partition.device.to_string().contains("nvme")
    {
        let root = PathBuf::from("/dev/").join(config.partition.device.as_str());
        unakite::setup_unakite(
            format!("{}2", root.to_str().unwrap()).as_str(),
            format!("{}3", root.to_str().unwrap()).as_str(),
            config.partition.efi,
            "/boot",
            format!("{}1", root.to_str().unwrap()).as_str(),
        )
    } else if config.partition.mode == PartitionMode::Auto
        && config.partition.efi
        && config.unakite.enable
        && !config.partition.device.to_string().contains("nvme")
    {
        let root = PathBuf::from("/dev/").join(config.partition.device.as_str());
        unakite::setup_unakite(
            format!("{}2", root.to_str().unwrap()).as_str(),
            format!("{}3", root.to_str().unwrap()).as_str(),
            config.partition.efi,
            "/boot/efi",
            format!("{}1", root.to_str().unwrap()).as_str(),
        )
    } else if config.unakite.enable {
        unakite::setup_unakite(
            &config.unakite.root,
            &config.unakite.oldroot,
            config.partition.efi,
            &config.unakite.efidir,
            &config.unakite.bootdev,
        );
    } else if config.partition.mode == PartitionMode::Auto
        && config.partition.efi
        && config.unakite.enable
        && config.partition.device.to_string().contains("nvme")
    {
        let root = PathBuf::from("/dev/").join(config.partition.device.as_str());
        unakite::setup_unakite(
            format!("{}p2", root.to_str().unwrap()).as_str(),
            format!("{}p3", root.to_str().unwrap()).as_str(),
            config.partition.efi,
            "/boot/efi",
            format!("{}p1", root.to_str().unwrap()).as_str(),
        )
    } else if config.partition.mode == PartitionMode::Auto
        && !config.partition.efi
        && config.unakite.enable
        && config.partition.device.to_string().contains("nvme")
    {
        let root = PathBuf::from("/dev/").join(config.partition.device.as_str());
        unakite::setup_unakite(
            format!("{}p2", root.to_str().unwrap()).as_str(),
            format!("{}p3", root.to_str().unwrap()).as_str(),
            config.partition.efi,
            "/boot",
            format!("{}p1", root.to_str().unwrap()).as_str(),
        )
    } else {
        log::info!("Unakite disabled");
    }
    println!("Installation finished! You may reboot now!")
}
