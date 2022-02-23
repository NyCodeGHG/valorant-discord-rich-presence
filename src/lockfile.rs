use std::env;
use std::error::Error;
use std::fmt::Display;
use std::path::Path;

use anyhow::Result;
use tokio::fs;

#[derive(Debug)]
pub struct RiotCredentials {
    pub name: String,
    pub pid: u32,
    pub port: u32,
    pub password: String,
    pub protocol: String,
}

macro_rules! next {
    ($lockfile_values:expr, $name:expr) => {
        $lockfile_values
            .next()
            .ok_or_else(|| FieldMissingError::new($name))?
            .parse()?
    };
}

pub async fn get_lockfile_credentials() -> Result<RiotCredentials> {
    let local_app_data = env::var("LOCALAPPDATA")?;
    let local_app_data = Path::new(&local_app_data);
    let lockfile = local_app_data.join("Riot Games/Riot Client/Config/lockfile");
    let lockfile_content = fs::read_to_string(&lockfile).await?;
    let mut lockfile_values = lockfile_content.split(':');
    let name: String = next!(lockfile_values, "name");
    let pid: u32 = next!(lockfile_values, "pid");
    let port: u32 = next!(lockfile_values, "port");
    let password = next!(lockfile_values, "password");
    let protocol = next!(lockfile_values, "protocol");
    Ok(RiotCredentials {
        name,
        pid,
        port,
        password,
        protocol,
    })
}

#[derive(Debug)]
struct FieldMissingError {
    field: String,
}

impl FieldMissingError {
    fn new(field: &str) -> FieldMissingError {
        FieldMissingError {
            field: field.to_string(),
        }
    }
}

impl Display for FieldMissingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("field {} is missing", self.field))
    }
}

impl Error for FieldMissingError {}
