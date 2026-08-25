use crate::local::LocalAddress;
use crate::public::{public_ip, TextFetcher};
use std::io::Write;

pub fn print_lookup<L, F, Out, ErrW>(
    stdout: &mut Out,
    stderr: &mut ErrW,
    local: &L,
    fetcher: &F,
) -> i32
where
    L: LocalAddress,
    F: TextFetcher,
    Out: Write,
    ErrW: Write,
{
    let mut failed = false;

    match local.local_ipv4() {
        Ok(ip) => {
            let _ = writeln!(stdout, "local: {ip}");
        }
        Err(err) => {
            let _ = writeln!(stderr, "myip: local: {err}");
            failed = true;
        }
    }

    match public_ip(fetcher) {
        Ok(ip) => {
            let _ = writeln!(stdout, "public: {ip}");
        }
        Err(err) => {
            let _ = writeln!(stderr, "myip: public: {err}");
            failed = true;
        }
    }

    if failed {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::local::LocalAddress;
    use crate::public::TextFetcher;

    struct OkLocal;
    impl LocalAddress for OkLocal {
        fn local_ipv4(&self) -> Result<String, String> {
            Ok("10.0.0.2".into())
        }
    }

    struct FailPublic;
    impl TextFetcher for FailPublic {
        fn get_text(&self, url: &str) -> Result<String, String> {
            Err(format!("{url} down"))
        }
    }

    #[test]
    fn public_failure_still_prints_local() {
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let code = print_lookup(&mut stdout, &mut stderr, &OkLocal, &FailPublic);
        assert_eq!(code, 1);
        assert_eq!(String::from_utf8(stdout).unwrap(), "local: 10.0.0.2\n");
        assert!(String::from_utf8(stderr).unwrap().contains("myip: public:"));
    }
}
