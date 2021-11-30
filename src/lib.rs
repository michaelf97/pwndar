use regex::Regex;
use std::time::Duration;
use ureq::{Agent, AgentBuilder, Error};

pub mod configuration;

pub fn verify_email(email: &str) -> bool {
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    email_regex.is_match(email)
}

pub fn check_hibp(email: &str, key: &str) -> Result<bool, ureq::Error> {
    let mut result = true;
    match hibp_request(
        &format!(
            "https://haveibeenpwned.com/api/v3/breachedaccount/{}",
            email
        ),
        &key,
    )
    .expect("Error getting breaches")
    .status()
    {
        200 => return Ok(false),
        404 => result = false,
        _ => panic!("Error reading breaches"),
    }

    match hibp_request(
        &format!("https://haveibeenpwned.com/api/v3/pasteaccount/{}", email),
        &key,
    )
    .expect("Error getting breaches")
    .status()
    {
        200 => return Ok(false),
        404 => result = false,
        _ => panic!("Error reading breaches"),
    }

    Ok(result)
}

fn hibp_request(url: &str, key: &str) -> ureq::Response {
    ureq::get(url).set("hibp-api-key", key).call()
}
