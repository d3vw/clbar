use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_clash_api_url")]
    pub clash_api_url: String,

    #[serde(default = "default_clash_secret")]
    pub clash_secret: String,

    #[serde(default = "default_proxy_groups")]
    pub proxy_groups: Vec<String>,

    #[serde(default = "default_refresh_interval")]
    pub refresh_interval_secs: u64,

    #[serde(default = "default_delay_test_group")]
    pub delay_test_group: String,

    #[serde(default = "default_delay_test_url")]
    pub delay_test_url: String,

    #[serde(default = "default_delay_test_timeout")]
    pub delay_test_timeout: u32,
}

fn default_clash_api_url() -> String {
    "http://127.0.0.1:9090".to_string()
}

fn default_clash_secret() -> String {
    String::new()
}

fn default_proxy_groups() -> Vec<String> {
    vec!["PROXY".to_string()]
}

fn default_refresh_interval() -> u64 {
    30
}

fn default_delay_test_group() -> String {
    "all".to_string()
}

fn default_delay_test_url() -> String {
    "https://www.gstatic.com/generate_204".to_string()
}

fn default_delay_test_timeout() -> u32 {
    5000
}

impl Default for Config {
    fn default() -> Self {
        Config {
            clash_api_url: default_clash_api_url(),
            clash_secret: default_clash_secret(),
            proxy_groups: default_proxy_groups(),
            refresh_interval_secs: default_refresh_interval(),
            delay_test_group: default_delay_test_group(),
            delay_test_url: default_delay_test_url(),
            delay_test_timeout: default_delay_test_timeout(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            let default_config = Config::default();
            default_config.save()?;
            return Ok(default_config);
        }

        let content = fs::read_to_string(&config_path)
            .context("Failed to read config file")?;

        let config: Config = toml::from_str(&content)
            .context("Failed to parse config file")?;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create config directory")?;
        }

        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;

        fs::write(&config_path, content)
            .context("Failed to write config file")?;

        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Failed to get config directory")?;

        Ok(config_dir.join("clbar").join("config.toml"))
    }
}
