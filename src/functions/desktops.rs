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
        DesktopSetup::Lxqt => install_lxqt(),
        DesktopSetup::Sway => install_sway(),
        DesktopSetup::I3gaps => install_i3gaps(),
        DesktopSetup::Herbstluftwm => install_herbstluftwm(),
        DesktopSetup::Awesome => install_awesome(),
        DesktopSetup::Bspwm => install_bspwm(),
        DesktopSetup::None => log::debug!("No desktop setup selected"),
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

fn install_bspwm() {
    install(vec![
        "xorg",
        "bspwm",
        "sxhkd",
        "xdo",
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

fn install_awesome() {
    install(vec![
        "xorg",
        "awesome",
        "dex",
        "rlwrap",
        "vicious",
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

fn install_herbstluftwm() {
    install(vec![
        "xorg",
        "herbstluftwm",
        "dmenu",
        "dzen2",
        "xorg-xsetroot",
        "xterm",
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

fn install_i3gaps() {
    install(vec![
        "xorg",
        "i3-gaps",
        "dmenu",
        "i3lock",
        "i3status",
        "rxvt-unicode",
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

fn install_sway() {
    install(vec![
        "xorg-xwayland",
        "sway",
        "bemenu",
        "foot",
        "mako",
        "polkit",
        "swaybg",
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

fn install_lxqt() {
    install(vec![
        "xorg",
        "lxqt",
        "breeze-icons",
        "nm-tray",
        "xscreensaver",
        "sddm",
    ]);
    enable_dm("sddm");
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
        "pipewire",
        "pipewire-pulse",
        "pipewire-jack",
        "pipewire-alsa",
        "pavucontrol",
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
        "gnome-shell",
        "nautilus",
        "gnome-terminal",
        "gnome-control-center",
        "xdg-user-dirs",
        "gdm",
        "xorg",
        "gnome-tweaks",
        "gnome-backgrounds",
        "sushi",
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
        "sushi",
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
