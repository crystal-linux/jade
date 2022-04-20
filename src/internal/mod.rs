pub mod config;
pub mod exec;
pub mod files;
pub mod install;
pub mod returncode_eval;
pub mod strings;

pub use install::install;
pub use returncode_eval::*;
pub use strings::crash;

#[macro_export]
macro_rules! uwu {
    ($x:expr) => {{
        let uwu: String = $x.to_string();
        uwu.replace("l", "w")
            .replace("L", "W")
            .replace("r", "w")
            .replace("R", "W")
            .replace("na", "nya")
            .replace("Na", "Nya")
            .replace("NA", "NYA")
    }};
}
