use axum::{serve, Router};
use tokio::net::TcpListener;

use crate::{config::Config, shutdown};

pub struct Server<'a> {
    config: &'a Config,
    pub router: Router,
}

impl<'a> Server<'a> {
    pub async fn start(&self) {
        log::info!("Listening on port {}", self.config.http_port);

        let address = format!("0.0.0.0:{}", self.config.http_port);

        let listener = TcpListener::bind(address).await.unwrap();

        serve(listener, self.router.clone())
            .with_graceful_shutdown(shutdown::wait_for_signal())
            .await
            .unwrap();
    }
}

pub fn new(config: &Config) -> Server {
    Server {
        config,
        router: Router::new(),
    }
}
