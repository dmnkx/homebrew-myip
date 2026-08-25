mod cli;
mod local;
mod output;
mod public;

pub use cli::{parse_args, Command, ParseError};
pub use local::{DefaultRoute, LocalAddress};
pub use public::{TextFetcher, UreqFetcher, ENDPOINTS};

use std::io::Write;

pub fn run_default<A, S, L, F, Out, ErrW>(
    args: A,
    stdout: &mut Out,
    stderr: &mut ErrW,
    local: &L,
    fetcher: &F,
) -> i32
where
    A: IntoIterator<Item = S>,
    S: AsRef<str>,
    L: LocalAddress,
    F: TextFetcher,
    Out: Write,
    ErrW: Write,
{
    run(
        args,
        stdout,
        stderr,
        local,
        fetcher,
        env!("CARGO_PKG_VERSION"),
    )
}

pub fn run<A, S, L, F, Out, ErrW>(
    args: A,
    stdout: &mut Out,
    stderr: &mut ErrW,
    local: &L,
    fetcher: &F,
    version: &str,
) -> i32
where
    A: IntoIterator<Item = S>,
    S: AsRef<str>,
    L: LocalAddress,
    F: TextFetcher,
    Out: Write,
    ErrW: Write,
{
    match parse_args(args) {
        Ok(Command::Help) => {
            let _ = write!(stdout, "{}", cli::help_text(version));
            0
        }
        Ok(Command::Version) => {
            let _ = writeln!(stdout, "{}", cli::version_text(version));
            0
        }
        Ok(Command::PrintIps) => output::print_lookup(stdout, stderr, local, fetcher),
        Err(ParseError::Unexpected(arg)) => {
            let _ = writeln!(stderr, "myip: unexpected argument: {arg}");
            let _ = write!(stdout, "{}", cli::help_text(version));
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::local::LocalAddress;
    use crate::public::TextFetcher;

    struct OkLocal(&'static str);
    impl LocalAddress for OkLocal {
        fn local_ipv4(&self) -> Result<String, String> {
            Ok(self.0.to_string())
        }
    }

    struct FailLocal;
    impl LocalAddress for FailLocal {
        fn local_ipv4(&self) -> Result<String, String> {
            Err("no route".into())
        }
    }

    struct MapFetcher(&'static [(&'static str, Result<&'static str, &'static str>)]);
    impl TextFetcher for MapFetcher {
        fn get_text(&self, url: &str) -> Result<String, String> {
            for (endpoint, result) in self.0 {
                if *endpoint == url {
                    return result.map(str::to_string).map_err(str::to_string);
                }
            }
            Err("missing endpoint".into())
        }
    }

    fn run_capture<L: LocalAddress, F: TextFetcher>(
        args: &[&str],
        local: &L,
        fetcher: &F,
    ) -> (i32, String, String) {
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let code = run(args.iter().copied(), &mut stdout, &mut stderr, local, fetcher, "0.1.0");
        (
            code,
            String::from_utf8(stdout).unwrap(),
            String::from_utf8(stderr).unwrap(),
        )
    }

    #[test]
    fn version_flag_exits_zero() {
        let fetcher = MapFetcher(&[]);
        let (code, stdout, stderr) = run_capture(&["myip", "--version"], &OkLocal("10.0.0.1"), &fetcher);
        assert_eq!(code, 0);
        assert_eq!(stdout, "myip 0.1.0\n");
        assert!(stderr.is_empty());
    }

    #[test]
    fn help_flag_exits_zero() {
        let fetcher = MapFetcher(&[]);
        let (code, stdout, stderr) = run_capture(&["myip", "--help"], &OkLocal("10.0.0.1"), &fetcher);
        assert_eq!(code, 0);
        assert!(stdout.contains("Usage:"));
        assert!(stderr.is_empty());
    }

    #[test]
    fn unknown_argument_exits_two() {
        let fetcher = MapFetcher(&[]);
        let (code, stdout, stderr) = run_capture(&["myip", "--foo"], &OkLocal("10.0.0.1"), &fetcher);
        assert_eq!(code, 2);
        assert!(stderr.contains("unexpected argument: --foo"));
        assert!(stdout.contains("Usage:"));
    }

    #[test]
    fn print_ips_success() {
        let fetcher = MapFetcher(&[(
            "https://api.ipify.org",
            Ok("8.8.8.8"),
        )]);
        let (code, stdout, stderr) = run_capture(&["myip"], &OkLocal("10.0.0.1"), &fetcher);
        assert_eq!(code, 0);
        assert_eq!(stdout, "local: 10.0.0.1\npublic: 8.8.8.8\n");
        assert!(stderr.is_empty());
    }

    #[test]
    fn print_ips_partial_failure_exits_one() {
        let fetcher = MapFetcher(&[(
            "https://api.ipify.org",
            Ok("1.1.1.1"),
        )]);
        let (code, stdout, stderr) = run_capture(&["myip"], &FailLocal, &fetcher);
        assert_eq!(code, 1);
        assert_eq!(stdout, "public: 1.1.1.1\n");
        assert!(stderr.contains("myip: local: no route"));
    }
}
