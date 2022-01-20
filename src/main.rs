use clap::{App, Arg, SubCommand}; 

fn main() {
    let app = App::new("jade")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("partition")
                .about("Partition the install destination")
                .arg(
                    Arg::with_name("mode")
                        .help("If jade should automatically partition (mode = auto) or the user manually partitioned it (mode = manual)")
                        .required(true),
                )
                .arg(
                    Arg::with_name("root")
                        .help("The root partition(mode = manual) or device to partition(mode = manual)")
                        .required(true),
                )
                .arg(
                    Arg::with_name("boot")
                        .help("The boot partition to use (only read if mode is manual)")
                )
                .arg(
                    Arg::with_name("swap")
                        .help("The swap partition to use (only read if mode is manual)")
                )
        )
        .subcommand(
            SubCommand::with_name("locale")
                .about("Set locale stuff")
                .arg(
                    Arg::with_name("keyboard")
                        .help("The keyboard layout to use")
                        .required(true),
                )
                .arg(
                    Arg::with_name("timezone")
                        .help("The timezone to use")
                        .required(true),
                )
                .arg(
                    Arg::with_name("locales")
                        .help("The locales to set")
                        .multiple(true)
                        .index(3)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("networking")
                .about("Set networking stuff")
                .arg(
                    Arg::with_name("hostname")
                        .help("The hostname to use")
                        .required(true),
                )
                .arg(
                    Arg::with_name("wifi")
                        .help("If wifi is used (will launch nmtui if set to true)")
                        .required(true),
                )
                .arg(
                    Arg::with_name("ipv6")
                        .help("Wether ipv6 should be enabled")
                        .required(true),
                )
        )
        .subcommand(
            SubCommand::with_name("users")
                .about("Configure users")
                .subcommand(
                    SubCommand::with_name("newUser")
                        .about("Create a new user")
                        .arg(
                            Arg::with_name("username")
                                .help("The username to create")
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("hasroot")
                                .help("If the user should have root privileges")
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("password")
                                .help("The password to set")
                                .required(true),
                        )
                .subcommand(
                    SubCommand::with_name("rootPass")
                        .about("Set the root password")
                        .arg(
                            Arg::with_name("rootPass")
                                .help("The root password to set")
                                .required(true),
                        ),
                )
            ),
        )
        .subcommand(
            SubCommand::with_name("desktops")
                .about("Graphical stuff (Desktop environment and Display Manager)")
                .arg(
                    Arg::with_name("desktopsetup")
                        .help("The desktop setup to use")
                        .required(true),
                )
                .arg(
                    Arg::with_name("de")
                        .help("The Desktop envionment to install (only read if desktopsetup is set to custom)")
                        .required_if("desktopsetup", "custom"),
                )
                .arg(
                    Arg::with_name("dm")
                        .help("The Display Manager to install (only read if desktopsetup is set to custom)") 
                        .required_if("desktopsetup", "custom"),
                    ),
    ).get_matches();    


    if let Some(app) = app.subcommand_matches("partition") {
        let mode = app.value_of("mode").unwrap();
        let root = app.value_of("root").unwrap_or("none");
        let boot = app.value_of("boot").unwrap_or(root);
        let swap = app.value_of("swap").unwrap_or("none");
        let device = if app.value_of("mode").unwrap() == "auto" {
            root
        } else {
            "none"
        };
        println!("mode: {}", mode);
        println!("root: {}", root);
        println!("boot: {}", boot);
        println!("swap: {}", swap);
        println!("device: {}", device);
    } else if let Some(app) = app.subcommand_matches("locale") {
        let kbrlayout = app.value_of("keyboard").unwrap();
        let timezn = app.value_of("timezone").unwrap();
        let locale = app.values_of("locales").unwrap();
        println!("keyboard layout: {}", kbrlayout);
        println!("timezone: {}", timezn);
        println!("locales: {:?}", locale);
    } else if let Some(app) = app.subcommand_matches("networking") {
        let hostname = app.value_of("hostname").unwrap();
        let wifi = app.value_of("wifi").unwrap();
        let ipv6 = app.value_of("ipv6").unwrap();
        println!("hostname: {}", hostname);
        println!("wifi: {}", wifi);
        println!("ipv6: {}", ipv6);
    } else if let Some(app) = app.subcommand_matches("users") {
        if let Some(app) = app.subcommand_matches("newUser") {
            let username = app.value_of("username").unwrap();
            let hasroot = app.value_of("hasroot").unwrap();
            let password = app.value_of("password").unwrap();
            println!("username: {}", username);
            println!("hasroot: {}", hasroot);
            println!("password: {}", password);
        } else if let Some(app) = app.subcommand_matches("rootPass") {
            let rootpass = app.value_of("rootPass").unwrap();
            println!("{}", rootpass);
        }
    } else if let Some(app) = app.subcommand_matches("desktops") {
        let desktopsetup = app.value_of("desktopsetup").unwrap();
        let de = app.value_of("de").unwrap_or("none");
        let dm = app.value_of("dm").unwrap_or("none");
        println!("desktopsetup: {}", desktopsetup);
        println!("de: {}", de);
        println!("dm: {}", dm);
    } else {
        println!("Running TUI installer");
    }
}
