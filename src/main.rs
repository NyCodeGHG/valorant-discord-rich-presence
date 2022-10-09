#![forbid(unsafe_code)]

use std::time::Duration;

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
    tracing_subscriber::fmt::init();
    let client = RiotSocketClient::from_lockfile(Box::new(|| {
        let lockfile_content = Lockfile::read_from_fs().ok()?;
        Lockfile::parse(lockfile_content).ok()
    }))
    .await?;
    client.subscribe(game::Event::Presences).await;
    tokio::time::sleep(Duration::from_secs(3600)).await;
    Ok(())
}
