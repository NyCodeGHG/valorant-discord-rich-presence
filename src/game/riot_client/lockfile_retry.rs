use tokio_retry::{strategy::FixedInterval, Retry};

use crate::lockfile::Lockfile;

use eyre::Result;

use super::LockfileSupplier;

pub async fn read_lockfile_with_retry(supplier: &LockfileSupplier) -> Option<Lockfile> {
    let strategy = FixedInterval::from_millis(500);
    Retry::spawn(strategy, || read_lockfile(supplier))
        .await
        .ok()
}

#[inline]
async fn read_lockfile(supplier: &LockfileSupplier) -> Result<Lockfile, ()> {
    match supplier() {
        Some(lockfile) => Ok(lockfile),
        None => Err(()),
    }
}
