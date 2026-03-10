use crate::storage::rdb::load_from_rdb;
use std::io::Error;

pub async fn restore_from_rdb() -> Result<(), Error> {
    load_from_rdb().await
}
