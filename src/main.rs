use clap::{Arg, Command};

fn main() {
    let matches = Command::new("RustyCookie")
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

    // Use `get_one` instead of `value_of`
    let template = matches.get_one::<String>("template").unwrap();
    let destination = matches.get_one::<String>("destination").unwrap();

    println!("Initializing project from template: {} at destination: {}", template, destination);

    // Call the initialization logic here (placeholder for now)
}
