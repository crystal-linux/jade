use crate::internal::exec::*;
use crate::internal::*;
use crate::functions::*;
use crate::functions::partition::mount;
use std::path::PathBuf;
use crate::args::DesktopSetup;
pub fn install_bootloader_efi(efidir: PathBuf) {
    install::install(vec!["grub", "efibootmgr", "grub-btrfs", "crystal-grub-theme"]);
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
                format!("--efi-directory={}" , efi_str),
                String::from("--bootloader-id=unakite"),
                String::from("--removable"),
            ],
        ),
        "install unakite grub as efi with --removable",
    );
    exec_eval(
        exec_chroot(
            "grub-install",
            vec![
                String::from("--target=x86_64-efi"),
                format!("--efi-directory={}", efi_str),
                String::from("--bootloader-id=unakite"),
            ],
        ),
        "install unakite grub as efi without --removable",
    );
    files_eval(
        files::append_file("/mnt/etc/default/grub", "GRUB_THEME=\"/usr/share/grub/themes/crystal/theme.txt\""),
        "enable crystal grub theme"
    );
    exec_eval(
        exec_chroot(
            "grub-mkconfig",
            vec![String::from("-o"), String::from("/boot/grub/grub.cfg")],
        ),
        "create grub.cfg",
    );
}

pub fn remount(root: &str, oldroot: &str, efi: bool, efidir: &str, bootdev: &str, firstrun: bool) {
    if efi && firstrun {
        exec_eval(
            exec(
                "umount",
                vec![String::from(bootdev)],
            ),
            &format!("Unmount {}", bootdev),
        );
        exec_eval(
            exec(
                "umount",
                vec![String::from("/")],
            ),
            "Unmount old root",
        );
        mount(root, "/", "");
        mount(bootdev, efidir, "");
    } else if efi && !firstrun {
        exec_eval(
            exec(
                "umount",
                vec![String::from(bootdev)],
            ),
            &format!("Unmount {}", bootdev),
        );
        exec_eval(
            exec(
                "umount",
                vec![String::from("/")],
            ),
            "Unmount unakite root",
        );
        mount(root, "/", "");
        mount(bootdev, efidir, "");
    } else if !efi && firstrun {
        exec_eval(
            exec(
                "umount",
                vec![String::from(bootdev)],
            ),
            &format!("Unmount {}", bootdev),
        );
        exec_eval(
            exec(
                "umount",
                vec![String::from("/")],
            ),
            "Unmount old root",
        );
        mount(root, "/", "");
        mount(bootdev, "/boot", "");
    } else if !efi && !firstrun {
        exec_eval(
            exec(
                "umount",
                vec![String::from(bootdev)],
            ),
            &format!("Unmount {}", bootdev),
        );
        exec_eval(
            exec(
                "umount",
                vec![String::from("/")],
            ),
            "Unmount unakite root",
        );
        mount(oldroot, "/", "");
        mount(bootdev, "/boot", "");
    } else {
        panic!("Unknown state");
    }
}

pub fn setup_unakite(root: &str, oldroot: &str, efi: bool, efidir: &str, bootdev: &str) {
    log::debug!("Setting up Unakite");
    remount(root, oldroot, efi, efidir, bootdev, true);
    base::install_base_packages();
    base::genfstab();
    if efi {
        install_bootloader_efi(PathBuf::from(efidir));
    }
    locale::set_locale("".to_string());
    locale::set_timezone("Europe/Berlin"); // TODO: get the proper timezone
    network::set_hostname("unakite");
    network::create_hosts();
    users::new_user(
        "unakite",
        true,
        "Cp7oN04ZY0PsA", // unakite
    );
    users::root_pass("Cp7oN04ZY0PsA"); // unakite
    desktops::install_desktop_setup(DesktopSetup::Xfce);
    install(vec!["gparted", "firefox"]);
    remount(root, oldroot, efi, efidir, bootdev, false);
}