use crate::utils::generators::{generate_guid, generate_hex_string, generate_random_string};
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use serde_json::Value;

/// Stripe token manager for generating and parsing payment tokens
pub struct StripeTokenManager {
    pub guid: String,
    pub muid: String,
    pub sid: String,
}

impl StripeTokenManager {
    /// Create a new token manager with fresh tokens
    pub fn new() -> Self {
        Self {
            guid: generate_guid(),
            muid: format!("mid_{}", generate_random_string(32)),
            sid: format!("sid_{}", generate_random_string(32)),
        }
    }

    /// Create with custom tokens (e.g., from cookies)
    pub fn with_tokens(guid: String, muid: String, sid: String) -> Self {
        Self { guid, muid, sid }
    }

    /// Get GUID
    pub fn guid(&self) -> &str {
        &self.guid
    }

    /// Get MID
    pub fn muid(&self) -> &str {
        &self.muid
    }

    /// Get SID
    pub fn sid(&self) -> &str {
        &self.sid
    }
}

impl Default for StripeTokenManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Braintree token manager
pub struct BraintreeTokenManager;

impl BraintreeTokenManager {
    /// Parse Braintree client token from base64
    pub fn parse_client_token(encoded_token: &str) -> Result<BraintreeClientToken> {
        let decoded = general_purpose::STANDARD.decode(encoded_token)?;
        let json_str = String::from_utf8(decoded)?;
        let value: Value = serde_json::from_str(&json_str)?;

        Ok(BraintreeClientToken {
            authorization_fingerprint: value["authorizationFingerprint"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            merchant_id: value["merchantId"].as_str().unwrap_or("").to_string(),
            environment: value["environment"].as_str().unwrap_or("").to_string(),
        })
    }

    /// Generate correlation ID for Braintree
    pub fn generate_correlation_id() -> String {
        generate_hex_string(32)
    }

    /// Generate device session ID
    pub fn generate_device_session_id() -> String {
        uuid::Uuid::new_v4().to_string()
    }
}

/// Parsed Braintree client token
#[derive(Debug, Clone)]
pub struct BraintreeClientToken {
    pub authorization_fingerprint: String,
    pub merchant_id: String,
    pub environment: String,
}

/// Extract payment method ID from Stripe response
pub fn extract_stripe_pm_id(response_text: &str) -> Option<String> {
    // Try JSON parsing first
    if let Ok(json) = serde_json::from_str::<Value>(response_text) {
        if let Some(id) = json["id"].as_str() {
            if id.starts_with("pm_") {
                return Some(id.to_string());
            }
        }
    }

    // Fallback to regex
    let re = regex::Regex::new(r#""id"\s*:\s*"(pm_[^"]+)""#).ok()?;
    re.captures(response_text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
}

/// Extract payment intent ID and client secret from response
pub fn extract_payment_intent(response_text: &str) -> Option<(String, String)> {
    if let Ok(json) = serde_json::from_str::<Value>(response_text) {
        let pi_id = json["id"].as_str()?;
        let client_secret = json["client_secret"].as_str()?;
        
        if pi_id.starts_with("pi_") {
            return Some((pi_id.to_string(), client_secret.to_string()));
        }
    }
    None
}

/// Extract Stripe source ID from response
pub fn extract_stripe_source_id(response_text: &str) -> Option<String> {
    if let Ok(json) = serde_json::from_str::<Value>(response_text) {
        if let Some(id) = json["id"].as_str() {
            if id.starts_with("src_") {
                return Some(id.to_string());
            }
        }
    }

    // Fallback to regex
    let re = regex::Regex::new(r#""id"\s*:\s*"(src_[^"]+)""#).ok()?;
    re.captures(response_text)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stripe_token_manager() {
        let manager = StripeTokenManager::new();
        assert!(manager.guid().starts_with("guid_"));
        assert!(manager.muid().starts_with("mid_"));
        assert!(manager.sid().starts_with("sid_"));
    }

    #[test]
    fn test_extract_pm_id() {
        let json = r#"{"id": "pm_1234567890", "object": "payment_method"}"#;
        let pm_id = extract_stripe_pm_id(json);
        assert_eq!(pm_id, Some("pm_1234567890".to_string()));
    }

    #[test]
    fn test_extract_source_id() {
        let json = r#"{"id": "src_1234567890", "object": "source"}"#;
        let src_id = extract_stripe_source_id(json);
        assert_eq!(src_id, Some("src_1234567890".to_string()));
    }

    #[test]
    fn test_braintree_correlation_id() {
        let corr_id = BraintreeTokenManager::generate_correlation_id();
        assert_eq!(corr_id.len(), 32);
    }
}
