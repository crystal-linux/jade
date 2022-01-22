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
        let devicep1 = format!("{}p1", device).as_str();
        let devicep2 = format!("{}p2", device).as_str();
        returncode_eval(exec("mkfs.vfat", vec![
            String::from(devicep1),
        ]));
        returncode_eval(exec("mkfs.btrfs", vec![
            String::from(devicep2),
        ]));
        mount(devicep2, "/mnt", "");
        returncode_eval(exec_workdir("btrfs", "/mnt", vec![
            String::from("subvolume"),
            String::from("create"),
            String::from("@"),
        ]));
        returncode_eval(exec_workdir("btrfs", "/mnt", vec![
            String::from("subvolume"),
            String::from("create"),
            String::from("@home"),
        ]));
        umount("/mnt");
        mount(devicep2, "/mnt/", "subvol=@");
        files::create_directory("/mnt/boot");
        files::create_directory("/mnt/boot/efi");
        files::create_directory("/mnt/home");
        mount(devicep2, "/mnt/home", "subvol=@home");
        mount(devicep1, "/mnt/boot/efi", "");
    } else {
        let devicep1 = format!("{}p1", device).as_str();
        let devicep2 = format!("{}p2", device).as_str();
        returncode_eval(exec("mkfs.ext4", vec![
            String::from(devicep1),
        ]));
        returncode_eval(exec("mkfs.btrfs", vec![
            String::from(devicep2),
        ]));
        mount(devicep2, "/mnt/", "");
        returncode_eval(exec_workdir("btrfs", "/mnt", vec![
            String::from("subvolume"),
            String::from("create"),
            String::from("@"),
        ]));
        returncode_eval(exec_workdir("btrfs", "/mnt", vec![
            String::from("subvolume"),
            String::from("create"),
            String::from("@home"),
        ]));
        umount("/mnt");
        mount(devicep2, "/mnt/", "subvol=@");
        files::create_directory("/mnt/boot");
        files::create_directory("/mnt/home");
        mount(devicep2, "/mnt/home", "subvol=@home");
        mount(devicep1, "/mnt/boot", "");
    }
}

fn part_disk(device: &str, efi: bool) {
    if efi {
        let device1 = format!("{}1", device).as_str();
        let device2 = format!("{}2", device).as_str();
        returncode_eval(exec("mkfs.vfat", vec![
            String::from(device1),
        ]));
        returncode_eval(exec("mkfs.btrfs", vec![
            String::from(device2),
        ]));
        mount(device2, "/mnt", "");
        returncode_eval(exec_workdir("btrfs", "/mnt", vec![
            String::from("subvolume"),
            String::from("create"),
            String::from("@"),
        ]));
        returncode_eval(exec_workdir("btrfs", "/mnt", vec![
            String::from("subvolume"),
            String::from("create"),
            String::from("@home"),
        ]));
        umount("/mnt");
        mount(device2, "/mnt/", "subvol=@");
        files::create_directory("/mnt/boot");
        files::create_directory("/mnt/boot/efi");
        files::create_directory("/mnt/home");
        mount(device2, "/mnt/home", "subvol=@home");
        mount(device1, "/mnt/boot/efi", "");
    } else {
        let device1 = format!("{}1", device).as_str();
        let device2 = format!("{}2", device).as_str();
        returncode_eval(exec("mkfs.ext4", vec![
            String::from(device1),
        ]));
        returncode_eval(exec("mkfs.btrfs", vec![
            String::from(device2),
        ]));
        mount(device2, "/mnt/", "");
        returncode_eval(exec_workdir("btrfs", "/mnt", vec![
            String::from("subvolume"),
            String::from("create"),
            String::from("@"),
        ]));
        returncode_eval(exec_workdir("btrfs", "/mnt", vec![
            String::from("subvolume"),
            String::from("create"),
            String::from("@home"),
        ]));
        umount("/mnt");
        mount(device2, "/mnt/", "subvol=@");
        files::create_directory("/mnt/boot");
        files::create_directory("/mnt/home");
        mount(device2, "/mnt/home", "subvol=@home");
        mount(device1, "/mnt/boot", "");
    }
}

fn mount(partition: &str, mountpoint: &str, options: &str) {
    let options = if options.is_empty() { "\"\"" } else { options };
    returncode_eval(exec("mount", vec![
        String::from(partition),
        String::from(mountpoint),
        String::from("-o"),
        String::from(options),
    ]));
}

fn umount(mountpoint: &str) {
    returncode_eval(exec("umount", vec![
        String::from(mountpoint),
    ]));
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