use dotenvy::dotenv;
use envconfig::Envconfig;

#[derive(Envconfig, Clone)]
pub struct Config {
    #[envconfig(from = "LOG_LEVEL", default = "info")]
    pub log_level: String,

    #[envconfig(from = "HTTP_PORT", default = "3000")]
    pub http_port: u16,

    #[envconfig(from = "NATS_URL")]
    pub nats_url: String,

    #[envconfig(from = "SSH_URL")]
    pub ssh_url: String,
}

pub fn load() -> Result<Config, envconfig::Error> {
    dotenv().ok();
    Config::init_from_env()
}

#[cfg(test)]
mod tests {
    use super::load;

    #[test]
    fn it_works() {
        let config = load().unwrap();
        assert_eq!(config.log_level, "info");
        assert_eq!(config.http_port, 3000);
    }
}
