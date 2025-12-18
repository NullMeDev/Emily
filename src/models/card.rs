use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a credit card with all necessary details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub number: String,
    pub month: String,
    pub year: String,
    pub cvv: String,
}

impl Card {
    /// Parse card from string format: NUMBER|MM|YY|CVV
    pub fn from_string(input: &str) -> Result<Self, CardError> {
        let parts: Vec<&str> = input.trim().split('|').collect();
        
        if parts.len() != 4 {
            return Err(CardError::InvalidFormat(
                "Expected format: NUMBER|MM|YY|CVV".to_string()
            ));
        }

        let number = parts[0].replace(" ", "").replace("-", "");
        let month = parts[1].trim().to_string();
        let mut year = parts[2].trim().to_string();
        let cvv = parts[3].trim().to_string();

        // Validate number is digits only
        if !number.chars().all(|c| c.is_ascii_digit()) {
            return Err(CardError::InvalidNumber);
        }

        // Validate number length (13-19 digits)
        if number.len() < 13 || number.len() > 19 {
            return Err(CardError::InvalidNumber);
        }

        // Validate month
        if !month.chars().all(|c| c.is_ascii_digit()) {
            return Err(CardError::InvalidMonth);
        }
        let month_num: u8 = month.parse().map_err(|_| CardError::InvalidMonth)?;
        if month_num < 1 || month_num > 12 {
            return Err(CardError::InvalidMonth);
        }
        let month = format!("{:02}", month_num);

        // Validate and normalize year
        if !year.chars().all(|c| c.is_ascii_digit()) {
            return Err(CardError::InvalidYear);
        }
        if year.len() == 4 {
            if year.starts_with("20") {
                year = year[2..].to_string();
            } else {
                return Err(CardError::InvalidYear);
            }
        } else if year.len() != 2 {
            return Err(CardError::InvalidYear);
        }

        // Validate CVV
        if !cvv.chars().all(|c| c.is_ascii_digit()) {
            return Err(CardError::InvalidCvv);
        }
        if cvv.len() < 3 || cvv.len() > 4 {
            return Err(CardError::InvalidCvv);
        }

        Ok(Card {
            number,
            month,
            year,
            cvv,
        })
    }

    /// Get the BIN (first 6 digits)
    pub fn bin(&self) -> String {
        self.number.chars().take(6).collect()
    }

    /// Get the last 4 digits
    pub fn last4(&self) -> String {
        let len = self.number.len();
        if len >= 4 {
            self.number.chars().skip(len - 4).collect()
        } else {
            self.number.clone()
        }
    }

    /// Get masked card number (e.g., 4532********1234)
    pub fn masked(&self) -> String {
        let len = self.number.len();
        if len <= 8 {
            return self.number.clone();
        }
        let first4: String = self.number.chars().take(4).collect();
        let last4 = self.last4();
        let stars = "*".repeat(len - 8);
        format!("{}{}{}", first4, stars, last4)
    }

    /// Format as original string: NUMBER|MM|YY|CVV
    pub fn to_string(&self) -> String {
        format!("{}|{}|{}|{}", self.number, self.month, self.year, self.cvv)
    }

    /// Get full year (20YY format)
    pub fn full_year(&self) -> String {
        format!("20{}", self.year)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}|{}|{}|{}", self.number, self.month, self.year, self.cvv)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CardError {
    #[error("Invalid card format: {0}")]
    InvalidFormat(String),
    
    #[error("Invalid card number")]
    InvalidNumber,
    
    #[error("Invalid expiration month")]
    InvalidMonth,
    
    #[error("Invalid expiration year")]
    InvalidYear,
    
    #[error("Invalid CVV")]
    InvalidCvv,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_card() {
        let card = Card::from_string("4532123456789012|12|25|123").unwrap();
        assert_eq!(card.number, "4532123456789012");
        assert_eq!(card.month, "12");
        assert_eq!(card.year, "25");
        assert_eq!(card.cvv, "123");
    }

    #[test]
    fn test_parse_with_4digit_year() {
        let card = Card::from_string("4532123456789012|6|2025|123").unwrap();
        assert_eq!(card.year, "25");
        assert_eq!(card.month, "06");
    }

    #[test]
    fn test_bin() {
        let card = Card::from_string("4532123456789012|12|25|123").unwrap();
        assert_eq!(card.bin(), "453212");
    }

    #[test]
    fn test_masked() {
        let card = Card::from_string("4532123456789012|12|25|123").unwrap();
        assert_eq!(card.masked(), "4532********9012");
    }

    #[test]
    fn test_invalid_format() {
        assert!(Card::from_string("4532123456789012|12|25").is_err());
    }

    #[test]
    fn test_invalid_month() {
        assert!(Card::from_string("4532123456789012|13|25|123").is_err());
    }
}
