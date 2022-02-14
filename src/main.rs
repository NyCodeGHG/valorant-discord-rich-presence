use lockfile::get_lockfile_credentials;

pub mod lockfile;

#[tokio::main]
async fn main() -> Result<(), ()>{
    let creds = get_lockfile_credentials().await?;
    println!("{:?}", &creds);
    Ok(())
}
