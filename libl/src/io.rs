use std::io::{stdin, stdout, Write};

pub fn input() -> String {
    let mut uinput=String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut uinput).expect("Did not enter a string");
    if let Some('\n') = uinput.chars().next_back() {
        uinput.pop();
    }
    if let Some('\r') = uinput.chars().next_back() {
        uinput.pop();
    }
    return uinput;
}
