use crate::models::response::PaymentResponse;
use scraper::{Html, Selector};
use serde_json::Value;

/// Parse payment response from various gateway formats
pub struct ResponseParser;

impl ResponseParser {
    /// Parse Stripe error response
    pub fn parse_stripe_error(response_text: &str) -> PaymentResponse {
        if let Ok(json) = serde_json::from_str::<Value>(response_text) {
            if let Some(error) = json.get("error") {
                let message = error["message"]
                    .as_str()
                    .unwrap_or("Unknown error")
                    .to_string();
                let code = error["code"].as_str().unwrap_or("").to_lowercase();
                let decline_code = error["decline_code"].as_str().unwrap_or("").to_lowercase();

                return Self::categorize_stripe_error(&message, &code, &decline_code);
            }
        }

        PaymentResponse::error("Failed to parse Stripe error response".to_string())
    }

    /// Categorize Stripe error into appropriate status
    fn categorize_stripe_error(message: &str, code: &str, decline_code: &str) -> PaymentResponse {
        let msg_lower = message.to_lowercase();

        // CCN Live indicators (card valid but other issues)
        if msg_lower.contains("security code is incorrect")
            || msg_lower.contains("incorrect_cvc")
            || code == "incorrect_cvc"
        {
            return PaymentResponse::ccn_live("Your card's security code is incorrect.".to_string());
        }

        if msg_lower.contains("postal code") || msg_lower.contains("zip code") {
            return PaymentResponse::ccn_live("Your postal code is incorrect.".to_string());
        }

        // Insufficient funds
        if msg_lower.contains("insufficient funds")
            || decline_code == "insufficient_funds"
            || msg_lower.contains("insufficient_funds")
        {
            return PaymentResponse::insufficient_funds();
        }

        // 3DS / Action Required
        if msg_lower.contains("authentication")
            || msg_lower.contains("3d secure")
            || msg_lower.contains("requires_action")
            || code == "payment_intent_authentication_failure"
        {
            return PaymentResponse::three_ds();
        }

        // Expired card
        if msg_lower.contains("expired") || code == "expired_card" {
            return PaymentResponse::declined("Your card has expired.".to_string());
        }

        // Generic decline
        if msg_lower.contains("declined")
            || msg_lower.contains("card was declined")
            || code == "card_declined"
        {
            return PaymentResponse::declined(format!("Card declined: {}", message));
        }

        // Default to error
        PaymentResponse::error(message.to_string())
    }

    /// Parse WooCommerce checkout response
    pub fn parse_woocommerce_response(response_text: &str) -> PaymentResponse {
        // Try JSON first
        if let Ok(json) = serde_json::from_str::<Value>(response_text) {
            if json["result"].as_str() == Some("success") {
                return PaymentResponse::charged("Payment successful".to_string());
            }

            if let Some(messages) = json["messages"].as_str() {
                return Self::parse_woocommerce_message(messages);
            }
        }

        // Try HTML parsing
        let html_lower = response_text.to_lowercase();
        
        if html_lower.contains("order received") || html_lower.contains("thank you") {
            return PaymentResponse::charged("Order completed successfully".to_string());
        }

        if html_lower.contains("payment failed") || html_lower.contains("error") {
            return Self::extract_woocommerce_error(response_text);
        }

        PaymentResponse::error("Unable to parse WooCommerce response".to_string())
    }

    /// Parse WooCommerce error message
    fn parse_woocommerce_message(message: &str) -> PaymentResponse {
        let msg_lower = message.to_lowercase();

        if msg_lower.contains("security code is incorrect") {
            return PaymentResponse::ccn_live("Security code is incorrect".to_string());
        }

        if msg_lower.contains("insufficient funds") {
            return PaymentResponse::insufficient_funds();
        }

        if msg_lower.contains("expired") {
            return PaymentResponse::declined("Card has expired".to_string());
        }

        if msg_lower.contains("declined") {
            return PaymentResponse::declined(message.to_string());
        }

        PaymentResponse::error(message.to_string())
    }

    /// Extract error from WooCommerce HTML
    fn extract_woocommerce_error(html: &str) -> PaymentResponse {
        let document = Html::parse_document(html);
        
        // Try to find error message in common selectors
        let selectors = [
            ".woocommerce-error",
            ".woocommerce-message",
            ".wc-block-components-notice-banner__content",
            "ul.woocommerce-error li",
        ];

        for selector_str in &selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(element) = document.select(&selector).next() {
                    let text = element.text().collect::<String>();
                    if !text.trim().is_empty() {
                        return Self::parse_woocommerce_message(&text);
                    }
                }
            }
        }

        PaymentResponse::error("Payment failed - unable to extract error details".to_string())
    }

    /// Parse Braintree response
    pub fn parse_braintree_response(response_text: &str) -> PaymentResponse {
        let html_lower = response_text.to_lowercase();

        // Success indicators
        if html_lower.contains("payment method successfully added")
            || html_lower.contains("successfully added")
        {
            return PaymentResponse::charged("Payment method added successfully".to_string());
        }

        // Extract reason from HTML
        if let Some(reason) = Self::extract_between(response_text, "Reason: ", "\t\t</li>") {
            let reason_lower = reason.to_lowercase();

            if reason_lower.contains("declined cvv") || reason_lower.contains("cvv") {
                return PaymentResponse::ccn_live(format!("CVV declined: {}", reason));
            }

            if reason_lower.contains("insufficient funds") {
                return PaymentResponse::insufficient_funds();
            }

            if reason_lower.contains("postal code") || reason_lower.contains("address") {
                return PaymentResponse::ccn_live(format!("AVS mismatch: {}", reason));
            }

            return PaymentResponse::declined(reason.to_string());
        }

        PaymentResponse::error("Unable to parse Braintree response".to_string())
    }

    /// Parse GiveWP donation response
    pub fn parse_givewp_response(response_text: &str) -> PaymentResponse {
        if let Ok(json) = serde_json::from_str::<Value>(response_text) {
            if json["success"].as_bool() == Some(true) {
                return PaymentResponse::charged("Donation successful".to_string());
            }

            if let Some(data) = json.get("data") {
                if let Some(error_message) = data["error_message"].as_str() {
                    return Self::categorize_stripe_error(error_message, "", "");
                }
            }
        }

        PaymentResponse::error("Unable to parse GiveWP response".to_string())
    }

    /// Helper: Extract text between two markers
    fn extract_between(text: &str, start: &str, end: &str) -> Option<String> {
        let start_idx = text.find(start)? + start.len();
        let end_idx = text[start_idx..].find(end)? + start_idx;
        Some(text[start_idx..end_idx].trim().to_string())
    }

    /// Parse generic HTML success page
    pub fn is_success_page(html: &str) -> bool {
        let html_lower = html.to_lowercase();
        html_lower.contains("thank you")
            || html_lower.contains("order received")
            || html_lower.contains("payment successful")
            || html_lower.contains("donation successful")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_stripe_cvc_error() {
        let json = r#"{"error": {"message": "Your card's security code is incorrect.", "code": "incorrect_cvc"}}"#;
        let response = ResponseParser::parse_stripe_error(json);
        assert_eq!(response.status, crate::models::response::PaymentStatus::CcnLive);
    }

    #[test]
    fn test_parse_stripe_insufficient_funds() {
        let json = r#"{"error": {"message": "Insufficient funds", "decline_code": "insufficient_funds"}}"#;
        let response = ResponseParser::parse_stripe_error(json);
        assert_eq!(response.status, crate::models::response::PaymentStatus::InsufficientFunds);
    }

    #[test]
    fn test_parse_stripe_declined() {
        let json = r#"{"error": {"message": "Your card was declined.", "code": "card_declined"}}"#;
        let response = ResponseParser::parse_stripe_error(json);
        assert_eq!(response.status, crate::models::response::PaymentStatus::Declined);
    }

    #[test]
    fn test_woocommerce_success() {
        let json = r#"{"result": "success", "redirect": "https://example.com/order-received"}"#;
        let response = ResponseParser::parse_woocommerce_response(json);
        assert_eq!(response.status, crate::models::response::PaymentStatus::Charged);
    }

    #[test]
    fn test_is_success_page() {
        let html = "<html><body><h1>Thank you for your order!</h1></body></html>";
        assert!(ResponseParser::is_success_page(html));
    }
}
