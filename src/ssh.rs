use std::sync::Arc;

use russh::{client, keys};
use url::Url;

use crate::config::Config;

pub async fn new_client(
    config: Config,
) -> Result<client::Handle<ClientHandler>, Box<dyn std::error::Error>> {
    let url = Url::parse(&config.ssh_url)?;

    let host = url.host_str().unwrap();
    let port = url.port().unwrap_or(22);
    let username = url.username();
    let password = url.password().unwrap();
    let address = format!("{}:{}", host, port);

    let ssh_config = Arc::new(client::Config::default());

    let handler = ClientHandler {};

    let mut session = client::connect(ssh_config, address, handler).await?;

    session.authenticate_password(username, password).await?;

    Ok(session)
}

pub struct ClientHandler {}

#[async_trait::async_trait]
impl client::Handler for ClientHandler {
    type Error = anyhow::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &keys::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }
}
