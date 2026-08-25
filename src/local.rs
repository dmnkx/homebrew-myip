pub trait LocalAddress {
    fn local_ipv4(&self) -> Result<String, String>;
}

pub struct DefaultRoute;

impl LocalAddress for DefaultRoute {
    fn local_ipv4(&self) -> Result<String, String> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Stub(&'static str);

    impl LocalAddress for Stub {
        fn local_ipv4(&self) -> Result<String, String> {
            Ok(self.0.to_string())
        }
    }

    #[test]
    fn stub_returns_configured_address() {
        assert_eq!(Stub("192.168.0.10").local_ipv4().unwrap(), "192.168.0.10");
    }
}
