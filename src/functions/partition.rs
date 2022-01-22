use crate::internal::exec::*;
use crate::internal::*;

pub fn partition(device: &str, mode: &str, efi: bool) {
    if mode == "manual" {
        log("Manual partitioning".to_string());
    } else {
        log(format!("automatically partitioning {}", device));
        if efi {
            let return_code = exec(
                "parted",
                vec![
                    String::from("-s"),
                    String::from(device),
                    String::from("mklabel"),
                    String::from("gpt"),
                ],
            );
            match return_code {
                Ok(_) => {
                    log("Created GPT label".to_string());
                }
                Err(e) => {
                    crash(format!("Failed to create GPT label, Error: {}", e), 1);
                }
            }
            let return_code = exec(
                "parted",
                vec![
                    String::from("-s"),
                    String::from(device),
                    String::from("mkpart"),
                    String::from("fat32"),
                    String::from("0"),
                    String::from("300"),
                ],
            );
            match return_code {
                Ok(_) => {
                    log("Created fat32 EFI partition".to_string());
                }
                Err(e) => {
                    crash(
                        format!("Failed to create fat32 EFI partition, Error: {}", e),
                        1,
                    );
                }
            }
            let return_code = exec(
                "parted",
                vec![
                    String::from("-s"),
                    String::from(device),
                    String::from("mkpart"),
                    String::from("btrfs"),
                    String::from("300"),
                    String::from("100%"),
                ],
            );
            match return_code {
                Ok(_) => {
                    log("Created btrfs root partition".to_string());
                }
                Err(e) => {
                    crash(
                        format!("Failed to create btrfs root partition, Error: {}", e),
                        1,
                    );
                }
            }
        } else {
            let return_code = exec(
                "parted",
                vec![
                    String::from("-s"),
                    String::from(device),
                    String::from("mklabel"),
                    String::from("msdos"),
                ],
            );
            match return_code {
                Ok(_) => {
                    log("Created MSDOS label".to_string());
                }
                Err(e) => {
                    crash(format!("Failed to create MSDOS label, Error: {}", e), 1);
                }
            }
            let return_code = exec(
                "parted",
                vec![
                    String::from("-s"),
                    String::from(device),
                    String::from("mkpart"),
                    String::from("btrfs"),
                    String::from("512MIB"),
                    String::from("100&"),
                ],
            );
            match return_code {
                Ok(_) => {
                    log("Created btrfs root partition".to_string());
                }
                Err(e) => {
                    crash(
                        format!("Failed to create btrfs root partition, Error: {}", e),
                        1,
                    );
                }
            }
            let return_code = exec(
                "parted",
                vec![
                    String::from("-s"),
                    String::from(device),
                    String::from("mkpart"),
                    String::from("ext4"),
                    String::from("1MIB"),
                    String::from("512MIB"),
                ],
            );
            match return_code {
                Ok(_) => {
                    log("Created ext4 boot partition".to_string());
                }
                Err(e) => {
                    crash(
                        format!("Failed to create ext4 boot partition, Error: {}", e),
                        1,
                    );
                }
            }
        }
    }
}
