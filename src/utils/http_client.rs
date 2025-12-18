use reqwest::{Client, ClientBuilder, Response};
use std::time::Duration;
use anyhow::Result;

/// HTTP client with session management and cookie support
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    /// Create a new HTTP client with default settings
    pub fn new() -> Result<Self> {
        let client = ClientBuilder::new()
            .cookie_store(true)
            .timeout(Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Mobile Safari/537.36")
            .build()?;

        Ok(Self { client })
    }

    /// Create a new HTTP client with custom user agent
    pub fn with_user_agent(user_agent: &str) -> Result<Self> {
        let client = ClientBuilder::new()
            .cookie_store(true)
            .timeout(Duration::from_secs(30))
            .user_agent(user_agent)
            .build()?;

        Ok(Self { client })
    }

    /// Create a new HTTP client with custom timeout
    pub fn with_timeout(timeout_secs: u64) -> Result<Self> {
        let client = ClientBuilder::new()
            .cookie_store(true)
            .timeout(Duration::from_secs(timeout_secs))
            .user_agent("Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Mobile Safari/537.36")
            .build()?;

        Ok(Self { client })
    }

    /// Get the inner reqwest client
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Perform a GET request
    pub async fn get(&self, url: &str) -> Result<Response> {
        let response = self.client.get(url).send().await?;
        Ok(response)
    }

    /// Perform a POST request with form data
    pub async fn post_form<T: serde::Serialize + ?Sized>(
        &self,
        url: &str,
        form: &T,
    ) -> Result<Response> {
        let response = self.client.post(url).form(form).send().await?;
        Ok(response)
    }

    /// Perform a POST request with JSON data
    pub async fn post_json<T: serde::Serialize + ?Sized>(
        &self,
        url: &str,
        json: &T,
    ) -> Result<Response> {
        let response = self.client.post(url).json(json).send().await?;
        Ok(response)
    }

    /// Perform a POST request with raw body
    pub async fn post_body(&self, url: &str, body: String) -> Result<Response> {
        let response = self.client.post(url).body(body).send().await?;
        Ok(response)
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default HTTP client")
    }
}

/// Common user agents
pub mod user_agents {
    pub const CHROME_MOBILE: &str = "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Mobile Safari/537.36";
    pub const CHROME_DESKTOP: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36";
    pub const FIREFOX_DESKTOP: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/121.0";
    pub const SAFARI_MOBILE: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_client() {
        let client = HttpClient::new();
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_custom_user_agent() {
        let client = HttpClient::with_user_agent("TestAgent/1.0");
        assert!(client.is_ok());
    }
}
