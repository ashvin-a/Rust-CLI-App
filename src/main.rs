use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("An error occured: {:?}", err);
        process::exit(1); // Terminates the program with the status code passed in.
    });
    //? unwrap_or_else method is used to write what the program should do when it
    //? encounters an error.

    println!("Searching for {:?}", config.query);
    println!("In file {:?}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1)
    }
}
