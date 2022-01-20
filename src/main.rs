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
                        .help("The root partition to use (only read if mode is manual)")
                        .required_if("mode", "manual"),
                )
                .arg(
                    Arg::with_name("boot")
                        .help("The boot partition to use (only read if mode is manual)")
                        .required_if("mode", "manual"),
                )
                .arg(
                    Arg::with_name("swap")
                        .help("The swap partition to use (only read if mode is manual)")
                        .required_if("mode", "manual"),
                )
                .arg(
                    Arg::with_name("device")
                        .help("The device to partition (only read if mode is automatic)")
                        .required_if("mode", "auto"),
                ),
        )
        .subcommand(
            SubCommand::with_name("timezone")
                .about("Set the timezone")
                .arg(
                    Arg::with_name("timezone")
                        .help("The timezone to set")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("locales")
                .about("Set the locales")
                .arg(
                    Arg::with_name("locales")
                        .help("The locales to set")
                        .multiple(true)
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("hostname")
                .about("Set the hostname")
                .arg(
                    Arg::with_name("hostname")
                        .help("The hostname to set")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("ipv6")
                .about("Activate IPv6")
                .arg(
                    Arg::with_name("ipv6")
                        .help("If ipv6 should be activated")
                        .required(true),
                ),   
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
        .subcommand(
            SubCommand::with_name("newUser")
                .about("Create a new user")
                .arg(
                    Arg::with_name("username")
                        .help("The username to create")
                        .required(true),
                )
                .arg(
                    Arg::with_name("password")
                        .help("The password to set")
                        .required(true),
                    )
        )
        .subcommand(
            SubCommand::with_name("graphical")
                .about("Graphical stuff (Desktop environment and Display Manager)")
                .arg(
                    Arg::with_name("de")
                        .help("The Desktop envionment to install")
                        .required(true),
                )
                .arg(
                    Arg::with_name("dm")
                        .help("The Display Manager to install")
                        .required(true),
                    ),
                )
        .subcommand(
            SubCommand::with_name("flatpak")
                .about("Flatpak")
                .arg(
                    Arg::with_name("flatpak")
                    .help("If flatpak should be installed")
                    .required(true),
                ),
    ).get_matches();    


    if let Some(app) = app.subcommand_matches("partition") {
        let mode = app.value_of("mode").unwrap();
        let root = app.value_of("root").unwrap_or("none");
        let boot = app.value_of("boot").unwrap_or("none");
        let swap = app.value_of("swap").unwrap_or("none");
        let device = app.value_of("device").unwrap_or("none");
        println!("mode: {}", mode);
        println!("root: {}", root);
        println!("boot: {}", boot);
        println!("swap: {}", swap);
        println!("device: {}", device);
    } else if let Some(app) = app.subcommand_matches("timezone") {
        let timezone = app.value_of("timezone").unwrap();
        println!("{}", timezone);
    } else if let Some(app) = app.subcommand_matches("locales") {
        let locales = app.values_of("locales").unwrap();
        println!("{:?}", locales);
    } else if let Some(app) = app.subcommand_matches("hostname") {
        let hostname = app.value_of("hostname").unwrap();
        println!("{}", hostname);
    } else if let Some(app) = app.subcommand_matches("ipv6") {
        let ipv6 = app.value_of("ipv6").unwrap();
        println!("{}", ipv6);
    } else if let Some(app) = app.subcommand_matches("rootPass") {
        let root_pass = app.value_of("rootPass").unwrap();
        println!("{}", root_pass);
    } else if let Some(app) = app.subcommand_matches("newUser") {
        let username = app.value_of("username").unwrap();
        let password = app.value_of("password").unwrap();
        println!("{}", username);
        println!("{}", password);
    } else if let Some(app) = app.subcommand_matches("graphical") {
        let de = app.value_of("de").unwrap();
        let dm = app.value_of("dm").unwrap();
        println!("{}", de);
        println!("{}", dm);
    } else if let Some(app) = app.subcommand_matches("flatpak") {
        let flatpak = app.value_of("flatpak").unwrap();
        println!("{}", flatpak);
    } else {
        println!("Running TUI installer");
    }
}
