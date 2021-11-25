use std::env;
use file_search::Config;
use std::process;

fn main() {
    // do_something()
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap();

    // // let config = Config::new(&args).unwrap_or_else(|err| {
    // //     println!("Problem parsing arguments: {}", err);
    // //     process::exit(1);
    // // });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = file_search::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
