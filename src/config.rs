use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub stage: String,
    pub app_id: String,
    pub app_secret: String,
    pub api: Api,
    pub app_store: AppStore,
    pub bot: Bot,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Api {
    pub public: Public,
}

#[derive(Debug, Deserialize)]
pub struct Public {
    pub http: Http,
}

#[derive(Debug, Deserialize)]
pub struct Http {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppStore {
    pub base_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Bot {
    pub name: String,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let stage = std::env::var("STAGE").unwrap_or_else(|_| "development".into());
        let path = format!("config/{}.yml", stage);
        let data = fs::read_to_string(&path)?;
        let mut config: Config = serde_yaml::from_str(&data)?;
        config.stage = stage;
        Ok(config)
    }
}
