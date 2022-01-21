pub fn partition(device: &str, mode: &str) {
    if mode == "manual" {
        println!("Manual partitioning");
    } else {
        println!("automatically partitioning {}", device);
    }
}
