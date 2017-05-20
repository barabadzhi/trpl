extern crate greprs;

use std::{env, process};
use std::io::Write;

use greprs::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        writeln!(&mut std::io::stderr(), "Problem parsing arguments: {}", err)
            .expect("Could not write to stderr");
        process::exit(1);
    });

    if let Err(e) = greprs::run(config) {
        writeln!(&mut std::io::stderr(), "Application error: {}", e)
            .expect("Could not write to stderr");

        process::exit(1);
    }
}
