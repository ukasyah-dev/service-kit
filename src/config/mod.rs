use dotenvy::dotenv;
use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "HTTP_PORT", default = "3000")]
    pub http_port: u16,
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
        assert_eq!(config.http_port, 3000);
    }
}