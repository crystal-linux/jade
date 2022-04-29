use crate::args::DesktopSetup;
use crate::internal::exec::*;
use crate::internal::*;

pub fn install_desktop_setup(desktop_setup: DesktopSetup) {
    log::debug!("Installing {:?}", desktop_setup);
    match desktop_setup {
        DesktopSetup::Onyx => install_onyx(),
        DesktopSetup::Gnome => install_gnome(),
        DesktopSetup::Kde => install_kde(),
        DesktopSetup::Budgie => install_budgie(),
        DesktopSetup::Cinnamon => install_cinnamon(),
        DesktopSetup::Mate => install_mate(),
        DesktopSetup::Xfce => install_xfce(),
        DesktopSetup::Enlightenment => install_enlightenment(),
        DesktopSetup::None => log::debug!("No desktop setup selected"),
        _ => crash("Unsupported desktop setup", 1),
    }
    install_networkmanager();
}

fn install_networkmanager() {
    install(vec!["networkmanager"]);
    exec_eval(
        exec_chroot(
            "systemctl",
            vec![String::from("enable"), String::from("NetworkManager")],
        ),
        "Enable network manager",
    );
}

fn install_enlightenment() {
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

fn install_xfce() {
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

fn install_mate() {
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

fn install_cinnamon() {
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

fn install_budgie() {
    install(vec![
        "xorg",
        "budgie-desktop",
        "gnome",
        "lightdm",
        "lightdm-gtk-greeter",
        "lightdm-gtk-greeter-settings",
        "xdg-desktop-portal",
        "xdg-desktop-portal-gtk",
        "xdg-utils",
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

fn install_kde() {
    install(vec![
        "xorg",
        "plasma",
        "plasma-wayland-session",
        "kde-utilities",
        "kde-system",
        "sddm",
    ]);
    enable_dm("sddm");
}

fn install_gnome() {
    install(vec![
        "xorg",
        "gnome",
        "gnome-tweaks",
        "chrome-gnome-shell",
        "gdm",
    ]);
    enable_dm("gdm");
}

fn install_onyx() {
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

fn enable_dm(dm: &str) {
    log::debug!("Enabling {}", dm);
    exec_eval(
        exec_chroot("systemctl", vec![String::from("enable"), String::from(dm)]),
        format!("Enable {}", dm).as_str(),
    );
}
