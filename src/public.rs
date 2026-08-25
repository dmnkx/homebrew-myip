pub const ENDPOINTS: [&str; 3] = [
    "https://api.ipify.org",
    "https://ifconfig.me/ip",
    "https://icanhazip.com",
];

pub trait TextFetcher {
    fn get_text(&self, url: &str) -> Result<String, String>;
}

pub struct UreqFetcher;

impl TextFetcher for UreqFetcher {
    fn get_text(&self, url: &str) -> Result<String, String> {
        let response = ureq::get(url)
            .timeout(std::time::Duration::from_secs(5))
            .call()
            .map_err(|err| err.to_string())?;
        response.into_string().map_err(|err| err.to_string())
    }
}

pub fn public_ip(fetcher: &impl TextFetcher) -> Result<String, String> {
    public_ip_from(fetcher, &ENDPOINTS)
}

pub fn public_ip_from(fetcher: &impl TextFetcher, endpoints: &[&str]) -> Result<String, String> {
    let mut last_error = String::from("could not fetch public IP");

    for url in endpoints {
        match fetcher.get_text(url) {
            Ok(body) => match parse_ipv4_body(&body) {
                Some(ip) => return Ok(ip),
                None => last_error = format!("{url} returned an invalid IPv4 body"),
            },
            Err(err) => last_error = format!("{url}: {err}"),
        }
    }

    Err(last_error)
}

pub fn parse_ipv4_body(body: &str) -> Option<String> {
    let ip = body.trim();
    ip.parse::<std::net::Ipv4Addr>().ok().map(|_| ip.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct SequenceFetcher {
        responses: Vec<Result<String, String>>,
        calls: std::cell::Cell<usize>,
    }

    impl SequenceFetcher {
        fn new(responses: Vec<Result<String, String>>) -> Self {
            Self {
                responses,
                calls: std::cell::Cell::new(0),
            }
        }
    }

    impl TextFetcher for SequenceFetcher {
        fn get_text(&self, _url: &str) -> Result<String, String> {
            let index = self.calls.get();
            self.calls.set(index + 1);
            self.responses
                .get(index)
                .cloned()
                .unwrap_or_else(|| Err("no more responses".into()))
        }
    }

    #[test]
    fn parse_ipv4_trims_whitespace() {
        assert_eq!(parse_ipv4_body("  1.2.3.4\n"), Some("1.2.3.4".into()));
    }

    #[test]
    fn parse_ipv4_rejects_empty_and_html() {
        assert_eq!(parse_ipv4_body(""), None);
        assert_eq!(parse_ipv4_body("<html>ok</html>"), None);
        assert_eq!(parse_ipv4_body("::1"), None);
    }

    #[test]
    fn uses_first_valid_endpoint() {
        let fetcher = SequenceFetcher::new(vec![Ok("9.9.9.9".into())]);
        assert_eq!(
            public_ip_from(&fetcher, &["https://a.example", "https://b.example"]).unwrap(),
            "9.9.9.9"
        );
        assert_eq!(fetcher.calls.get(), 1);
    }

    #[test]
    fn skips_invalid_then_succeeds() {
        let fetcher = SequenceFetcher::new(vec![Ok("not-an-ip".into()), Ok("4.4.4.4".into())]);
        assert_eq!(
            public_ip_from(&fetcher, &["https://a.example", "https://b.example"]).unwrap(),
            "4.4.4.4"
        );
    }

    #[test]
    fn skips_error_then_succeeds() {
        let fetcher = SequenceFetcher::new(vec![Err("timeout".into()), Ok("5.5.5.5".into())]);
        assert_eq!(
            public_ip_from(&fetcher, &["https://a.example", "https://b.example"]).unwrap(),
            "5.5.5.5"
        );
    }

    #[test]
    fn all_endpoints_fail() {
        let fetcher = SequenceFetcher::new(vec![Err("down".into()), Ok("nope".into())]);
        let err = public_ip_from(&fetcher, &["https://a.example", "https://b.example"]).unwrap_err();
        assert!(err.contains("invalid IPv4"));
    }
}
