use hibp::HibpRequest;
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

pub fn run(email: &str) {
    HibpRequest::new();
}
