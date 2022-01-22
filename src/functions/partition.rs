use crate::internal::exec::*;
use crate::internal::*;

pub fn partition(device: &str, mode: &str, efi: bool) {
    if mode == "manual" {
        log("Manual partitioning".to_string());
    } else {
        log(format!("automatically partitioning {}", device));
        if efi {
            returncode_eval(exec(
                "parted",
                vec![
                    String::from("-s"),
                    String::from(device),
                    String::from("mklabel"),
                    String::from("gpt"),
                ],
            ));
            returncode_eval(exec(
                "parted",
                vec![
                    String::from("-s"),
                    String::from(device),
                    String::from("mkpart"),
                    String::from("fat32"),
                    String::from("0"),
                    String::from("300"),
                ],
            ));
            returncode_eval(exec(
                "parted",
                vec![
                    String::from("-s"),
                    String::from(device),
                    String::from("mkpart"),
                    String::from("btrfs"),
                    String::from("300"),
                    String::from("100%"),
                ],
            ));
        } else {
            returncode_eval(exec(
                "parted",
                vec![
                    String::from("-s"),
                    String::from(device),
                    String::from("mklabel"),
                    String::from("msdos"),
                ],
            ));
            returncode_eval(exec(
                "parted",
                vec![
                    String::from("-s"),
                    String::from(device),
                    String::from("mkpart"),
                    String::from("btrfs"),
                    String::from("512MIB"),
                    String::from("100&"),
                ],
            ));
            returncode_eval(exec(
                "parted",
                vec![
                    String::from("-s"),
                    String::from(device),
                    String::from("mkpart"),
                    String::from("ext4"),
                    String::from("1MIB"),
                    String::from("512MIB"),
                ],
            ));
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
        returncode_eval(exec(
            "mkfs.vfat",
            vec![String::from(format!("{}p1", device))],
        ));
        returncode_eval(exec(
            "mkfs.btrfs",
            vec![String::from(format!("{}p2", device))],
        ));
        mount(format!("{}p2", device), "/mnt", "");
        returncode_eval(exec_workdir(
            "btrfs",
            "/mnt",
            vec![
                String::from("subvolume"),
                String::from("create"),
                String::from("@"),
            ],
        ));
        returncode_eval(exec_workdir(
            "btrfs",
            "/mnt",
            vec![
                String::from("subvolume"),
                String::from("create"),
                String::from("@home"),
            ],
        ));
        umount("/mnt");
        mount(format!("{}p2", device), "/mnt/", "subvol=@");
        files::create_directory("/mnt/boot");
        files::create_directory("/mnt/boot/efi");
        files::create_directory("/mnt/home");
        mount(format!("{}p2", device), "/mnt/home", "subvol=@home");
        mount(format!("{}p1", device), "/mnt/boot/efi", "");
    } else {
        returncode_eval(exec(
            "mkfs.ext4",
            vec![String::from(format!("{}p1", device))],
        ));
        returncode_eval(exec(
            "mkfs.btrfs",
            vec![String::from(format!("{}p2", device))],
        ));
        mount(format!("{}p2", device), "/mnt/", "");
        returncode_eval(exec_workdir(
            "btrfs",
            "/mnt",
            vec![
                String::from("subvolume"),
                String::from("create"),
                String::from("@"),
            ],
        ));
        returncode_eval(exec_workdir(
            "btrfs",
            "/mnt",
            vec![
                String::from("subvolume"),
                String::from("create"),
                String::from("@home"),
            ],
        ));
        umount("/mnt");
        mount(format!("{}p2", device), "/mnt/", "subvol=@");
        files::create_directory("/mnt/boot");
        files::create_directory("/mnt/home");
        mount(format!("{}p2", device), "/mnt/home", "subvol=@home");
        mount(format!("{}p1", device), "/mnt/boot", "");
    }
}

fn part_disk(device: &str, efi: bool) {
    if efi {
        returncode_eval(exec(
            "mkfs.vfat",
            vec![String::from(format!("{}1", device))],
        ));
        returncode_eval(exec(
            "mkfs.btrfs",
            vec![String::from(format!("{}2", device))],
        ));
        mount(format!("{}2", device), "/mnt", "");
        returncode_eval(exec_workdir(
            "btrfs",
            "/mnt",
            vec![
                String::from("subvolume"),
                String::from("create"),
                String::from("@"),
            ],
        ));
        returncode_eval(exec_workdir(
            "btrfs",
            "/mnt",
            vec![
                String::from("subvolume"),
                String::from("create"),
                String::from("@home"),
            ],
        ));
        umount("/mnt");
        mount(format!("{}2", device), "/mnt/", "subvol=@");
        files::create_directory("/mnt/boot");
        files::create_directory("/mnt/boot/efi");
        files::create_directory("/mnt/home");
        mount(format!("{}2", device), "/mnt/home", "subvol=@home");
        mount(format!("{}1", device), "/mnt/boot/efi", "");
    } else {
        returncode_eval(exec(
            "mkfs.ext4",
            vec![String::from(format!("{}1", device))],
        ));
        returncode_eval(exec(
            "mkfs.btrfs",
            vec![String::from(format!("{}2", device))],
        ));
        mount(format!("{}2", device), "/mnt/", "");
        returncode_eval(exec_workdir(
            "btrfs",
            "/mnt",
            vec![
                String::from("subvolume"),
                String::from("create"),
                String::from("@"),
            ],
        ));
        returncode_eval(exec_workdir(
            "btrfs",
            "/mnt",
            vec![
                String::from("subvolume"),
                String::from("create"),
                String::from("@home"),
            ],
        ));
        umount("/mnt");
        mount(format!("{}2", device), "/mnt/", "subvol=@");
        files::create_directory("/mnt/boot");
        files::create_directory("/mnt/home");
        mount(format!("{}2", device), "/mnt/home", "subvol=@home");
        mount(format!("{}1", device), "/mnt/boot", "");
    }
}

fn mount(partition: String, mountpoint: &str, options: &str) {
    let options = if options.is_empty() { "\"\"" } else { options };
    returncode_eval(exec(
        "mount",
        vec![
            String::from(partition),
            String::from(mountpoint),
            String::from("-o"),
            String::from(options),
        ],
    ));
}

fn umount(mountpoint: &str) {
    returncode_eval(exec("umount", vec![String::from(mountpoint)]));
}

fn returncode_eval(return_code: std::result::Result<std::process::Output, std::io::Error>) {
    match return_code {
        Ok(_) => {
            log("Success".to_string());
        }
        Err(e) => {
            crash(format!("Failed with error: {}", e), 1);
        }
    }
}
