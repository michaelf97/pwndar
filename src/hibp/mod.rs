#[allow(non_snake_case)]
pub mod Hibp {

    use colored::Colorize;
    use std::error::Error as E;
    use std::fmt;
    use std::time::Duration;
    use ureq::OrAnyStatus;

    #[derive(Debug)]
    pub struct Builder {
        agent: ureq::Agent,
        key: String,
        breaches: Vec<Breach>,
        pastes: Vec<Paste>,
    }

    impl fmt::Display for Builder {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let breached = if self.breaches.len() > 0 {
                "COMPROMISED!".red().bold()
            } else {
                "CLEAR!".green().bold()
            };
            write!(f, "{} {}\n", "[BREACH RESULTS]: ".bold(), breached)?;
            self.breaches
                .iter()
                .enumerate()
                .for_each(|(counter, breach)| {
                    write!(f, "\t{}: {}\n", counter, &breach.Name);
                });
            write!(f, "\n")?;

            let pasted = if self.pastes.len() > 0 {
                "PASTED!".yellow().bold()
            } else {
                "CLEAR!".green().bold()
            };
            write!(f, "{} {}\n", "[PASTE RESULTS]: ".bold(), pasted)?;
            self.pastes.iter().enumerate().for_each(|(counter, paste)| {
                write!(f, "\t{}: {}\n", counter, &paste.Source);
            });
            write!(f, "\n")
        }
    }

    impl Builder {
        pub fn new(key: String) -> Builder {
            Builder {
                agent: Builder::build_agent(5),
                key,
                breaches: Vec::new(),
                pastes: Vec::new(),
            }
        }
        fn build_agent(timeout: u64) -> ureq::Agent {
            ureq::AgentBuilder::new()
                .timeout(Duration::from_secs(timeout))
                .build()
        }

        pub fn request(
            &mut self,
            method: &str,
            url: &str,
            truncate: bool,
        ) -> Result<ureq::Response, Error> {
            let response = match method {
                "get" => Ok(self.agent.get(url)),
                "post" => Ok(self.agent.post(url)),
                _ => Err(Error::new(format!("Method {} not supported", method))),
            }
            .unwrap()
            .query("truncateResponse", if truncate { "false" } else { "true" })
            .set("hibp-api-key", &self.key)
            .call()
            .or_any_status();

            response.map_or(Err(Error::new(format!("Error sending request"))), |resp| {
                if resp.status() == 200 || resp.status() == 404 {
                    Ok(resp)
                } else {
                    Err(Error::new(format!(
                        "Error recieved from server: {}",
                        resp.status_text()
                    )))
                }
            })
        }

        fn parse_breaches(&mut self, response: ureq::Response) -> Result<(), Error> {
            Ok(self.breaches = response.into_json().unwrap_or(Vec::new()))
        }

        fn parse_pastes(&mut self, response: ureq::Response) -> Result<(), Error> {
            Ok(self.pastes = response.into_json().unwrap_or(Vec::new()))
        }

        pub fn get_breaches(&mut self, email: &str, verbose: bool) -> Result<(), Error> {
            let breaches_url = format!(
                "https://haveibeenpwned.com/api/v3/breachedaccount/{}",
                email
            );

            let response = self.request("get", &breaches_url, verbose);
            self.parse_breaches(response.unwrap())
        }

        pub fn get_pastes(&mut self, email: &str, verbose: bool) -> Result<(), Error> {
            let pastes_url = format!("https://haveibeenpwned.com/api/v3/pasteaccount/{}", email);

            let response = self.request("get", &pastes_url, verbose);
            self.parse_pastes(response.unwrap())
        }
    }

    #[derive(serde::Deserialize, Debug)]
    #[allow(non_snake_case)]
    struct Paste {
        Source: String,
        Id: Option<String>,
        Title: Option<String>,
        Date: Option<String>,
        EmailCount: Option<usize>,
    }

    #[derive(serde::Deserialize, Debug)]
    #[allow(non_snake_case)]
    struct Breach {
        Name: String,
        Title: Option<String>,
        Domain: Option<String>,
        BreachedDate: Option<String>,
        AddedDate: Option<String>,
        ModifiedDate: Option<String>,
        PwnCount: Option<usize>,
        Description: Option<String>,
        IsVerified: Option<bool>,
        IsFabricated: Option<bool>,
        IsSensitive: Option<bool>,
        IsRetired: Option<bool>,
        IsSpamList: Option<bool>,
        LogoPath: Option<String>,
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

    impl E for Error {
        fn description(&self) -> &str {
            &self.details
        }
    }
}
