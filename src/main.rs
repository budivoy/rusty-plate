use clap::{Arg, Command};

// Import RustyPlate from the new module
mod rusty_plate;
use crate::rusty_plate::RustyPlate;

fn main() {
    let matches = Command::new("RustyPlate")
        .version("1.0")
        .about("Initialize projects from cookiecutter templates")
        .arg(
            Arg::new("template")
                .short('t')
                .long("template")
                .value_name("TEMPLATE_URL")
                .help("Specifies the cookiecutter template URL or local path")
                .required(true),
        )
        .arg(
            Arg::new("destination")
                .short('d')
                .long("destination")
                .value_name("DESTINATION_PATH")
                .help("Specifies the directory where the project will be created")
                .required(true),
        )
        .get_matches();

    // Fetch the values of template and destination
    let template = matches.get_one::<String>("template").unwrap();
    let destination = matches.get_one::<String>("destination").unwrap();

    println!("Initializing project from template: {} at destination: {}", template, destination);

    // Create an instance of RustyPlate
    let rusty_plate = RustyPlate {
        template,
        destination,
    };

    // Call the method on the RustyPlate instance
    if let Err(e) = rusty_plate.initialize_project() {
        eprintln!("Error initializing project: {}", e);
    }
}
