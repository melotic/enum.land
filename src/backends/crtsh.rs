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
    base_url: String,
}

impl CrtshClientImpl {
    pub fn new(client: Client, base_url: String) -> Self {
        Self { client, base_url }
    }
}

impl Default for CrtshClientImpl {
    fn default() -> Self {
        // create client with user-agent
        let client = Client::builder()
            .user_agent("enumland/0.1.0")
            .build()
            .unwrap();

        Self::new(client, "https://crt.sh".to_string())
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
        let url = format!("{}/?q={}&output=json", self.base_url, domain);
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
        let mut server = mockito::Server::new();
        let mock = server
            .mock("GET", "/?q=example.com&output=json")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file("tests/data/crtsh.json")
            .create_async()
            .await;
        let client = CrtshClientImpl::new(reqwest::Client::new(), server.url());

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
        mock.assert_async().await;
    }
}
