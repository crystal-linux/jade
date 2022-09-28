use crate::internal::exec::*;
use crate::internal::files::append_file;
use crate::internal::*;
use log::warn;
use std::path::PathBuf;

pub fn install_base_packages(kernel: String) {
    std::fs::create_dir_all("/mnt/etc").unwrap();
    let kernel_to_install = if kernel.is_empty() {
        "linux"
    } else {
        match kernel.as_str() {
            "linux" => "linux",
            "linux-lts" => "linux-lts",
            "linux-zen" => "linux-zen",
            "linux-hardened" => "linux-hardened",
            _ => {
                warn!("Unknown kernel: {}, using default instead", kernel);
                "linux"
            }
        }
    };
    install::install(vec![
        // Base Arch
        "base",
        kernel_to_install,
        "linux-firmware",
        "systemd-sysvcompat",
        "networkmanager",
        "man-db",
        "man-pages",
        "texinfo",
        "nano",
        "sudo",
        "curl",
        "archlinux-keyring",
        // Extra goodies
        "neofetch",
        "btrfs-progs",
        "which",
        "pkg-warner",
        "base-devel",
        // Fonts
        "noto-fonts",
        "noto-fonts-emoji",
        "noto-fonts-cjk",
        "noto-fonts-extra",
        "ttf-nerd-fonts-symbols-common",
        // Common packages for all desktops
        "xterm",
        "pipewire",
        "pipewire-pulse",
        "pipewire-alsa",
        "pipewire-jack",
    ]);
    files::copy_file("/etc/pacman.conf", "/mnt/etc/pacman.conf");
}

pub fn genfstab() {
    exec_eval(
        exec(
            "bash",
            vec![
                String::from("-c"),
                String::from("genfstab -U /mnt >> /mnt/etc/fstab"),
            ],
        ),
        "Generate fstab",
    );
}

pub fn install_bootloader_efi(efidir: PathBuf) {
    install::install(vec![
        "grub",
        "efibootmgr",
        "grub-btrfs",
        "crystal-grub-theme",
        "os-prober",
    ]);
    let efidir = std::path::Path::new("/mnt").join(efidir);
    let efi_str = efidir.to_str().unwrap();
    if !std::path::Path::new(&format!("/mnt{efi_str}")).exists() {
        crash(format!("The efidir {efidir:?} doesn't exist"), 1);
    }
    exec_eval(
        exec_chroot(
            "grub-install",
            vec![
                String::from("--target=x86_64-efi"),
                format!("--efi-directory={}", efi_str),
                String::from("--bootloader-id=crystal"),
                String::from("--removable"),
            ],
        ),
        "install grub as efi with --removable",
    );
    exec_eval(
        exec_chroot(
            "grub-install",
            vec![
                String::from("--target=x86_64-efi"),
                format!("--efi-directory={}", efi_str),
                String::from("--bootloader-id=crystal"),
            ],
        ),
        "install grub as efi without --removable",
    );
    files_eval(
        append_file(
            "/mnt/etc/default/grub",
            "GRUB_THEME=\"/usr/share/grub/themes/crystal/theme.txt\"",
        ),
        "enable crystal grub theme",
    );
    exec_eval(
        exec_chroot(
            "grub-mkconfig",
            vec![String::from("-o"), String::from("/boot/grub/grub.cfg")],
        ),
        "create grub.cfg",
    );
}

pub fn install_bootloader_legacy(device: PathBuf) {
    install::install(vec![
        "grub",
        "grub-btrfs",
        "crystal-grub-theme",
        "os-prober",
    ]);
    if !device.exists() {
        crash(format!("The device {device:?} does not exist"), 1);
    }
    let device = device.to_string_lossy().to_string();
    exec_eval(
        exec_chroot(
            "grub-install",
            vec![String::from("--target=i386-pc"), device],
        ),
        "install grub as legacy",
    );
    files_eval(
        append_file(
            "/mnt/etc/default/grub",
            "GRUB_THEME=\"/usr/share/grub/themes/crystal/theme.txt\"",
        ),
        "enable crystal grub theme",
    );
    exec_eval(
        exec_chroot(
            "grub-mkconfig",
            vec![String::from("-o"), String::from("/boot/grub/grub.cfg")],
        ),
        "create grub.cfg",
    );
}

pub fn setup_timeshift() {
    install(vec!["timeshift", "timeshift-autosnap"]);
    exec_eval(
        exec_chroot("timeshift", vec![String::from("--btrfs")]),
        "setup timeshift",
    )
}

pub fn install_homemgr() {
    install(vec!["nix"]);
}

pub fn install_flatpak() {
    install(vec!["flatpak"]);
    exec_eval(
        exec_chroot(
            "flatpak",
            vec![
                String::from("remote-add"),
                String::from("--if-not-exists"),
                String::from("flathub"),
                String::from("https://flathub.org/repo/flathub.flatpakrepo"),
            ],
        ),
        "add flathub remote",
    )
}

pub fn install_zram() {
    install(vec!["zramd"]);
    exec_eval(
        exec_chroot(
            "systemctl",
            vec![String::from("enable"), String::from("zramd")],
        ),
        "Enable zramd service",
    );
}
