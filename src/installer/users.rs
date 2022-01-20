pub fn new_user(username: &str, hasroot: bool, password: &str) {
    println!("Creating new user '{}'", username);
    if hasroot {
        println!("User '{}' will have root privileges", username);
    } else {
        println!("User '{}' will not have root privileges", username);
    }
    println!("Setting password for user '{}' to '{}'", username, password);
}

pub fn root_pass(root_pass: &str) {
    println!("Setting root password to '{}'", root_pass);
}