use libl::constants::DEVELOPERS;

pub fn help_main() {
    println!("________________________________________");
    println!("| Help menu                            |");
    println!("________________________________________");
    println!("Options:");
    println!("\tB - To go back");
    println!("\tE - To exit");
    println!("\tI - For installed list");
    println!("\tS - For store");
    let authors = DEVELOPERS;
    println!("Developer help:");
    println!("Contact:");
    println!("\t{}", authors);
    println!("Developer documentation:");
    println!("\thttps://smoke-installer.github.io/devleoper/documentation/index.html");
    print!("\n\n");
}
