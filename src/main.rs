#![forbid(unsafe_code)]

use game::RiotSocketClient;
use lockfile::Lockfile;

mod game;
mod lockfile;
mod tls;

#[cfg(not(windows))]
compile_error!(
    "This application is only built for windows because valorant only runs on windows machines."
);

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let lockfile_content = Lockfile::read_from_fs()?;
    let lockfile = Lockfile::parse(&lockfile_content)?;
    let client = RiotSocketClient::from_lockfile(lockfile).await?;
    client.subscribe(game::Event::Presences).await;
    Ok(())
}
