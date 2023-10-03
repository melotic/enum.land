use std::collections::HashSet;

use anyhow::anyhow;
use axum::async_trait;
use reqwest::Client;
use serde::Deserialize;
use tracing::{info, instrument, warn};

use crate::{errors::AppError, utils};

#[async_trait]
pub trait CrtshClient {
    async fn get_domains(&self, domain: &str) -> Result<HashSet<String>, AppError>;
}

pub struct CrtshClientImpl {
    client: reqwest::Client,
}

impl CrtshClientImpl {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl Default for CrtshClientImpl {
    fn default() -> Self {
        // create client with user-agent
        let client = Client::builder()
            .user_agent("enumland/0.1.0")
            .build()
            .unwrap();

        Self::new(client)
    }
}

#[derive(Deserialize)]
struct CrtshResponse {
    name_value: String,
}

#[async_trait]
impl CrtshClient for CrtshClientImpl {
    #[instrument(skip(self))]
    async fn get_domains(&self, domain: &str) -> Result<HashSet<String>, AppError> {
        let url = format!("https://crt.sh/?q=%25.{}&output=json", domain);
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!(e))?
            .json::<Vec<CrtshResponse>>()
            .await
            .map_err(|_| AppError::BadResponse)?;

        info!("crt.sh returned {} rows", response.len());

        // Filter emails and split \n separated domains
        let raw_domains = response
            .into_iter()
            .map(|r| r.name_value)
            .collect::<HashSet<String>>();

        let mut domains = HashSet::with_capacity(raw_domains.len());

        for domain in raw_domains {
            let mut domain = domain.split('\n').collect::<Vec<&str>>();
            for d in domain.iter_mut() {
                if !d.contains('@') && !d.starts_with("*.") && utils::valid_domain(d) {
                    domains.insert(d.to_lowercase().to_string());
                } else {
                    warn!("invalid domain: {}", d);
                }
            }
        }

        Ok(domains)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertor::*;

    #[tokio::test]
    async fn example_com_works() {
        let client = CrtshClientImpl::default();
        let domains = client.get_domains("example.com").await.unwrap();

        let domains: Vec<&str> = domains.iter().map(|s| s.as_ref()).collect();
        let expected = vec![
            "example.com",
            "www.example.com",
            "m.example.com",
            "dev.example.com",
            "products.example.com",
            "support.example.com",
            "m.testexample.com",
        ];

        assert_that!(domains).contains_exactly(expected);
    }
}
