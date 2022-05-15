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
                vec![String::from(oldroot)],
            ),
            "Unmount old root",
        );
        mount(root, "/mnt", "");
        exec_eval(
            exec(
                "mkdir",
                vec![
                    String::from("-p"),
                    String::from(efidir),
                ],
            ),
            format!("Creating mountpoint {efidir} for {bootdev}").as_str(),
        );
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
                vec![String::from(root)],
            ),
            "Unmount unakite root",
        );
        mount(oldroot, "/mnt", "");
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
                vec![String::from(oldroot)],
            ),
            "Unmount old root",
        );
        mount(root, "/mnt", "");
        exec_eval(
            exec(
                "mkdir",
                vec![
                    String::from("-p"),
                    String::from("/mnt/boot"),
                ],
            ),
            format!("Creating mountpoint /boot for {bootdev}").as_str(),
        );
        mount(bootdev, "/mnt/boot", "");
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
                vec![String::from(root)],
            ),
            "Unmount unakite root",
        );
        mount(oldroot, "/mnt", "");
        mount(bootdev, "/mnt/boot", "");
    } else {
        panic!("Unknown state");
    }
}

pub fn setup_unakite(root: &str, oldroot: &str, efi: bool, efidir: &str, bootdev: &str) {
    log::debug!("Setting up Unakite");
    remount(root, oldroot, efi, efidir, bootdev, true);
    base::install_base_packages();
    base::genfstab();
    locale::set_locale("en_US.UTF-8 UTF-8".to_string());
    locale::set_timezone("Europe/Berlin"); // TODO: get the proper timezone
    network::set_hostname("unakite");
    network::create_hosts();
    users::new_user(
        "unakite",
        true,
        "Cp7oN04ZY0PsA", // unakite
    );
    exec_eval(
        exec(
            "sed",
            vec![
                String::from("-i"),
                String::from("-e"),
                String::from("s/crystal/unakite/g"),
                String::from("/mnt/etc/os-release"),
            ],
        ),
        "Change os-release",
    );
    exec_eval(
        exec(
            "sed",
            vec![
                String::from("-i"),
                String::from("-e"),
                String::from("s/Crystal/Unakite/g"),
                String::from("/mnt/etc/os-release"),
            ],
        ),
        "Change os-release",
    );
    if efi {
        install_bootloader_efi(PathBuf::from(efidir.replace("/mnt", "")));
    }
    users::root_pass("Cp7oN04ZY0PsA"); // unakite
    desktops::install_desktop_setup(DesktopSetup::Xfce);
    install(vec!["gparted", "firefox"]);
    remount(root, oldroot, efi, efidir, bootdev, false);
    exec_eval(
        exec_chroot(
            "grub-mkconfig",
            vec![String::from("-o"), String::from("/boot/grub/grub.cfg")],
        ),
        "Recreate grub.cfg in crystal"
    );
}