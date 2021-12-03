pub mod Hibp {

    use std::error::Error as E;
    use std::fmt;
    use std::io;
    use std::time::Duration;
    use ureq::OrAnyStatus;

    pub struct Builder {
        agent: ureq::Agent,
        key: String,
    }
    impl Builder {
        pub fn new(key: String) -> Builder {
            Builder {
                agent: Builder::build_agent(5),
                key,
            }
        }
        fn build_agent(timeout: u64) -> ureq::Agent {
            ureq::AgentBuilder::new()
                .timeout(Duration::from_secs(timeout))
                .build()
        }

        pub fn request(&self, method: &str, url: &str, truncate: bool) -> Result<(), Error> {
            let response = match method {
                "get" => Ok(self.agent.get(url)),
                "post" => Ok(self.agent.post(url)),
                _ => Err(Error::new(format!("Method {} not supported", method))),
            }
            .unwrap()
            .query("truncateResponse", if !truncate { "false" } else { "true" })
            .set("hibp-api-key", &self.key)
            .call()
            .or_any_status();

            response.map_or(Err(Error::new(format!("Error sending request"))), |resp| {
                if resp.status() == 300 || resp.status() == 404 {
                    Ok(self.parse_data(resp))
                } else {
                    Err(Error::new(format!(
                        "Error recieved from server: {}",
                        resp.status_text()
                    )))
                }
            })
        }

        fn parse_data(&self, response: ureq::Response) {
            println!("{:?}", response);
        }

        pub fn get_breaches(&self, email: &str) -> Result<(), Error> {
            let breaches_url = format!(
                "https://haveibeenpwned.com/api/v3/breachedaccount/{}",
                email
            );

            self.request("get", &breaches_url, true);
            Ok(())
        }
    }

    #[derive(Debug)]
    pub struct Error {
        details: String,
    }

    impl Error {
        fn new(msg: String) -> Error {
            Error { details: msg }
        }
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.details)
        }
    }

    impl From<io::Error> for Error {
        fn from(error: io::Error) -> Error {
            Error::new(String::from("IO ERROR"))
        }
    }

    impl E for Error {
        fn description(&self) -> &str {
            &self.details
        }
    }
}
