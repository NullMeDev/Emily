use anyhow::{anyhow, Result};
use scraper::{Html, Selector};
use std::collections::HashMap;

/// Scrape dynamic fields from HTML pages
pub struct FieldScraper;

impl FieldScraper {
    /// Extract nonce from WooCommerce forms
    pub fn extract_wc_nonce(html: &str, nonce_name: &str) -> Result<String> {
        let document = Html::parse_document(html);
        
        // Try input field first
        let input_selector = Selector::parse(&format!(r#"input[name="{}"]"#, nonce_name))
            .map_err(|e| anyhow!("Invalid selector: {}", e))?;
        
        if let Some(input) = document.select(&input_selector).next() {
            if let Some(value) = input.value().attr("value") {
                return Ok(value.to_string());
            }
        }

        Err(anyhow!("Nonce '{}' not found in HTML", nonce_name))
    }

    /// Extract multiple nonces at once
    pub fn extract_multiple_nonces(html: &str, nonce_names: &[&str]) -> HashMap<String, String> {
        let mut nonces = HashMap::new();
        
        for nonce_name in nonce_names {
            if let Ok(value) = Self::extract_wc_nonce(html, nonce_name) {
                nonces.insert(nonce_name.to_string(), value);
            }
        }
        
        nonces
    }

    /// Extract Stripe publishable key from page
    pub fn extract_stripe_key(html: &str) -> Result<String> {
        // Try script tags with Stripe.setPublishableKey
        if let Some(key) = Self::extract_between(html, "Stripe.setPublishableKey('", "'") {
            return Ok(key);
        }

        // Try data attributes
        let document = Html::parse_document(html);
        let selectors = [
            r#"[data-stripe-publishable-key]"#,
            r#"[data-key]"#,
            r#".stripe-publishable-key"#,
        ];

        for selector_str in &selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(element) = document.select(&selector).next() {
                    if let Some(key) = element.value().attr("data-stripe-publishable-key")
                        .or_else(|| element.value().attr("data-key"))
                    {
                        if key.starts_with("pk_") {
                            return Ok(key.to_string());
                        }
                    }
                }
            }
        }

        // Try inline scripts
        let script_selector = Selector::parse("script").unwrap();
        for script in document.select(&script_selector) {
            let text = script.text().collect::<String>();
            if let Some(key) = Self::extract_stripe_key_from_script(&text) {
                return Ok(key);
            }
        }

        Err(anyhow!("Stripe publishable key not found"))
    }

    /// Extract Stripe key from JavaScript code
    fn extract_stripe_key_from_script(script: &str) -> Option<String> {
        // Common patterns
        let patterns = [
            r#"pk_live_[a-zA-Z0-9]+"#,
            r#"pk_test_[a-zA-Z0-9]+"#,
        ];

        for pattern in &patterns {
            if let Ok(re) = regex::Regex::new(pattern) {
                if let Some(cap) = re.find(script) {
                    return Some(cap.as_str().to_string());
                }
            }
        }

        None
    }

    /// Extract Braintree client token
    pub fn extract_braintree_token(html: &str) -> Result<String> {
        // Try data attribute
        let document = Html::parse_document(html);
        let selector = Selector::parse(r#"[data-braintree-client-token]"#)
            .map_err(|e| anyhow!("Invalid selector: {}", e))?;

        if let Some(element) = document.select(&selector).next() {
            if let Some(token) = element.value().attr("data-braintree-client-token") {
                return Ok(token.to_string());
            }
        }

        // Try hidden input
        let input_selector = Selector::parse(r#"input[name="braintree_client_token"]"#)
            .map_err(|e| anyhow!("Invalid selector: {}", e))?;

        if let Some(input) = document.select(&input_selector).next() {
            if let Some(value) = input.value().attr("value") {
                return Ok(value.to_string());
            }
        }

        Err(anyhow!("Braintree client token not found"))
    }

    /// Extract CSRF token from meta tag
    pub fn extract_csrf_token(html: &str) -> Result<String> {
        let document = Html::parse_document(html);
        let selector = Selector::parse(r#"meta[name="csrf-token"]"#)
            .map_err(|e| anyhow!("Invalid selector: {}", e))?;

        if let Some(meta) = document.select(&selector).next() {
            if let Some(content) = meta.value().attr("content") {
                return Ok(content.to_string());
            }
        }

        Err(anyhow!("CSRF token not found"))
    }

    /// Extract form action URL
    pub fn extract_form_action(html: &str, form_id: Option<&str>) -> Result<String> {
        let document = Html::parse_document(html);
        
        let selector_str = if let Some(id) = form_id {
            format!(r#"form#{}"#, id)
        } else {
            "form".to_string()
        };

        let selector = Selector::parse(&selector_str)
            .map_err(|e| anyhow!("Invalid selector: {}", e))?;

        if let Some(form) = document.select(&selector).next() {
            if let Some(action) = form.value().attr("action") {
                return Ok(action.to_string());
            }
        }

        Err(anyhow!("Form action not found"))
    }

    /// Extract all hidden input fields from a form
    pub fn extract_hidden_fields(html: &str, form_id: Option<&str>) -> HashMap<String, String> {
        let document = Html::parse_document(html);
        let mut fields = HashMap::new();

        let selector_str = if let Some(id) = form_id {
            format!(r#"form#{} input[type="hidden"]"#, id)
        } else {
            r#"input[type="hidden"]"#.to_string()
        };

        if let Ok(selector) = Selector::parse(&selector_str) {
            for input in document.select(&selector) {
                if let (Some(name), Some(value)) = (
                    input.value().attr("name"),
                    input.value().attr("value"),
                ) {
                    fields.insert(name.to_string(), value.to_string());
                }
            }
        }

        fields
    }

    /// Helper: Extract text between two markers
    fn extract_between(text: &str, start: &str, end: &str) -> Option<String> {
        let start_idx = text.find(start)? + start.len();
        let end_idx = text[start_idx..].find(end)? + start_idx;
        Some(text[start_idx..end_idx].to_string())
    }

    /// Extract value using regex pattern
    pub fn extract_with_regex(text: &str, pattern: &str) -> Result<String> {
        let re = regex::Regex::new(pattern)
            .map_err(|e| anyhow!("Invalid regex pattern: {}", e))?;
        
        re.captures(text)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| anyhow!("Pattern not found in text"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_nonce() {
        let html = r#"<input type="hidden" name="woocommerce-register-nonce" value="abc123" />"#;
        let nonce = FieldScraper::extract_wc_nonce(html, "woocommerce-register-nonce").unwrap();
        assert_eq!(nonce, "abc123");
    }

    #[test]
    fn test_extract_stripe_key() {
        let html = r#"<script>Stripe.setPublishableKey('pk_test_1234567890');</script>"#;
        let key = FieldScraper::extract_stripe_key(html).unwrap();
        assert_eq!(key, "pk_test_1234567890");
    }

    #[test]
    fn test_extract_hidden_fields() {
        let html = r#"
            <form id="test-form">
                <input type="hidden" name="field1" value="value1" />
                <input type="hidden" name="field2" value="value2" />
            </form>
        "#;
        let fields = FieldScraper::extract_hidden_fields(html, Some("test-form"));
        assert_eq!(fields.len(), 2);
        assert_eq!(fields.get("field1"), Some(&"value1".to_string()));
    }

    #[test]
    fn test_extract_with_regex() {
        let text = "The payment ID is: pm_1234567890";
        let result = FieldScraper::extract_with_regex(text, r"pm_(\w+)").unwrap();
        assert_eq!(result, "1234567890");
    }
}
