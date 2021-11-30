#[cfg(test)]
mod tests {
    use pwndar::verify_email;
    #[test]
    fn verify_email_works() {
        let email_addresses = [
            ("foo@bar.com", true),
            ("foo at bar.com", false),
            ("foo.bar42@c.com", true),
            ("42@c.com", true),
            ("f@42.co", true),
            ("foo@4-2.team", true),
            (".x@c.com", false),
            ("x.@c.com", false),
            ("foo_bar@bar.com", true),
            ("_bar@bar.com", true),
            ("foo_@bar.com", true),
            ("foo+bar@bar.com", true),
            ("+bar@bar.com", true),
            ("foo+@bar.com", true),
            ("foo.lastname@bar.com", true),
        ];

        for (email, result) in email_addresses {
            assert_eq!(verify_email(&email), result);
        }
    }
}
