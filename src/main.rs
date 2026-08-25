fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("-h" | "--help") => {
            print_help();
            return;
        }
        Some("-V" | "--version") => {
            println!("myip {}", env!("CARGO_PKG_VERSION"));
            return;
        }
        Some(other) => {
            eprintln!("myip: unexpected argument: {other}");
            print_help();
            std::process::exit(2);
        }
        None => {}
    }

    let local = local_ip();
    let public = public_ip();
    let mut failed = false;

    match &local {
        Ok(ip) => println!("local: {ip}"),
        Err(err) => {
            eprintln!("myip: local: {err}");
            failed = true;
        }
    }

    match &public {
        Ok(ip) => println!("public: {ip}"),
        Err(err) => {
            eprintln!("myip: public: {err}");
            failed = true;
        }
    }

    if failed {
        std::process::exit(1);
    }
}

fn print_help() {
    print!(
        "\
myip {}
Print local and public IPv4 addresses.

Usage:
  myip
  myip -h | --help
  myip -V | --version
",
        env!("CARGO_PKG_VERSION")
    );
}

fn local_ip() -> Result<String, String> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").map_err(|err| err.to_string())?;
    socket
        .connect("8.8.8.8:80")
        .map_err(|err| err.to_string())?;
    Ok(socket
        .local_addr()
        .map_err(|err| err.to_string())?
        .ip()
        .to_string())
}

fn public_ip() -> Result<String, String> {
    const ENDPOINTS: [&str; 3] = [
        "https://api.ipify.org",
        "https://ifconfig.me/ip",
        "https://icanhazip.com",
    ];

    let mut last_error = String::from("could not fetch public IP");

    for url in ENDPOINTS {
        match fetch_text(url) {
            Ok(body) => {
                let ip = body.trim();
                if !ip.is_empty() {
                    return Ok(ip.to_string());
                }
                last_error = format!("{url} returned an empty body");
            }
            Err(err) => last_error = format!("{url}: {err}"),
        }
    }

    Err(last_error)
}

fn fetch_text(url: &str) -> Result<String, String> {
    let response = ureq::get(url)
        .timeout(std::time::Duration::from_secs(5))
        .call()
        .map_err(|err| err.to_string())?;

    response.into_string().map_err(|err| err.to_string())
}
