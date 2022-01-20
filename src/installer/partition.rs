pub fn partition(root: &str, boot: &str, swap: &str, mode: &str, device: &str) {
    if mode == "manual" {
        println!("Using {} as root partition", root);
        println!("Using {} as boot partition", boot);
        println!("Using {} as swap partition", swap);
    } else {
        println!("automatically partitioning {}", device);
    }
}