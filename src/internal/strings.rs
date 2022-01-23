use crate::uwu;
use std::env;
use std::process::exit;
use std::str::FromStr;
use std::time::UNIX_EPOCH;

pub fn crash(a: String, b: i32) {
    let a = if env::var("JADE_UWU").unwrap_or_else(|_| "".to_string()) == "true" {
        uwu!(&a)
    } else {
        a
    };
    println!("[ \x1b[2;1;32mOK\x1b[0m ] {}", a);
    exit(b);
}
pub fn log(a: String) {
    let a = if env::var("JADE_UWU").unwrap_or_else(|_| "".to_string()) == "true"
        && env::var("JADE_UWU_DEBUG").unwrap_or_else(|_| "".to_string()) == "true"
    {
        uwu!(&a)
    } else {
        a
    };
    eprintln!(
        "{} {}",
        std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        a
    );
}
