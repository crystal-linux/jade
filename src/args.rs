use clap::{ArgEnum, Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(name="jade", version=env!("CARGO_PKG_VERSION"), about=env!("CARGO_PKG_DESCRIPTION"), author=env!("CARGO_PKG_AUTHORS"))]
pub struct Opt {
    #[clap(subcommand)]
    pub command: Command,

    #[clap(long, short, parse(from_occurrences))]
    pub verbose: usize,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Partition the install destination
    #[clap(name = "partition")]
    Partition(PartitionArgs),

    /// Install base packages
    #[clap(name = "install-base")]
    InstallBase,

    /// Generate fstab file for mounting partitions
    #[clap(name = "genfstab")]
    GenFstab,

    /// Setup timeshift
    #[clap(name = "setup-timeshift")]
    SetupTimeshift,

    /// Install the bootloader
    #[clap(name = "bootloader")]
    Bootloader {
        #[clap(subcommand)]
        subcommand: BootloaderSubcommand,
    },

    /// Set locale stuff
    #[clap(name = "locale")]
    Locale(LocaleArgs),

    /// Set networking stuff
    #[clap(name = "networking")]
    Networking(NetworkingArgs),

    /// Configure users and passwords
    #[clap(name = "users")]
    Users {
        #[clap(subcommand)]
        subcommand: UsersSubcommand,
    },

    /// Install the nix package manager
    #[clap(name = "nix")]
    Nix,

    /// Read jade installation config
    #[clap(name = "config")]
    Config {
        /// The config to read
        config: PathBuf,
    },

    /// Install a graphical desktop setup
    #[clap(name = "desktops")]
    Desktops {
        /// The desktop setup to use
        #[clap(arg_enum)]
        desktop: DesktopSetup,
    },
}

#[derive(Debug, Args)]
pub struct PartitionArgs {
    /// If jade should automatically partition (mode = auto)
    /// or the user manually partitioned it (mode = manual)
    #[clap(arg_enum)]
    pub mode: PartitionMode,

    /// The device to partition
    #[clap(required_if_eq("mode", "PartitionMode::Auto"))]
    pub device: PathBuf,

    /// If the install destination should be partitioned with EFI
    #[clap(long)]
    pub efi: bool,

    /// The partitions to use for manual partitioning
    #[clap(required_if_eq("mode", "Partition::Manual"), parse(try_from_str = parse_partitions))]
    pub partitions: Vec<Partition>,
}

#[derive(Debug)]
pub struct Partition {
    pub mountpoint: String,
    pub blockdevice: String,
    pub filesystem: String,
}

impl Partition {
    pub fn new(mountpoint: String, blockdevice: String, filesystem: String) -> Self {
        Self {
            mountpoint,
            blockdevice,
            filesystem,
        }
    }
}

pub fn parse_partitions(s: &str) -> Result<Partition, &'static str> {
    println!("{}", s);
    Ok(Partition::new(
        s.split(':').collect::<Vec<&str>>()[0].to_string(),
        s.split(':').collect::<Vec<&str>>()[1].to_string(),
        s.split(':').collect::<Vec<&str>>()[2].to_string(),
    ))
}


#[derive(Debug, ArgEnum, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum PartitionMode {
    #[clap(name = "auto")]
    Auto,
    #[clap(name = "manual")]
    Manual,
}

#[derive(Debug, Subcommand)]
pub enum BootloaderSubcommand {
    /// Install grub in efi mode
    #[clap(name = "grub-efi")]
    GrubEfi {
        /// The directory to install the EFI bootloader to
        efidir: PathBuf,
    },

    /// Install grub in legacy (BIOS) mode
    #[clap(name = "grub-legacy")]
    GrubLegacy {
        /// The device to install the bootloader to
        device: PathBuf,
    },
}

#[derive(Debug, Args)]
pub struct LocaleArgs {
    /// The keyboard layout to use
    pub keyboard: String,

    /// The timezone to use
    pub timezone: String,

    /// The locales to set
    pub locales: Vec<String>,
}

#[derive(Debug, Args)]
pub struct NetworkingArgs {
    /// The hostname to assign to the system
    pub hostname: String,

    /// Whether ipv6 should be enabled
    #[clap(long)]
    pub ipv6: bool,
}

#[derive(Debug, Subcommand)]
pub enum UsersSubcommand {
    /// Create a new user
    #[clap(name="new-user", aliases=&["newUser"])]
    NewUser(NewUserArgs),

    /// Set the password of the root user
    #[clap(name="root-password", aliases=&["root-pass", "rootPass"])]
    RootPass {
        /// The password to set. NOTE: Takes hashed password, use `openssl passwd -1 <password>` to generate the hash.
        password: String,
    },
}

#[derive(Debug, Args)]
pub struct NewUserArgs {
    /// The name of the user to create
    pub username: String,

    /// If the user should have root privileges
    #[clap(long, aliases=&["has-root", "sudoer", "root"])]
    pub hasroot: bool,

    /// The password to set. NOTE: Takes hashed password, use `openssl passwd -1 <password>` to generate the hash.
    /// When not providing a password openssl jumps into an interactive masked input mode allowing you to hide your password
    /// from the terminal history.
    pub password: String,
}

#[derive(Debug, ArgEnum, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum DesktopSetup {
    #[clap(name = "onyx")]
    Onyx,

    #[clap(name = "gnome")]
    Gnome,

    #[clap(name = "kde", aliases = ["plasma"])]
    Kde,

    #[clap(name = "budgie")]
    Budgie,

    #[clap(name = "cinnamon")]
    Cinnamon,

    #[clap(name = "mate")]
    Mate,

    #[clap(name = "xfce")]
    Xfce,

    #[clap(name = "enlightenment")]
    Enlightenment,

    #[clap(name = "None/DIY")]
    None,
}
