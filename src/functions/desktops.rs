use crate::internal::*;

pub fn choose_pkgs(desktop_setup: &str) {
    println!("Installing {}", desktop_setup);
    match desktop_setup {
        "onyx" => {
            install(vec![
                "onyx",
                "lightdm",
                "lightdm-gtk-greeter",
                "lightdm-gtk-greeter-settings",
            ]);
        }
        "gnome" => {
            install(vec!["gnome", "gnome-tweaks", "chrome-gnome-shell", "gdm"]);
        }
        "kde" => {
            install(vec![
                "kde",
                "plasma",
                "plasma-wayland-session",
                "kde-applications",
                "sddm",
            ]);
        }
        "budgie" => {
            install(vec![
                "budgie-desktop",
                "gnome",
                "lightdm",
                "lightdm-gtk-greeter",
                "lightdm-gtk-greeter-settings",
            ]);
        }
        "cinnamon" => {
            install(vec![
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
                "mate",
                "lightdm",
                "lightdm-gtk-greeter",
                "lightdm-gtk-greeter-settings",
                "mate-extra",
            ]);
        }
        "xfce" => {
            install(vec![
                "xfce4",
                "lightdm",
                "lightdm-gtk-greeter",
                "lightdm-gtk-greeter-settings",
                "xfce4-goodies",
            ]);
        }
        "enlightenment" => {
            install(vec![
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
