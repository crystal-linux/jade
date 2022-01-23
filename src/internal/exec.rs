use std::process::Command;

pub fn exec(command: &str, args: Vec<String>) -> Result<std::process::ExitStatus, std::io::Error> {
    let returncode = Command::new(command).args(args).status();
    returncode
}

pub fn exec_chroot(
    command: &str,
    args: Vec<String>
) -> Result<std::process::ExitStatus, std::io::Error> {
    let returncode = Command::new("arch-chroot")
        .args(&["/mnt", command])
        .args(args)
        .status();
    returncode
}

pub fn exec_workdir(
    command: &str,
    workdir: &str,
    args: Vec<String>,
) -> Result<std::process::ExitStatus, std::io::Error> {
    let returncode = Command::new(command)
        .args(args)
        .current_dir(workdir)
        .status();
    returncode
}
