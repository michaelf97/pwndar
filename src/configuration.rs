#[derive(serde::Deserialize, Debug)]
pub struct Keys {
    pub hibp: String,
}

pub fn get_keys() -> Result<Keys, config::ConfigError> {
    let mut keys = config::Config::default();
    keys.merge(config::File::with_name("keys"))?;
    keys.try_into()
}
