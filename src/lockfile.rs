use std::env;
use std::path::Path;

use tokio::fs;

#[derive(Debug)]
pub struct RiotCredentials {
    name: String,
    pid: u32,
    port: u32,
    password: String,
    protocol: String,
}

pub async fn get_lockfile_credentials() -> Result<RiotCredentials, ()> {
    let local_app_data = match env::var("LOCALAPPDATA") {
        Ok(path) => path,
        Err(_) => return Err(()),
    };
    let local_app_data = Path::new(&local_app_data);
    let lockfile = local_app_data.join("Riot Games/Riot Client/Config/lockfile");
    let lockfile_content = match fs::read_to_string(&lockfile).await {
        Ok(content) => content,
        Err(_) => return Err(()),
    };
    let mut lockfile_values = lockfile_content.split(':');
    let name = lockfile_values.next().ok_or(())?.to_string();
    let pid = match lockfile_values.next().ok_or(())?.parse::<u32>() {
        Ok(n) => n,
        Err(_) => return Err(()),
    };
    let port = match lockfile_values.next().ok_or(())?.parse::<u32>() {
        Ok(n) => n,
        Err(_) => return Err(()),
    };
    let password = lockfile_values.next().ok_or(())?.to_string();
    let protocol = lockfile_values.next().ok_or(())?.to_string();
    Ok(RiotCredentials {
        name,
        pid,
        port,
        password,
        protocol,
    })
}
