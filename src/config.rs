use figment::providers::{Format, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub prefix: String,
    pub token: String,
}

impl Config {
    pub fn new() -> Config {
        figment::Figment::new()
            .join(Json::file("Config.json"))
            .extract::<Config>()
            .expect("Failed to load config")
    }
}
