use crate::internal::*;

pub fn exec_eval(return_code: std::result::Result<std::process::ExitStatus, std::io::Error>, logmsg: &str) {
    match &return_code {
        Ok(_) => {
            log(format!("{}: Success", logmsg));
        }
        Err(e) => {
            crash(format!("{}: Failed with error: {}", logmsg, e), return_code.unwrap_err().raw_os_error().unwrap());
        }
    }
}

pub fn files_eval(return_code: std::result::Result<(), std::io::Error>, logmsg: &str) {
    match &return_code {
        Ok(_) => {
            log(format!("[ \x1b[2;1;32mOK\x1b[0m ] {}", logmsg));
        }
        Err(e) => {
            crash(format!("[ \x1b[2;1;31mFAILED\x1b[0m ] {}  ERROR: {}", logmsg, e), return_code.unwrap_err().raw_os_error().unwrap());
        }
    }
}