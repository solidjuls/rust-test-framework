use std::env;
use testframework::Config;
use std::process;

// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// fn bool_maker() -> bool { true }

// fn expect<T>(param: &T) -> fn() -> bool {
//     println!("calculating slowly...");
//     bool_maker
// }

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap();

    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = testframework::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

