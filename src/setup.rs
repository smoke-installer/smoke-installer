use libl::constants::VERSION;

pub fn setup() {
    print!("\x1B[2J");
    let version = VERSION;
    println!("Starting Smoke Installer v{}...", version);
    println!("\n\n");
    println!("________________________________________");
    println!("| I) Installed | S) Store | H) Help    |");
    println!("_______________________________________|");
}
