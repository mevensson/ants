use ants;
use std::io;
use std::process;

fn main() {
    let stdio = io::stdin();
    let input = stdio.lock();
    let settings = ants::Settings::parse(input).unwrap_or_else(|err| {
        eprintln!("Error: {}!", err);
        process::exit(1);
    });
    ants::run(settings).unwrap();
}
