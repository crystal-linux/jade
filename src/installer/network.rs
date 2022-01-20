pub fn set_hostname(hostname: &str) {
    println!("Setting hostname to {}", hostname);
}

pub fn enable_ipv6(ipv6: bool) {
    if ipv6 {
        println!("enabling ipv6");
    } else {
        println!("disabling ipv6");
    }
}