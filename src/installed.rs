use libl::io::input;
use std::fs::{File, read_to_string};
use std::io::{BufReader, BufRead, Read};
use std::path::Path;
use std::process::Command;
use std::thread::spawn;

fn display_installed_list() {
    let mut installed_reader = BufReader::new(File::open("/etc/ludum/installed.txt").expect("Couldn't read installed.txt"));
    let mut line_num = 0;
    let mut names = 0;
    for line in installed_reader.lines() {
        if names == 3 {
            names = 0;
            print!("\n");
        }
        print!("{:?}    ", line.unwrap().as_str());
        line_num = line_num + 1;
        names = names + 1;
    }
}

fn installed_loop() {
    loop {
        print!("\n>>> ");
        let mut installed_input = input();
        if installed_input == "B" {
            break;
        }
        let mut installed_file = read_to_string("/etc/ludum/installed.txt").unwrap();
        if installed_file.contains(&installed_input) {
            let mut game_command = installed_input.replace(" ", "_");
            let mut game_path = "/bin/";
            game_path.to_string().push_str(&*game_command);
            if Path::new(game_path.to_string().as_str()).exists() {
                spawn(|| {
                    Command::new(game_command)
                        .spawn();
                });
            } else {
                println!("Game executable can't be found");
            }
        } else {
            println!("{} - Game not found", installed_input);
        }
    }
}

pub fn installed_main() {
    println!("______________________________");
    println!("| Installed games and apps   |");
    println!("______________________________");
    display_installed_list();
    installed_loop();
}
