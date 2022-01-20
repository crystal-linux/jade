pub fn choose_pkgs(desktop_setup: &str, de: &str, dm: &str) {
    if desktop_setup == "custom" {
        install_desktop(de, dm);
    } else {
        println!("Installing {}", desktop_setup);
        match desktop_setup {
            "onyx" => {
                install_desktop("onyx", "lightdm");
            }
            "gnome" => {
                install_desktop("gnome", "gdm");
            },
            "kde" => {
                install_desktop("kde", "sddm");
            },
            "budgie" => {
                install_desktop("budgie", "lightdm");
            },
            "cinnamon" => {
                install_desktop("cinnamon", "lightdm");
            },
            "mate" => {
                install_desktop("mate", "lightdm");
            },
            "xfce" => {
                install_desktop("xfce", "lightdm");
            },
            "pantheon" => {
                install_desktop("pantheon", "lightdm");
            },
            "enlightenment" => {
                install_desktop("enlightenment", "lightdm");
            },
            
            _ => {
                println!("Unknown desktop setup");
            }
        }
    }
}

fn install_desktop(de: &str, dm: &str) {
    println!("Installing {}", de);
    println!("Installing {}", dm);
}