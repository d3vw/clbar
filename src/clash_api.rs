use anyhow::{Context, Result};
use reqwest::Client;
use std::collections::HashMap;

use crate::config::Config;
use crate::models::{ProxyGroup, ProxiesResponse, SwitchRequest};

pub struct ClashApi {
    client: Client,
    base_url: String,
    secret: String,
}

impl ClashApi {
    pub fn new(config: &Config) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(ClashApi {
            client,
            base_url: config.clash_api_url.clone(),
            secret: config.clash_secret.clone(),
        })
    }

    pub async fn get_proxies(&self) -> Result<HashMap<String, ProxyGroup>> {
        let url = format!("{}/proxies", self.base_url);

        let mut request = self.client.get(&url);

        if !self.secret.is_empty() {
            request = request.header("Authorization", format!("Bearer {}", self.secret));
        }

        let response = request
            .send()
            .await
            .context("Failed to send request to Clash API")?;

        if !response.status().is_success() {
            anyhow::bail!("Clash API returned error: {}", response.status());
        }

        let proxies_response: ProxiesResponse = response
            .json()
            .await
            .context("Failed to parse proxies response")?;

        let mut proxy_groups = HashMap::new();

        // Filter for proxy groups (Selector, URLTest, Fallback)
        for (name, proxy) in proxies_response.proxies {
            if proxy.proxy_type == "Selector" || proxy.proxy_type == "URLTest" || proxy.proxy_type == "Fallback" {
                // Convert Proxy to ProxyGroup
                let group = ProxyGroup {
                    name: proxy.name,
                    group_type: proxy.proxy_type,
                    now: proxy.now,
                    all: proxy.all,
                };
                proxy_groups.insert(name, group);
            }
        }

        Ok(proxy_groups)
    }

    pub async fn switch_node(&self, group_name: &str, node_name: &str) -> Result<()> {
        let url = format!("{}/proxies/{}", self.base_url, group_name);

        let mut request = self.client.put(&url);

        if !self.secret.is_empty() {
            request = request.header("Authorization", format!("Bearer {}", self.secret));
        }

        let switch_request = SwitchRequest {
            name: node_name.to_string(),
        };

        let response = request
            .json(&switch_request)
            .send()
            .await
            .context("Failed to send switch request to Clash API")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to switch node: {}", response.status());
        }

        Ok(())
    }

    pub async fn test_connection(&self) -> Result<()> {
        let url = format!("{}/proxies", self.base_url);

        let mut request = self.client.get(&url);

        if !self.secret.is_empty() {
            request = request.header("Authorization", format!("Bearer {}", self.secret));
        }

        let response = request
            .send()
            .await
            .context("Failed to connect to Clash API")?;

        if !response.status().is_success() {
            anyhow::bail!("Clash API returned error: {}", response.status());
        }

        Ok(())
    }
}
