use crate::internal::exec::*;
use crate::internal::*;

pub fn choose_pkgs(desktop_setup: &str) {
    log(format!("Installing {}", desktop_setup));
    match desktop_setup {
        "onyx" => {
            install(vec![
                "xorg",
                "onyx",
                "lightdm",
                "lightdm-gtk-greeter",
                "lightdm-gtk-greeter-settings",
            ]);
            files_eval(
                files::append_file(
                    "/mnt/etc/lightdm/lightdm.conf",
                    "[SeatDefaults]\ngreeter-session=lightdm-gtk-greeter\n",
                ),
                "Add lightdm greeter",
            );
            enable_dm("lightdm");
        }
        "gnome" => {
            install(vec![
                "xorg",
                "gnome",
                "gnome-tweaks",
                "chrome-gnome-shell",
                "gdm",
            ]);
            enable_dm("gdm");
        }
        "kde" => {
            install(vec![
                "xorg",
                "plasma",
                "plasma-wayland-session",
                "kde-applications",
                "sddm",
            ]);
            enable_dm("sddm");
        }
        "budgie" => {
            install(vec![
                "xorg",
                "budgie-desktop",
                "gnome",
                "lightdm",
                "lightdm-gtk-greeter",
                "lightdm-gtk-greeter-settings",
            ]);
            files_eval(
                files::append_file(
                    "/mnt/etc/lightdm/lightdm.conf",
                    "[SeatDefaults]\ngreeter-session=lightdm-gtk-greeter\n",
                ),
                "Add lightdm greeter",
            );
            enable_dm("lightdm");
        }
        "cinnamon" => {
            install(vec![
                "xorg",
                "cinnamon",
                "lightdm",
                "lightdm-gtk-greeter",
                "lightdm-gtk-greeter-settings",
                "metacity",
                "gnome-shell",
            ]);
            files_eval(
                files::append_file(
                    "/mnt/etc/lightdm/lightdm.conf",
                    "[SeatDefaults]\ngreeter-session=lightdm-gtk-greeter\n",
                ),
                "Add lightdm greeter",
            );
            enable_dm("lightdm");
        }
        "mate" => {
            install(vec![
                "xorg",
                "mate",
                "lightdm",
                "lightdm-gtk-greeter",
                "lightdm-gtk-greeter-settings",
                "mate-extra",
            ]);
            files_eval(
                files::append_file(
                    "/mnt/etc/lightdm/lightdm.conf",
                    "[SeatDefaults]\ngreeter-session=lightdm-gtk-greeter\n",
                ),
                "Add lightdm greeter",
            );
            enable_dm("lightdm");
        }
        "xfce" => {
            install(vec![
                "xorg",
                "xfce4",
                "lightdm",
                "lightdm-gtk-greeter",
                "lightdm-gtk-greeter-settings",
                "xfce4-goodies",
            ]);
            files_eval(
                files::append_file(
                    "/mnt/etc/lightdm/lightdm.conf",
                    "[SeatDefaults]\ngreeter-session=lightdm-gtk-greeter\n",
                ),
                "Add lightdm greeter",
            );
            enable_dm("lightdm");
        }
        "enlightenment" => {
            install(vec![
                "xorg",
                "enlightenment",
                "lightdm",
                "lightdm-gtk-greeter",
                "lightdm-gtk-greeter-settings",
                "terminology",
            ]);
            files_eval(
                files::append_file(
                    "/mnt/etc/lightdm/lightdm.conf",
                    "[SeatDefaults]\ngreeter-session=lightdm-gtk-greeter\n",
                ),
                "Add lightdm greeter",
            );
            enable_dm("lightdm");
        }

        _ => {
            crash("Unknown desktop setup".to_string(), 1);
        }
    }
    install(vec!["networkmanager"]);
    exec_eval(
        exec_chroot(
            "systemctl",
            vec![String::from("enable"), String::from("NetworkManager")],
        ),
        "Enable network manager",
    );
}

fn enable_dm(dm: &str) {
    log(format!("Enabling {}", dm));
    exec_eval(
        exec_chroot("systemctl", vec![String::from("enable"), String::from(dm)]),
        format!("Enable {}", dm).as_str(),
    );
}
