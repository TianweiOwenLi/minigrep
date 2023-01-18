use std::env::{self};
mod config;
use config::{Config};

mod grep_query;
use grep_query::{search};


fn print_help_msg() {
    println!("Usage: ./minigrep query filename")
}


// does not work with commandline pipe operators,
// possible soln is to somehow read cmdline standard input / output?
fn main() {
    let args : Vec<String> = env::args().collect();

    let ac: Config = Config::new(&args).unwrap_or_else(|e| {
        println!("Cannot parse cli args: {e}");
        print_help_msg();
        std::process::exit(1)
    });

    let s: String= ac.read_file().unwrap_or_else(|e| {
        println!("Cannot read from file: {e}");
        std::process::exit(1)
    });

    let res = search(ac.get_query(), s);

    for i in &res {
        println!("{i}");
    }

    std::process::exit(if res.is_empty() {1} else {0});

}
