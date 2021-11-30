use crate::configuration;
use ureq::{Agent, AgentBuilder};

pub struct HibpRequest {
    key: String,
    agent: Agent,
    breaches: Vec<HibpBreach>,
    pastes: Vec<HibpPaste>,
}

struct HibpBreach {}

struct HibpPaste {}

impl HibpRequest {
    pub fn new() {
        let keys = configuration::get_keys().unwrap();
        println!("{:?}", keys);
    }

    fn check_breaches(&self) {}

    fn check_pastes(&self) {}
}
