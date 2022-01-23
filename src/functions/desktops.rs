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
        }
        "gnome" => {
            install(vec![
                "xorg",
                "gnome",
                "gnome-tweaks",
                "chrome-gnome-shell",
                "gdm",
            ]);
        }
        "kde" => {
            install(vec![
                "xorg",
                "kde",
                "plasma",
                "plasma-wayland-session",
                "kde-applications",
                "sddm",
            ]);
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
        }

        _ => {
            crash("Unknown desktop setup".to_string(), 1);
        }
    }
}
