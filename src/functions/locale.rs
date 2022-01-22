use crate::internal::exec::*;
use crate::internal::*;

pub fn set_timezone(timezone: &str) {
    exec_eval(exec(
        "ln",
        vec![
            "-sf".to_string(),
            format!("/usr/share/zoneinfo/{}", timezone),
            "/etc/localtime".to_string(),
        ],
    ));
    exec_eval(exec_chroot("hwclock", vec!["--systohc".to_string()]));
}

pub fn set_locale(locale: String) {
    files_eval(files::append_file("/etc/locale.gen", "en_US.UTF-8 UTF-8"));
    files_eval(files::append_file("/etc/locale.gen", locale.as_str()));
    exec_eval(exec_chroot("locale-gen", vec!["".to_string()]));
    files::create_file("/etc/locale.conf");
    files_eval(files::append_file("/etc/locale.conf", "LANG=en_US.UTF-8"));
}

pub fn set_keyboard(keyboard: &str) {
    files::create_file("/etc/vconsole.conf");
    files_eval(files::append_file(
        "/etc/vconsole.conf",
        format!("KEYMAP={}", keyboard).as_str(),
    ));
}

fn files_eval(return_code: std::result::Result<(), std::io::Error>) {
    match return_code {
        Ok(_) => {
            log("Success".to_string());
        }
        Err(e) => {
            crash(format!("Failed to create file, Error: {}", e), 1);
        }
    }
}

fn exec_eval(return_code: std::result::Result<std::process::Output, std::io::Error>) {
    match return_code {
        Ok(_) => {
            log("Success".to_string());
        }
        Err(e) => {
            crash(format!("Failed with error: {}", e), 1);
        }
    }
}
