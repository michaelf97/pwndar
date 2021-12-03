extern crate clap;
use clap::{App, Arg};
use pwndar::*;

fn main() {
    let banner = include_str!("banner.txt");

    let matches = App::new("Pwndar")
        .version("0.1")
        .author("Michael Forret <michael.forret@quorumcyber.com>")
        .arg(
            Arg::with_name("EMAIL")
                .help("Sets the email to parse")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Display verbose output"),
        )
        .before_help(banner)
        .get_matches();

    let email = matches.value_of("EMAIL").expect("Please supply an email");
    let verbose = matches.is_present("verbose");
    run(email, verbose);
}
