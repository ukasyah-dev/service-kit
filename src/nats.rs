use async_nats::{connect, Client, Error};

use crate::config::Config;

pub async fn new_client(config: Config) -> Result<Client, Error> {
    let client = connect(config.nats_url).await?;
    Ok(client)
}
