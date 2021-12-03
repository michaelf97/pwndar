use configuration::*;
use pwndar::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let email = args.get(1).expect("Please supply an email");

    if !verify_email(email) {
        panic!("INVALID EMAIL");
    }

    run(email);
}
