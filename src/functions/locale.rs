use crate::internal::exec::*;
use crate::internal::*;

pub fn set_timezone(timezone: &str) {
    exec_eval(
        exec(
            "ln",
            vec![
                "-sf".to_string(),
                format!("/mnt/usr/share/zoneinfo/{}", timezone),
                "/mnt/etc/localtime".to_string(),
            ],
        ),
        "Set timezone",
    );
    exec_eval(
        exec_chroot("hwclock", vec!["--systohc".to_string()]),
        "Set system clock",
    );
}

pub fn set_locale(locale: String) {
    files_eval(
        files::append_file("/mnt/etc/locale.gen", "en_US.UTF-8 UTF-8\n"),
        "add en_US.UTF-8 UTF-8 to locale.gen",
    );
    for i in (0..locale.split(' ').count()).step_by(2) {
        files_eval(
            files::append_file(
                "/mnt/etc/locale.gen",
                format!(
                    "{} {}\n",
                    locale.split(' ').collect::<Vec<&str>>()[i],
                    locale.split(' ').collect::<Vec<&str>>()[i + 1]
                )
                .as_str(),
            ),
            "add locales to locale.gen",
        );
    }
    exec_eval(exec_chroot("locale-gen", vec![]), "generate locales");
    files::create_file("/mnt/etc/locale.conf");
    files_eval(
        files::append_file("/mnt/etc/locale.conf", "LANG=en_US.UTF-8"),
        "edit locale.conf",
    );
}

pub fn set_keyboard(keyboard: &str) {
    files::create_file("/mnt/etc/vconsole.conf");
    files_eval(
        files::append_file(
            "/mnt/etc/vconsole.conf",
            format!("KEYMAP={}", keyboard).as_str(),
        ),
        "set keyboard layout",
    );
}
