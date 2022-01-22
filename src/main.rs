mod functions;
mod internal;

use crate::functions::*;
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
                        .possible_values(&["auto", "manual"])
                        .required(true),
                )
                .arg(
                    Arg::with_name("device")
                        .help("The device to partition")
                        .required_if("mode", "auto"),
                )
                .arg(
                    Arg::with_name("efi")
                        .help("If the install destination should be partitioned with EFI")
                        .long("efi")
                        .takes_value(false),
                ),
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
        )
        .subcommand(
            SubCommand::with_name("desktops")
                .about("Graphical stuff (Desktop environment and Display Manager)")
                .arg(
                    Arg::with_name("desktopsetup")
                        .help("The desktop setup to use")
                        .required(true),
                ),
    ).get_matches();

    if let Some(app) = app.subcommand_matches("partition") {
        partition::partition(
            app.value_of("device").unwrap(),
            app.value_of("mode").unwrap(),
            app.is_present("efi"),
        );
    } else if let Some(app) = app.subcommand_matches("locale") {
        let kbrlayout = app.value_of("keyboard").unwrap();
        let timezn = app.value_of("timezone").unwrap();
        let locale = app.values_of("locales").unwrap();
        println!("keyboard layout: {}", kbrlayout);
        println!("timezone: {}", timezn);
        println!("locales: {:?}", locale);
    } else if let Some(app) = app.subcommand_matches("networking") {
        network::enable_ipv6(app.value_of("ipv6").unwrap().parse::<bool>().unwrap());
        network::set_hostname(app.value_of("hostname").unwrap())
    } else if let Some(app) = app.subcommand_matches("users") {
        if let Some(app) = app.subcommand_matches("newUser") {
            users::new_user(
                app.value_of("username").unwrap(),
                app.value_of("hasroot").unwrap().parse::<bool>().unwrap(),
                app.value_of("password").unwrap(),
            );
        } else if let Some(app) = app.subcommand_matches("rootPass") {
            let rootpass = app.value_of("rootPass").unwrap();
            println!("{}", rootpass);
            users::root_pass(app.value_of("rootPass").unwrap());
        }
    } else if let Some(app) = app.subcommand_matches("desktops") {
        desktops::choose_pkgs(app.value_of("desktopsetup").unwrap());
    } else {
        println!("Running TUI installer");
    }
}
