use myip::{DefaultRoute, UreqFetcher};

fn main() {
    std::process::exit(myip::run_default(
        std::env::args(),
        &mut std::io::stdout(),
        &mut std::io::stderr(),
        &DefaultRoute,
        &UreqFetcher,
    ));
}
