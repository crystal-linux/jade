use clap::{App, AppSettings, Arg, ArgMatches, ArgSettings, Shell, SubCommand}; 

fn main() {
    let app = App::new("jade")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("set")
                .about("Sets a value for installation")
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
            )
    )
        .get_matches();    
}
