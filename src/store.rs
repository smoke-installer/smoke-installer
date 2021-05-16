use libl::io::input;
use super::install::install_game;
use std::fs::{read_to_string, File};
use std::io::{BufReader, BufRead};

fn display_store_list() {
    let mut store_reader = BufReader::new(File::open("/etc/ludum/store.txt").expect("Couldn't read installed.txt"));
    let mut line_num = 0;
    let mut names = 0;
    for line in store_reader.lines() {
        if names == 3 {
            print!("\n");
            names = 0;
        }
        print!("{:?}    ", line.unwrap().as_str());
        line_num = line_num + 1;
        names = names + 1;
    }
    print!("\n");
}

fn store_loop() {
    loop {
        print!(">>> ");
        let store_input = input();
        let store_list = read_to_string("/etc/ludum/store.txt").unwrap();
        if store_input == "B" {
            break;
        }
        /* --- */
        if store_list.contains(&store_input) {
            install_game(store_input.as_str());
        } else {
            println!("{} - Game not found", store_input);
        }
    }
}

pub fn store_main() {
    println!("________________________________________");
    println!("| Available games and apps             |");
    println!("________________________________________");
    display_store_list();
    store_loop();
}
