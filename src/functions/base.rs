use crate::internal::exec::*;
use crate::internal::*;

pub fn install_base_packages() {
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
        "timeshift",
        "timeshift-autosnap",
        "which",
    ]);
}

pub fn install_bootloader_efi(efidir: &str) {
    install::install(vec!["grub", "efibootmgr"]);
    exec_eval(
        exec_chroot(
            "grub-install",
            vec![
                String::from("--target=x86_64-efi"),
                format!("--efi-directory={}", efidir),
                String::from("--bootloader-id=crystal"),
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
    install::install(vec!["grub"]);
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
