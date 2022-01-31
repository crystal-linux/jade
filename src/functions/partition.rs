use crate::internal::exec::*;
use crate::internal::*;

pub fn partition(device: &str, mode: &str, efi: bool) {
    if mode == "manual" {
        log("Manual partitioning".to_string());
    } else {
        log(format!("automatically partitioning {}", device));
        if efi {
            exec_eval(
                exec(
                    "parted",
                    vec![
                        String::from("-s"),
                        String::from(device),
                        String::from("mklabel"),
                        String::from("gpt"),
                    ],
                ),
                format!("create gpt label on {}", device).as_str(),
            );
            exec_eval(
                exec(
                    "parted",
                    vec![
                        String::from("-s"),
                        String::from(device),
                        String::from("mkpart"),
                        String::from("fat32"),
                        String::from("0"),
                        String::from("300"),
                    ],
                ),
                "create EFI partition",
            );
            exec_eval(
                exec(
                    "parted",
                    vec![
                        String::from("-s"),
                        String::from(device),
                        String::from("mkpart"),
                        String::from("btrfs"),
                        String::from("300"),
                        String::from("100%"),
                    ],
                ),
                "Create btrfs root partition",
            );
        } else {
            exec_eval(
                exec(
                    "parted",
                    vec![
                        String::from("-s"),
                        String::from(device),
                        String::from("mklabel"),
                        String::from("msdos"),
                    ],
                ),
                format!("Create msdos label on {}", device).as_str(),
            );
            exec_eval(
                exec(
                    "parted",
                    vec![
                        String::from("-s"),
                        String::from(device),
                        String::from("mkpart"),
                        String::from("primary"),
                        String::from("ext4"),
                        String::from("1MIB"),
                        String::from("512MIB"),
                    ],
                ),
                "create bios boot partition",
            );
            exec_eval(
                exec(
                    "parted",
                    vec![
                        String::from("-s"),
                        String::from(device),
                        String::from("mkpart"),
                        String::from("primary"),
                        String::from("btrfs"),
                        String::from("512MIB"),
                        String::from("100%"),
                    ],
                ),
                "create btrfs root partition",
            );
        }
    }
    if device.contains("nvme") {
        part_nvme(device, efi);
    } else {
        part_disk(device, efi);
    }
}

fn part_nvme(device: &str, efi: bool) {
    if efi {
        exec_eval(
            exec("mkfs.vfat", vec![format!("{}p1", device)]),
            format!("format {}p1 as fat32", device).as_str(),
        );
        exec_eval(
            exec("mkfs.btrfs", vec![format!("{}p2", device)]),
            format!("format {}p2 as btrfs", device).as_str(),
        );
        mount(format!("{}p2", device).as_str(), "/mnt", "");
        exec_eval(
            exec_workdir(
                "btrfs",
                "/mnt",
                vec![
                    String::from("subvolume"),
                    String::from("create"),
                    String::from("@"),
                ],
            ),
            "Create btrfs subvolume @",
        );
        exec_eval(
            exec_workdir(
                "btrfs",
                "/mnt",
                vec![
                    String::from("subvolume"),
                    String::from("create"),
                    String::from("@home"),
                ],
            ),
            "Create btrfs subvolume @home",
        );
        umount("/mnt");
        mount(format!("{}p2", device).as_str(), "/mnt/", "subvol=@");
        files_eval(files::create_directory("/mnt/boot"), "create /mnt/boot");
        files_eval(
            files::create_directory("/mnt/boot/efi"),
            "create /mnt/boot/efi",
        );
        files_eval(files::create_directory("/mnt/home"), "create /mnt/home");
        mount(
            format!("{}p2", device).as_str(),
            "/mnt/home",
            "subvol=@home",
        );
        mount(format!("{}p1", device).as_str(), "/mnt/boot/efi", "");
    } else {
        exec_eval(
            exec("mkfs.ext4", vec![format!("{}p1", device)]),
            format!("format {}p1 as ext4", device).as_str(),
        );
        exec_eval(
            exec("mkfs.btrfs", vec![format!("{}p2", device)]),
            format!("format {}p2 as btrfs", device).as_str(),
        );
        mount(format!("{}p2", device).as_str(), "/mnt/", "");
        exec_eval(
            exec_workdir(
                "btrfs",
                "/mnt",
                vec![
                    String::from("subvolume"),
                    String::from("create"),
                    String::from("@"),
                ],
            ),
            "Create btrfs subvolume @",
        );
        exec_eval(
            exec_workdir(
                "btrfs",
                "/mnt",
                vec![
                    String::from("subvolume"),
                    String::from("create"),
                    String::from("@home"),
                ],
            ),
            "Create btrfs subvolume @home",
        );
        umount("/mnt");
        mount(format!("{}p2", device).as_str(), "/mnt/", "subvol=@");
        files_eval(files::create_directory("/mnt/boot"), "create /mnt/boot");
        files_eval(files::create_directory("/mnt/home"), "create /mnt/home");
        mount(
            format!("{}p2", device).as_str(),
            "/mnt/home",
            "subvol=@home",
        );
        mount(format!("{}p1", device).as_str(), "/mnt/boot", "");
    }
}

fn part_disk(device: &str, efi: bool) {
    if efi {
        exec_eval(
            exec("mkfs.vfat", vec![format!("{}1", device)]),
            format!("format {}1 as fat32", device).as_str(),
        );
        exec_eval(
            exec("mkfs.btrfs", vec!["-f".to_string(), format!("{}2", device)]),
            format!("format {}2 as btrfs", device).as_str(),
        );
        mount(format!("{}2", device).as_str(), "/mnt", "");
        exec_eval(
            exec_workdir(
                "btrfs",
                "/mnt",
                vec![
                    String::from("subvolume"),
                    String::from("create"),
                    String::from("@"),
                ],
            ),
            "Create btrfs subvolume @",
        );
        exec_eval(
            exec_workdir(
                "btrfs",
                "/mnt",
                vec![
                    String::from("subvolume"),
                    String::from("create"),
                    String::from("@home"),
                ],
            ),
            "Create btrfs subvolume @home",
        );
        umount("/mnt");
        mount(format!("{}2", device).as_str(), "/mnt/", "subvol=@");
        files_eval(files::create_directory("/mnt/boot"), "create /mnt/boot");
        files_eval(
            files::create_directory("/mnt/boot/efi"),
            "create /mnt/boot/efi",
        );
        files_eval(files::create_directory("/mnt/home"), "create /mnt/home");
        mount(format!("{}2", device).as_str(), "/mnt/home", "subvol=@home");
        mount(format!("{}1", device).as_str(), "/mnt/boot/efi", "");
    } else {
        exec_eval(
            exec("mkfs.ext4", vec![format!("{}1", device)]),
            format!("format {}1 as ext4", device).as_str(),
        );
        exec_eval(
            exec("mkfs.btrfs", vec!["-f".to_string(), format!("{}2", device)]),
            format!("format {}2 as btrfs", device).as_str(),
        );
        mount(format!("{}2", device).as_str(), "/mnt/", "");
        exec_eval(
            exec_workdir(
                "btrfs",
                "/mnt",
                vec![
                    String::from("subvolume"),
                    String::from("create"),
                    String::from("@"),
                ],
            ),
            "Create btrfs subvolume @",
        );
        exec_eval(
            exec_workdir(
                "btrfs",
                "/mnt",
                vec![
                    String::from("subvolume"),
                    String::from("create"),
                    String::from("@home"),
                ],
            ),
            "create btrfs subvolume @home",
        );
        umount("/mnt");
        mount(format!("{}2", device).as_str(), "/mnt/", "subvol=@");
        files_eval(
            files::create_directory("/mnt/boot"),
            "create directory /mnt/boot",
        );
        files_eval(
            files::create_directory("/mnt/home"),
            "create directory /mnt/home",
        );
        mount(format!("{}2", device).as_str(), "/mnt/home", "subvol=@home");
        mount(format!("{}1", device).as_str(), "/mnt/boot", "");
    }
}

fn mount(partition: &str, mountpoint: &str, options: &str) {
    if !options.is_empty() {
        exec_eval(
            exec(
                "mount",
                vec![
                    String::from(partition),
                    String::from(mountpoint),
                    String::from("-o"),
                    String::from(options),
                ],
            ),
            format!(
                "mount {} with options {} at {}",
                partition, options, mountpoint
            )
            .as_str(),
        );
    } else {
        exec_eval(
            exec(
                "mount",
                vec![String::from(partition), String::from(mountpoint)],
            ),
            format!("mount {} with no options at {}", partition, mountpoint).as_str(),
        );
    }
}

fn umount(mountpoint: &str) {
    exec_eval(
        exec("umount", vec![String::from(mountpoint)]),
        format!("unmount {}", mountpoint).as_str(),
    );
}
