use std::process::exit;

pub fn crash<S: AsRef<str>>(a: S, b: i32) -> ! {
    log::error!("{}", a.as_ref());
    exit(b);
}
