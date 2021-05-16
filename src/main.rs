extern crate libl;
use libl::io::input;

use std::fs::{copy, rename, remove_file, File};
use std::path::Path;
use std::process::{exit};
use libl::zip::unzip;
use libl::fs::mv;
use std::env::set_current_dir;edition = "2018"

use std::io::Write;

mod help;
mod ids;
mod install;
mod installed;
mod passwords;
mod setup;
mod store;

fn main() {
    setup::setup();
    loop {
        print!("\n>>> ");
        let mut main_input = input();
        if main_input == "E" {
            exit(0);
        } else if main_input == "H" {
            help::help_main();
        } else if main_input == "I" {
            installed::installed_main();
        } else if main_input == "S" {
            store::store_main();
        } else {
            println!("{} - Not an option", main_input);
            println!("Type 'H' for help");
        }
    }
}
