use lazy_regex::regex_captures;
use thiserror::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Lockfile<'a> {
    name: &'a str,
    process_id: u16,
    port: u16,
    password: &'a str,
    protocol: Protocol,
}

impl<'a> Lockfile<'a> {
    /// Parses the riot client lockfile in the format `name:pid:port:password:protocol`.
    pub fn parse(text: &str) -> Result<Lockfile, Error> {
        let (_, name, process_id, port, password, protocol) =
            regex_captures!(r#"([^:]+):(\d+):(\d+):([^:]+):(https?)"#, text)
                .ok_or(Error::InvalidFormat)?;

        let process_id: u16 = process_id.parse().map_err(|_| Error::InvalidProcessId)?;
        let port: u16 = port.parse().map_err(|_| Error::InvalidPort)?;

        let protocol = match protocol {
            "https" => Protocol::Secure,
            "http" => Protocol::Insecure,
            _ => unreachable!("the regex ensures this can't be a different value"),
        };

        let lockfile = Lockfile {
            name,
            process_id,
            port,
            password,
            protocol,
        };
        Ok(lockfile)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Protocol {
    Insecure,
    Secure,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum Error {
    #[error("The lockfile text has an invalid format.")]
    InvalidFormat,
    #[error("The port is an invalid number")]
    InvalidPort,
    #[error("The process id is an invalid number")]
    InvalidProcessId,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_lockfile() {
        let text = "Riot Client:22568:54846:$@ah7iGKU^9eXkqiVRvZ4:https";
        let lockfile = Lockfile::parse(text).unwrap();
        assert_eq!(
            lockfile,
            Lockfile {
                name: "Riot Client",
                process_id: 22568,
                port: 54846,
                password: "$@ah7iGKU^9eXkqiVRvZ4",
                protocol: Protocol::Secure
            }
        );
    }

    #[test]
    fn invalid_string() {
        let text = "hello world";
        assert_eq!(Lockfile::parse(text).unwrap_err(), Error::InvalidFormat);
    }

    #[test]
    fn invalid_process_id() {
        let text = "Riot Client:22568225688:54846:$@ah7iGKU^9eXkqiVRvZ4:https";
        assert_eq!(Lockfile::parse(text).unwrap_err(), Error::InvalidProcessId);
    }

    #[test]
    fn invalid_port() {
        let text = "Riot Client:22568:5484654846:$@ah7iGKU^9eXkqiVRvZ4:https";
        assert_eq!(Lockfile::parse(text).unwrap_err(), Error::InvalidPort);
    }
}
