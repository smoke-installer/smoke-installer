use std::process::Command;
use tools::CURL;

pub fn curl(url: String, output: String) {
    Command::new("sh")
        .arg(CURL)
        .arg(url)
        .arg(output)
        .arg("2>")
        .arg("/dev/null")
        .spawn();
}
