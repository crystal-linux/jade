use crate::args;
use crate::args::PartitionMode;
use crate::internal::exec::*;
use crate::internal::*;
use std::path::{Path, PathBuf};

/*mkfs.bfs mkfs.cramfs mkfs.ext3  mkfs.fat mkfs.msdos  mkfs.xfs
mkfs.btrfs mkfs.ext2  mkfs.ext4  mkfs.minix mkfs.vfat mkfs.f2fs */

pub fn fmt_mount(mountpoint: &str, filesystem: &str, blockdevice: &str) {
    match filesystem {
        "vfat" => exec_eval(
            exec(
                "mkfs.vfat",
                vec![
                    String::from("-F"),
                    String::from("32"),
                    String::from(blockdevice),
                ],
            ),
            format!("Formatting {blockdevice} as vfat").as_str(),
        ),
        "bfs" => exec_eval(
            exec("mkfs.bfs", vec![String::from(blockdevice)]),
            format!("Formatting {blockdevice} as bfs").as_str(),
        ),
        "cramfs" => exec_eval(
            exec("mkfs.cramfs", vec![String::from(blockdevice)]),
            format!("Formatting {blockdevice} as cramfs").as_str(),
        ),
        "ext3" => exec_eval(
            exec("mkfs.ext3", vec![String::from(blockdevice)]),
            format!("Formatting {blockdevice} as ext3").as_str(),
        ),
        "fat" => exec_eval(
            exec("mkfs.fat", vec![String::from(blockdevice)]),
            format!("Formatting {blockdevice} as fat").as_str(),
        ),
        "msdos" => exec_eval(
            exec("mkfs.msdos", vec![String::from(blockdevice)]),
            format!("Formatting {blockdevice} as msdos").as_str(),
        ),
        "xfs" => exec_eval(
            exec("mkfs.xfs", vec![String::from(blockdevice)]),
            format!("Formatting {blockdevice} as xfs").as_str(),
        ),
        "btrfs" => exec_eval(
            exec(
                "mkfs.btrfs",
                vec![String::from("-f"), String::from(blockdevice)],
            ),
            format!("Formatting {blockdevice} as btrfs").as_str(),
        ),
        "ext2" => exec_eval(
            exec("mkfs.ext2", vec![String::from(blockdevice)]),
            format!("Formatting {blockdevice} as ext2").as_str(),
        ),
        "ext4" => exec_eval(
            exec("mkfs.ext4", vec![String::from(blockdevice)]),
            format!("Formatting {blockdevice} as ext4").as_str(),
        ),
        "minix" => exec_eval(
            exec("mkfs.minix", vec![String::from(blockdevice)]),
            format!("Formatting {blockdevice} as minix").as_str(),
        ),
        "f2fs" => exec_eval(
            exec("mkfs.f2fs", vec![String::from(blockdevice)]),
            format!("Formatting {blockdevice} as f2fs").as_str(),
        ),
        "don't format" => {
            log::debug!("Not formatting {}", blockdevice);
        }
        "noformat" => {
            log::debug!("Not formatting {}", blockdevice);
        }
        _ => {
            crash(
                format!("Unknown filesystem {filesystem}, used in partition {blockdevice}"),
                1,
            );
        }
    }
    exec_eval(
        exec("mkdir", vec![String::from("-p"), String::from(mountpoint)]),
        format!("Creating mountpoint {mountpoint} for {blockdevice}").as_str(),
    );
    mount(blockdevice, mountpoint, "");
}

pub fn partition(
    device: PathBuf,
    mode: PartitionMode,
    efi: bool,
    partitions: &mut Vec<args::Partition>,
    unakite: bool,
) {
    println!("{:?}", mode);
    match mode {
        PartitionMode::Auto => {
            if !device.exists() {
                crash(format!("The device {device:?} doesn't exist"), 1);
            }
            log::debug!("automatically partitioning {device:?}");
            if efi {
                partition_with_efi(&device, unakite);
            } else {
                partition_no_efi(&device, unakite);
            }
            if device.to_string_lossy().contains("nvme") {
                part_nvme(&device, efi, unakite);
            } else {
                part_disk(&device, efi, unakite);
            }
        }
        PartitionMode::Manual => {
            log::debug!("Manual partitioning");
            partitions.sort_by(|a, b| a.mountpoint.len().cmp(&b.mountpoint.len()));
            for i in 0..partitions.len() {
                println!("{:?}", partitions);
                println!("{}", partitions.len());
                println!("{}", &partitions[i].mountpoint);
                println!("{}", &partitions[i].filesystem);
                println!("{}", &partitions[i].blockdevice);
                fmt_mount(
                    &partitions[i].mountpoint,
                    &partitions[i].filesystem,
                    &partitions[i].blockdevice,
                );
            }
        }
    }
}

fn partition_no_efi(device: &Path, unakite: bool) {
    let device = device.to_string_lossy().to_string();
    exec_eval(
        exec(
            "parted",
            vec![
                String::from("-s"),
                String::from(&device),
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
                String::from(&device),
                String::from("mkpart"),
                String::from("primary"),
                String::from("ext4"),
                String::from("1MIB"),
                String::from("512MIB"),
            ],
        ),
        "create bios boot partition",
    );
    if unakite {
        exec_eval(
            exec(
                "parted",
                vec![
                    String::from("-s"),
                    String::from(&device),
                    String::from("mkpart"),
                    String::from("primary"),
                    String::from("btrfs"),
                    String::from("512MIB"),
                    String::from("10048MIB"),
                ],
            ),
            "create btrfs Unakite root partition",
        );
        exec_eval(
            exec(
                "parted",
                vec![
                    String::from("-s"),
                    device,
                    String::from("mkpart"),
                    String::from("primary"),
                    String::from("btrfs"),
                    String::from("10048MIB"),
                    String::from("100%"),
                ],
            ),
            "create btrfs Crystal root partition",
        );
    } else {
        exec_eval(
            exec(
                "parted",
                vec![
                    String::from("-s"),
                    device,
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

fn partition_with_efi(device: &Path, unakite: bool) {
    let device = device.to_string_lossy().to_string();
    exec_eval(
        exec(
            "parted",
            vec![
                String::from("-s"),
                String::from(&device),
                String::from("mklabel"),
                String::from("gpt"),
            ],
        ),
        format!("create gpt label on {}", &device).as_str(),
    );
    exec_eval(
        exec(
            "parted",
            vec![
                String::from("-s"),
                String::from(&device),
                String::from("mkpart"),
                String::from("fat32"),
                String::from("0"),
                String::from("300"),
            ],
        ),
        "create EFI partition",
    );
    if unakite {
        exec_eval(
            exec(
                "parted",
                vec![
                    String::from("-s"),
                    String::from(&device),
                    String::from("mkpart"),
                    String::from("primary"),
                    String::from("btrfs"),
                    String::from("512MIB"),
                    String::from("10048MIB"),
                ],
            ),
            "create btrfs Unakite root partition",
        );
        exec_eval(
            exec(
                "parted",
                vec![
                    String::from("-s"),
                    device,
                    String::from("mkpart"),
                    String::from("primary"),
                    String::from("btrfs"),
                    String::from("10048MIB"),
                    String::from("100%"),
                ],
            ),
            "create btrfs Crystal root partition",
        );
    } else {
        exec_eval(
            exec(
                "parted",
                vec![
                    String::from("-s"),
                    device,
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

fn part_nvme(device: &Path, efi: bool, unakite: bool) {
    let device = device.to_string_lossy().to_string();
    if efi && !unakite {
        exec_eval(
            exec("mkfs.vfat", vec![format!("-F32, {}p1", device)]),
            format!("format {}p1 as fat32", device).as_str(),
        );
        exec_eval(
            exec(
                "mkfs.btrfs",
                vec!["-f".to_string(), format!("{}p2", device)],
            ),
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
    } else if !efi && !unakite {
        exec_eval(
            exec("mkfs.ext4", vec![format!("{}p1", device)]),
            format!("format {}p1 as ext4", device).as_str(),
        );
        exec_eval(
            exec(
                "mkfs.btrfs",
                vec!["-f".to_string(), format!("{}p2", device)],
            ),
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
    } else if efi && unakite {
        exec_eval(
            exec("mkfs.vfat", vec![format!("-F32 {}p1", device)]),
            format!("format {}p1 as fat32", device).as_str(),
        );
        exec_eval(
            exec(
                "mkfs.btrfs",
                vec!["-f".to_string(), format!("{}p2", device)],
            ),
            format!("format {}p2 as btrfs", device).as_str(),
        );
        exec_eval(
            exec(
                "mkfs.btrfs",
                vec!["-f".to_string(), format!("{}p3", device)],
            ),
            format!("format {}p3 as btrfs", device).as_str(),
        );
        mount(format!("{}p3", device).as_str(), "/mnt", "");
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
        mount(format!("{}p3", device).as_str(), "/mnt/", "subvol=@");
        files_eval(files::create_directory("/mnt/boot"), "create /mnt/boot");
        files_eval(
            files::create_directory("/mnt/boot/efi"),
            "create /mnt/boot/efi",
        );
        files_eval(files::create_directory("/mnt/home"), "create /mnt/home");
        mount(
            format!("{}p3", device).as_str(),
            "/mnt/home",
            "subvol=@home",
        );
        mount(format!("{}p1", device).as_str(), "/mnt/boot/efi", "");
    } else if !efi && unakite {
        exec_eval(
            exec("mkfs.ext4", vec![format!("{}p1", device)]),
            format!("format {}p1 as ext4", device).as_str(),
        );
        exec_eval(
            exec(
                "mkfs.btrfs",
                vec!["-f".to_string(), format!("{}p2", device)],
            ),
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

fn part_disk(device: &Path, efi: bool, unakite: bool) {
    let device = device.to_string_lossy().to_string();
    if efi && !unakite {
        exec_eval(
            exec("mkfs.vfat", vec![format!("-F32 {}1", device)]),
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
    } else if !efi && !unakite {
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
    } else if efi && unakite {
        exec_eval(
            exec("mkfs.vfat", vec![format!("-F32 {}1", device)]),
            format!("format {}1 as fat32", device).as_str(),
        );
        exec_eval(
            exec("mkfs.btrfs", vec!["-f".to_string(), format!("{}2", device)]),
            format!("format {}2 as btrfs", device).as_str(),
        );
        exec_eval(
            exec("mkfs.btrfs", vec!["-f".to_string(), format!("{}3", device)]),
            format!("format {}3 as btrfs", device).as_str(),
        );
        mount(format!("{}3", device).as_str(), "/mnt", "");
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
        mount(format!("{}3", device).as_str(), "/mnt/", "subvol=@");
        files_eval(files::create_directory("/mnt/boot"), "create /mnt/boot");
        files_eval(
            files::create_directory("/mnt/boot/efi"),
            "create /mnt/boot/efi",
        );
        files_eval(files::create_directory("/mnt/home"), "create /mnt/home");
        mount(format!("{}3", device).as_str(), "/mnt/home", "subvol=@home");
        mount(format!("{}1", device).as_str(), "/mnt/boot/efi", "");
    } else if !efi && unakite {
        exec_eval(
            exec("mkfs.ext4", vec![format!("{}1", device)]),
            format!("format {}1 as ext4", device).as_str(),
        );
        exec_eval(
            exec("mkfs.btrfs", vec!["-f".to_string(), format!("{}2", device)]),
            format!("format {}2 as btrfs", device).as_str(),
        );
        exec_eval(
            exec("mkfs.btrfs", vec!["-f".to_string(), format!("{}3", device)]),
            format!("format {}3 as btrfs", device).as_str(),
        );
        mount(format!("{}3", device).as_str(), "/mnt/", "");
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
        mount(format!("{}3", device).as_str(), "/mnt/", "subvol=@");
        files_eval(
            files::create_directory("/mnt/boot"),
            "create directory /mnt/boot",
        );
        files_eval(
            files::create_directory("/mnt/home"),
            "create directory /mnt/home",
        );
        mount(format!("{}3", device).as_str(), "/mnt/home", "subvol=@home");
        mount(format!("{}1", device).as_str(), "/mnt/boot", "");
    }
}

pub fn mount(partition: &str, mountpoint: &str, options: &str) {
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

pub fn umount(mountpoint: &str) {
    exec_eval(
        exec("umount", vec![String::from(mountpoint)]),
        format!("unmount {}", mountpoint).as_str(),
    );
}
