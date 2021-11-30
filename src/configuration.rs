use std::fmt;

#[derive(serde::Deserialize, Debug)]
pub struct Keys {
    pub hibp: String,
}

impl Keys {
    fn get(&self, key: &str) -> Result<&str, ()> {
        match key {
            "hibp" => Ok(&self.hibp),
            _ => Err(()),
        }
    }
}

pub fn get_keys() -> Result<Keys, config::ConfigError> {
    let mut keys = config::Config::default();
    keys.merge(config::File::with_name("keys.yaml"))?;
    keys.try_into()
}
