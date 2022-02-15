use std::env;
use std::io::Error;
use std::num::ParseIntError;
use std::path::Path;

use tokio::fs;

#[derive(Debug)]
pub struct RiotCredentials {
    pub name: String,
    pub pid: u32,
    pub port: u32,
    pub password: String,
    pub protocol: String,
}

pub async fn get_lockfile_credentials() -> Result<RiotCredentials, LockfileError> {
    let local_app_data = env::var("LOCALAPPDATA").map_err(|_| LockfileError::EnvReadFailed)?;
    let local_app_data = Path::new(&local_app_data);
    let lockfile = local_app_data.join("Riot Games/Riot Client/Config/lockfile");
    let lockfile_content = match fs::read_to_string(&lockfile).await {
        Ok(content) => content,
        Err(error) => return Err(LockfileError::FileReadFailed(error)),
    };
    let mut lockfile_values = lockfile_content.split(':');
    let name = lockfile_values
        .next()
        .ok_or(LockfileError::FieldMissing("name"))?
        .to_string();
    let pid = lockfile_values
        .next()
        .ok_or(LockfileError::FieldMissing("pid"))?
        .parse::<u32>()
        .map_err(LockfileError::NumberFormat)?;
    let port = lockfile_values
        .next()
        .ok_or(LockfileError::FieldMissing("port"))?
        .parse::<u32>()
        .map_err(LockfileError::NumberFormat)?;
    let password = lockfile_values
        .next()
        .ok_or(LockfileError::FieldMissing("password"))?
        .to_string();
    let protocol = lockfile_values
        .next()
        .ok_or(LockfileError::FieldMissing("protocol"))?
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
pub enum LockfileError {
    EnvReadFailed,
    FileReadFailed(Error),
    FieldMissing(&'static str),
    NumberFormat(ParseIntError),
}
