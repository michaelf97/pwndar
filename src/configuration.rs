use std::error::Error;
use std::fmt;

#[derive(serde::Deserialize, Debug)]
pub struct Keys {
    pub hibp: Option<String>,
}

impl Keys {
    pub fn get(&self, key: &str) -> Result<&String, KeysError> {
        match key {
            "hibp" => self
                .hibp
                .as_ref()
                .ok_or(KeysError::new(&format!("Error getting key: {}", key))),
            _ => Err(KeysError::new(&format!("Error missing key: {}", key))),
        }
    }
}

#[derive(Debug)]
pub struct KeysError {
    details: String,
}

impl KeysError {
    fn new(msg: &str) -> KeysError {
        KeysError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for KeysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for KeysError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub fn get_keys() -> Result<Keys, config::ConfigError> {
    let mut keys = config::Config::default();
    keys.merge(config::File::with_name("keys.yaml"))?;
    keys.try_into()
}
