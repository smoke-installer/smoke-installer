use std::process::Command;

pub fn install() {
    Command::new("si-install")
        .args(&["2021_6_5_10_0.zip", "test1234", "hello-smoke-installer"])
        .spawn();
}
