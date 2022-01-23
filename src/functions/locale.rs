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
    ), "Set timezone");
    exec_eval(exec_chroot("hwclock", vec!["--systohc".to_string()]), "Set system clock");
}

pub fn set_locale(locale: String) {
    files_eval(files::append_file("/etc/locale.gen", "en_US.UTF-8 UTF-8"), "add en_US.UTF-8 UTF-8 to locale.gen");
    files_eval(files::append_file("/etc/locale.gen", locale.as_str()), "add locales to locale.gen");
    exec_eval(exec_chroot("locale-gen", vec!["".to_string()]), "generate locales");
    files::create_file("/etc/locale.conf");
    files_eval(files::append_file("/etc/locale.conf", "LANG=en_US.UTF-8"), "edit locale.conf");
}

pub fn set_keyboard(keyboard: &str) {
    files::create_file("/etc/vconsole.conf");
    files_eval(files::append_file(
        "/etc/vconsole.conf",
        format!("KEYMAP={}", keyboard).as_str(),
    ), "set keyboard layout");
}
