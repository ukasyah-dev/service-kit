use anyhow::Ok;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    serve, Json, Router,
};
use tokio::net::TcpListener;

use crate::{config::Config, model::BasicResponse, shutdown};

impl IntoResponse for crate::error::Error {
    fn into_response(self) -> Response {
        let code = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let body = Json(BasicResponse {
            message: self.message,
        });
        (code, body).into_response()
    }
}

pub struct Server {
    config: Config,
    pub router: Router,
}

impl Server {
    pub async fn start(&self) -> Result<(), anyhow::Error> {
        log::info!("Listening on port {}", self.config.http_port);

        let address = format!("0.0.0.0:{}", self.config.http_port);

        let listener = TcpListener::bind(address).await?;

        serve(listener, self.router.clone())
            .with_graceful_shutdown(shutdown::wait_for_signal())
            .await?;

        Ok(())
    }
}

pub fn new_server(config: Config) -> Server {
    Server {
        config,
        router: Router::new(),
    }
}
