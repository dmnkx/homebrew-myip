#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    PrintIps,
    Help,
    Version,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    Unexpected(String),
}

pub fn parse_args<I, S>(args: I) -> Result<Command, ParseError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut iter = args.into_iter();
    let _program = iter.next();
    match iter.next().as_ref().map(|value| value.as_ref()) {
        None => Ok(Command::PrintIps),
        Some("-h" | "--help") => Ok(Command::Help),
        Some("-V" | "--version") => Ok(Command::Version),
        Some(other) => Err(ParseError::Unexpected(other.to_string())),
    }
}

pub fn help_text(version: &str) -> String {
    format!(
        "\
myip {version}
Print local and public IPv4 addresses.

Usage:
  myip
  myip -h | --help
  myip -V | --version
"
    )
}

pub fn version_text(version: &str) -> String {
    format!("myip {version}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_args_prints_ips() {
        assert_eq!(parse_args(["myip"]), Ok(Command::PrintIps));
    }

    #[test]
    fn help_aliases() {
        assert_eq!(parse_args(["myip", "-h"]), Ok(Command::Help));
        assert_eq!(parse_args(["myip", "--help"]), Ok(Command::Help));
    }

    #[test]
    fn version_aliases() {
        assert_eq!(parse_args(["myip", "-V"]), Ok(Command::Version));
        assert_eq!(parse_args(["myip", "--version"]), Ok(Command::Version));
    }

    #[test]
    fn rejects_unknown_flag() {
        assert_eq!(
            parse_args(["myip", "--json"]),
            Err(ParseError::Unexpected("--json".into()))
        );
    }

    #[test]
    fn version_text_includes_name() {
        assert_eq!(version_text("1.2.3"), "myip 1.2.3");
    }
}
