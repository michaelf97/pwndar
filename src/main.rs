use configuration::get_keys;
use pwndar::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let email = args.get(1).expect("Please supply an email");
    let keys = get_keys().expect("Error reading configuration");
    match check_hibp(&email, &keys.hibp).expect("Error during API call") {
        true => println!("Compromised!"),
        false => println!("Clear!"),
    }
}
