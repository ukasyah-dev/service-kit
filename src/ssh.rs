use std::sync::Arc;

use anyhow::Ok;
use russh::{client, keys, ChannelMsg};
use tokio::io::AsyncWriteExt;
use url::Url;

use crate::config::Config;

#[derive(Clone, Copy)]
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

pub struct Session {
    session: client::Handle<ClientHandler>,
}

impl Session {
    pub async fn exec(&self, command: &str) -> Result<(), anyhow::Error> {
        let mut channel = self.session.channel_open_session().await?;

        channel.exec(true, command).await?;

        let mut code = None;
        let mut stdout = tokio::io::stdout();

        loop {
            // There's an event available on the session channel
            let Some(msg) = channel.wait().await else {
                break;
            };

            match msg {
                // Write data to the terminal
                ChannelMsg::Data { ref data } => {
                    stdout.write_all(data).await?;
                    stdout.flush().await?;
                }
                // The command has returned an exit code
                ChannelMsg::ExitStatus { exit_status } => {
                    code = Some(exit_status);
                    // cannot leave the loop immediately, there might still be more data to receive
                }
                _ => {}
            }
        }

        if let Some(code) = code {
            if code != 0 {
                return Err(anyhow::anyhow!("Command failed with exit code {}", code));
            }
        }

        Ok(())
    }
}

pub async fn new_session(config: Config) -> Result<Session, anyhow::Error> {
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

    Ok(Session { session })
}
