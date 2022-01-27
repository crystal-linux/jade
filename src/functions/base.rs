use crate::internal::exec::*;
use crate::internal::*;

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

pub fn install_bootloader_efi(efidir: &str) {
    install::install(vec!["grub", "efibootmgr", "grub-btrfs"]);
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
        "install grub as efi",
    );
    exec_eval(
        exec_chroot(
            "grub-mkconfig",
            vec![String::from("-o"), String::from("/boot/grub/grub.cfg")],
        ),
        "create grub.cfg",
    );
}

pub fn install_bootloader_legacy(device: &str) {
    install::install(vec!["grub", "grub-btrfs"]);
    exec_eval(
        exec_chroot(
            "grub-install",
            vec![String::from("--target=i386-pc"), String::from(device)],
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
