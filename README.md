<p align="center">
  <a href="https://git.tar.black/crystal/ame/">
    <img src="https://git.tar.black/crystal/branding/-/raw/main/logos/crystal-logo-minimal.png" alt="Logo" width="150" height="150">
  </a>
</p>
<h2 align="center">Jade</h2>
<p align="center">
    <a href="https://discord.gg/yp4xpZeAgW"><img alt="Discord" src="https://img.shields.io/discord/825473796227858482?color=blue&label=Discord&logo=Discord&logoColor=white"?link=https://discord.gg/yp4xpZeAgW&link=https://discord.gg/yp4xpZeAgW></a>
</p>

<p align="center">Jade is the backend and TUI installer for crystal linux.</p>

## Use the TUI (not implemented yet)
just run <br>
`jade`

## Backend usage

### autopartition the drive
```sh
# autopartition /dev/sda with efi enabled
jade partition auto /dev/sda --efi

# autopartition /dev/nvmen0 with efi disabled
jade partition auto /dev/nvmen0
```

### install base packages
```sh
jade install-base
```

### install bootloader
```sh
# install as efi with esp being /boot/efi
jade bootloader grub-efi /boot/efi

# install as legacy on /dev/sda
jade bootloader grub-legacy /dev/sda
```

### generate fstab
```sh
jade genfstab
```

### configuring locale settings
```sh
# set the keyboard layout to colemak, the timezone to Europe/Berlin and set en_US.UTF-8 as the locale
jade locale colemak Europe/Berlin en_US.UTF-8 UTF-8
```

### configue network settings
```sh
# set the hostname to getcryst.al with ipv6 disabled
jade networking getcryst.al 

# set the hostname to getcryst.al with ipv6 enabled
jade networking getcryst.al --ipv6
```

### configure users
```sh
# make a new user called nonRootHaver, without sudo and easytohack as the password
jade users newUser nonRootaver easytohack

# make a user called rootHaver, with sudo and omgsosuperhardtohack as the password
jade users newUser rootHaver omgsosuperhardtohack --sudoer
```

### set root password
```sh
# set the root password to 'muchSecurity,veryHardToHack'
jade users rootPass muchSecurity,veryHardToHack
```

### install a desktop environment
```sh
# install onyx
jade desktops onyx

# install gnome
jade desktops gnome
```

### setup timeshift
```sh
jade setup-timeshift
```

### debug logging

debug messages:
```sh
jade -v
```

traces:
```sh
jade -vv
```

## How to build:

Tested on latest Cargo (1.60.0-nightly)

<br>

#### Debug/development builds

- `cargo build`

#### Optimised/release builds

- `cargo build --release`


echo "JADE_UWU=true" >> ~/.zshrc <br>
echo "JADE_UWU=true" >> ~/.bashrc <br>
set -Ux JADE_UWU true <br>
<br>
if you want to have your log and crash output be "cute"
