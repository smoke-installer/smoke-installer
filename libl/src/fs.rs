use std::process::Command;

pub fn mv(sudo: &str, from: &str, to: &str) {
    Command::new("echo")
        .args(&[sudo, "|"])
        .args(&["mv", from, to])
        .spawn();
}

pub fn refresh() {
    Command::new("cd")
        .arg(".")
        .spawn();
}
