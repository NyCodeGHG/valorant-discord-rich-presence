use lockfile::{get_lockfile_credentials, LockfileError};

pub mod lockfile;

#[tokio::main]
async fn main() -> Result<(), LockfileError> {
    let creds = get_lockfile_credentials().await?;
    println!("{:?}", &creds);
    Ok(())
}
