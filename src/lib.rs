use hibp::Hibp;
use regex::Regex;

pub mod configuration;
pub mod hibp;

pub fn verify_email(email: &str) -> bool {
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    email_regex.is_match(email)
}

pub fn run(email: &str, verbose: bool) {
    let keys = configuration::get_keys().unwrap();
    let hibp_key = keys.get("hibp").unwrap();
    let mut builder = Hibp::Builder::new(hibp_key.to_owned());
    builder
        .get_breaches(email, verbose)
        .expect("Error getting breaches");
    builder
        .get_pastes(email, verbose)
        .expect("Error getting pastes");
    println!("{}", builder);
}
