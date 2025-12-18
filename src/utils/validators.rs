/// Validate card number using Luhn algorithm
pub fn validate_luhn(card_number: &str) -> bool {
    let digits: Vec<u32> = card_number
        .chars()
        .filter(|c| c.is_ascii_digit())
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.is_empty() {
        return false;
    }

    let mut sum = 0;
    let mut double = false;

    // Process digits from right to left
    for &digit in digits.iter().rev() {
        let mut value = digit;
        
        if double {
            value *= 2;
            if value > 9 {
                value -= 9;
            }
        }
        
        sum += value;
        double = !double;
    }

    sum % 10 == 0
}

/// Calculate Luhn check digit for a partial card number
pub fn calculate_luhn_digit(partial: &str) -> Option<u32> {
    let digits: Vec<u32> = partial
        .chars()
        .filter(|c| c.is_ascii_digit())
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.is_empty() {
        return None;
    }

    let mut sum = 0;
    let mut double = true; // Start with true because we're adding a digit at the end

    // Process existing digits from right to left
    for &digit in digits.iter().rev() {
        let mut value = digit;
        
        if double {
            value *= 2;
            if value > 9 {
                value -= 9;
            }
        }
        
        sum += value;
        double = !double;
    }

    // Calculate check digit
    let check_digit = (sum * 9) % 10;
    Some(check_digit)
}

/// Detect card brand from BIN
pub fn detect_card_brand(bin: &str) -> &'static str {
    if bin.is_empty() {
        return "Unknown";
    }

    let first_digit = bin.chars().next().unwrap();
    let first_two: String = bin.chars().take(2).collect();
    let first_four: String = bin.chars().take(4).collect();

    // Visa
    if first_digit == '4' {
        return "Visa";
    }

    // Mastercard
    if let Ok(num) = first_two.parse::<u32>() {
        if (51..=55).contains(&num) || (2221..=2720).contains(&num) {
            return "Mastercard";
        }
    }

    // American Express
    if first_two == "34" || first_two == "37" {
        return "American Express";
    }

    // Discover
    if first_two == "65" || first_four == "6011" {
        return "Discover";
    }

    // JCB
    if let Ok(num) = first_four.parse::<u32>() {
        if (3528..=3589).contains(&num) {
            return "JCB";
        }
    }

    // Diners Club
    if first_two == "36" || first_two == "38" || first_two == "30" {
        return "Diners Club";
    }

    // UnionPay
    if first_two == "62" {
        return "UnionPay";
    }

    "Unknown"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_luhn_valid() {
        // Valid test card numbers
        assert!(validate_luhn("4532015112830366")); // Visa
        assert!(validate_luhn("5425233430109903")); // Mastercard
        assert!(validate_luhn("374245455400126"));  // Amex
    }

    #[test]
    fn test_luhn_invalid() {
        assert!(!validate_luhn("4532015112830367")); // Wrong check digit
        assert!(!validate_luhn("1234567890123456")); // Invalid
    }

    #[test]
    fn test_calculate_luhn() {
        assert_eq!(calculate_luhn_digit("453201511283036"), Some(6));
        assert_eq!(calculate_luhn_digit("542523343010990"), Some(3));
    }

    #[test]
    fn test_detect_brand() {
        assert_eq!(detect_card_brand("4532"), "Visa");
        assert_eq!(detect_card_brand("5425"), "Mastercard");
        assert_eq!(detect_card_brand("3742"), "American Express");
        assert_eq!(detect_card_brand("6011"), "Discover");
    }
}
