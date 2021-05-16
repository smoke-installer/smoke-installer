use std::process::Command;

pub fn unzip(file: &str, password: &str) {
    Command::new("unzip")
        .arg("-P")
        .arg(password)
        .arg(file)
        .arg("2>")
        .arg("/dev/null")
        .spawn();
}
