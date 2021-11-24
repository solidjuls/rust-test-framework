// use std::env;
// use testframework::Config;
// use std::process;
use macro_demo::test;

#[test(a)]
fn do_something() {
    let a = 9;
    a = 6;
    a = 0;
}

fn main() {
    // do_something()
    // let args: Vec<String> = env::args().collect();

    // let config = Config::new(&args).unwrap();

    // // let config = Config::new(&args).unwrap_or_else(|err| {
    // //     println!("Problem parsing arguments: {}", err);
    // //     process::exit(1);
    // // });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    // if let Err(e) = testframework::run(config) {
    //     println!("Application error: {}", e);
    //     process::exit(1);
    // }
}
