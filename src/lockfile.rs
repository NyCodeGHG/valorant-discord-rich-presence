use std::{env, fs, io, path::Path};

use lazy_regex::regex_captures;
use thiserror::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Lockfile<'a> {
    pub name: &'a str,
    pub process_id: u16,
    pub port: u16,
    pub password: &'a str,
    pub protocol: Protocol,
}

impl<'a> Lockfile<'a> {
    /// Parses the riot client lockfile in the format `name:pid:port:password:protocol`.
    pub fn parse(text: &str) -> Result<Lockfile, Error> {
        let (_, name, process_id, port, password, protocol) =
            regex_captures!(r#"([^:]+):(\d+):(\d+):([^:]+):(https?)"#, text)
                .ok_or(Error::Format)?;

        let process_id: u16 = process_id.parse().map_err(|_| Error::ProcessId)?;
        let port: u16 = port.parse().map_err(|_| Error::Port)?;

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

    /// Reads the lockfiles content from the filesystem.
    ///
    /// # Panics
    /// The function panics if the environment variable "LocalAppData" is not set.
    /// This should never happen, otherwise your machine is really fucked. (Windows only ofc)
    pub fn read_from_fs() -> Result<String, io::Error> {
        // construct path to the lockfile
        let local_app_data =
            env::var("LocalAppData").expect("environment variable LocalAppData is not set.");
        let path = Path::new(&local_app_data).join("Riot Games/Riot Client/Config/lockfile");

        // read the lockfile content
        let content = fs::read_to_string(&path)?;
        Ok(content)
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
    Format,
    #[error("The port is an invalid number")]
    Port,
    #[error("The process id is an invalid number")]
    ProcessId,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_lockfile() {
        // this is not a real password. don't worry :)
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
        assert_eq!(Lockfile::parse(text).unwrap_err(), Error::Format);
    }

    #[test]
    fn invalid_process_id() {
        let text = "Riot Client:22568225688:54846:$@ah7iGKU^9eXkqiVRvZ4:https";
        assert_eq!(Lockfile::parse(text).unwrap_err(), Error::ProcessId);
    }

    #[test]
    fn invalid_port() {
        let text = "Riot Client:22568:5484654846:$@ah7iGKU^9eXkqiVRvZ4:https";
        assert_eq!(Lockfile::parse(text).unwrap_err(), Error::Port);
    }
}
