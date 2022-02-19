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

pub async fn get_lockfile_credentials() -> Result<RiotCredentials> {
    let local_app_data = env::var("LOCALAPPDATA")?;
    let local_app_data = Path::new(&local_app_data);
    let lockfile = local_app_data.join("Riot Games/Riot Client/Config/lockfile");
    let lockfile_content = fs::read_to_string(&lockfile).await?;
    let mut lockfile_values = lockfile_content.split(':');
    let name = lockfile_values
        .next()
        .ok_or_else(|| FieldMissingError::new("name"))?
        .to_string();
    let pid = lockfile_values
        .next()
        .ok_or_else(|| FieldMissingError::new("pid"))?
        .parse::<u32>()?;
    let port = lockfile_values
        .next()
        .ok_or_else(|| FieldMissingError::new("port"))?
        .parse::<u32>()?;
    let password = lockfile_values
        .next()
        .ok_or_else(|| FieldMissingError::new("password"))?
        .to_string();
    let protocol = lockfile_values
        .next()
        .ok_or_else(|| FieldMissingError::new("protocol"))?
        .to_string();
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
