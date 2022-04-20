use crate::internal::exec::*;
use crate::internal::*;
use std::path::PathBuf;

pub fn install_base_packages() {
    std::fs::create_dir_all("/mnt/etc").unwrap();
    files::copy_file("/etc/pacman.conf", "/mnt/etc/pacman.conf");
    install::install(vec![
        "base",
        "linux",
        "linux-firmware",
        "systemd-sysvcompat",
        "networkmanager",
        "man-db",
        "man-pages",
        "texinfo",
        "micro",
        "sudo",
        "curl",
        "archlinux-keyring",
        "neofetch",
        "btrfs-progs",
        "which",
    ]);
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
    install::install(vec!["grub", "efibootmgr", "grub-btrfs"]);
    if !efidir.exists() {
        crash(format!("The efidir {efidir:?} doesn't exist"), 1);
    }
    let efidir = efidir.to_string_lossy().to_string();
    exec_eval(
        exec_chroot(
            "grub-install",
            vec![
                String::from("--target=x86_64-efi"),
                format!("--efi-directory={}", efidir),
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
                format!("--efi-directory={}", efidir),
                String::from("--bootloader-id=crystal"),
            ],
        ),
        "install grub as efi without --removable",
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
    install::install(vec!["grub", "grub-btrfs"]);
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
