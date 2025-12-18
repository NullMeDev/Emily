use serde::{Deserialize, Serialize};
use std::fmt;

/// Payment response status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentStatus {
    /// Payment was successful/charged
    Charged,
    /// Card is live but CVV/CVC is incorrect
    CcnLive,
    /// 3D Secure or additional action required
    ThreeDsRequired,
    /// Insufficient funds on the card
    InsufficientFunds,
    /// Card was declined
    Declined,
    /// Gateway or script error
    Error,
    /// Unknown response
    Unknown,
}

impl PaymentStatus {
    /// Check if this status should be sent to Telegram (successful hits only)
    pub fn should_notify(&self) -> bool {
        matches!(
            self,
            PaymentStatus::Charged
                | PaymentStatus::CcnLive
                | PaymentStatus::ThreeDsRequired
                | PaymentStatus::InsufficientFunds
        )
    }

    /// Get emoji icon for this status
    pub fn icon(&self) -> &'static str {
        match self {
            PaymentStatus::Charged => "✅",
            PaymentStatus::CcnLive => "✅",
            PaymentStatus::ThreeDsRequired => "🔑",
            PaymentStatus::InsufficientFunds => "💰",
            PaymentStatus::Declined => "❌",
            PaymentStatus::Error => "⚠️",
            PaymentStatus::Unknown => "❓",
        }
    }

    /// Get display title for this status
    pub fn title(&self) -> &'static str {
        match self {
            PaymentStatus::Charged => "Charged Successfully",
            PaymentStatus::CcnLive => "CCN Live",
            PaymentStatus::ThreeDsRequired => "3DS/Action Required",
            PaymentStatus::InsufficientFunds => "Insufficient Funds",
            PaymentStatus::Declined => "Card Declined",
            PaymentStatus::Error => "Script Error",
            PaymentStatus::Unknown => "Unknown Response",
        }
    }
}

impl fmt::Display for PaymentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.title())
    }
}

/// Complete payment response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResponse {
    /// Status of the payment
    pub status: PaymentStatus,
    /// Detailed message
    pub message: String,
    /// Optional redirect URL (for successful charges)
    pub redirect_url: Option<String>,
    /// Raw response snippet for debugging
    pub raw_response: Option<String>,
    /// Execution time in seconds
    pub execution_time: f64,
}

impl PaymentResponse {
    pub fn new(status: PaymentStatus, message: String) -> Self {
        Self {
            status,
            message,
            redirect_url: None,
            raw_response: None,
            execution_time: 0.0,
        }
    }

    pub fn charged(message: String) -> Self {
        Self::new(PaymentStatus::Charged, message)
    }

    pub fn ccn_live(message: String) -> Self {
        Self::new(PaymentStatus::CcnLive, message)
    }

    pub fn three_ds() -> Self {
        Self::new(
            PaymentStatus::ThreeDsRequired,
            "3D Secure authentication required".to_string(),
        )
    }

    pub fn insufficient_funds() -> Self {
        Self::new(
            PaymentStatus::InsufficientFunds,
            "Insufficient funds".to_string(),
        )
    }

    pub fn declined(message: String) -> Self {
        Self::new(PaymentStatus::Declined, message)
    }

    pub fn error(message: String) -> Self {
        Self::new(PaymentStatus::Error, message)
    }

    pub fn unknown(message: String) -> Self {
        Self::new(PaymentStatus::Unknown, message)
    }

    pub fn with_redirect(mut self, url: String) -> Self {
        self.redirect_url = Some(url);
        self
    }

    pub fn with_raw(mut self, raw: String) -> Self {
        self.raw_response = Some(raw);
        self
    }

    pub fn with_time(mut self, time: f64) -> Self {
        self.execution_time = time;
        self
    }
}

/// BIN information from lookup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinInfo {
    pub bin: String,
    pub brand: String,
    pub card_type: String,
    pub bank: String,
    pub country: String,
    pub country_flag: String,
}

impl Default for BinInfo {
    fn default() -> Self {
        Self {
            bin: "N/A".to_string(),
            brand: "N/A".to_string(),
            card_type: "N/A".to_string(),
            bank: "N/A".to_string(),
            country: "N/A".to_string(),
            country_flag: "🏳️".to_string(),
        }
    }
}

impl BinInfo {
    pub fn new(bin: String) -> Self {
        Self {
            bin,
            ..Default::default()
        }
    }
}
