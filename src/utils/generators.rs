use rand::Rng;
use uuid::Uuid;

const FIRST_NAMES: &[&str] = &[
    "Michael", "Christopher", "Jessica", "Matthew", "Ashley", "Jennifer", 
    "Joshua", "Amanda", "Daniel", "David", "James", "Robert", "John", 
    "Joseph", "Andrew", "Ryan", "Brandon", "Jason", "Justin", "Sarah", 
    "William", "Jonathan", "Stephanie", "Brian", "Nicole", "Nicholas", 
    "Anthony", "Heather", "Eric", "Elizabeth", "Emily", "Olivia", "Sophia", 
    "Emma", "Ava", "Isabella", "Mia", "Abigail", "Madison", "Charlotte", 
    "Liam", "Noah", "Jacob", "Ethan", "Alexander", "Benjamin", "Lucas", "Henry"
];

const LAST_NAMES: &[&str] = &[
    "Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", 
    "Davis", "Rodriguez", "Martinez", "Hernandez", "Lopez", "Gonzalez", 
    "Wilson", "Anderson", "Thomas", "Taylor", "Moore", "Martin", "Jackson", 
    "Lee", "Perez", "Thompson", "White", "Harris", "Sanchez", "Clark", 
    "Ramirez", "Lewis", "Robinson", "Walker", "Young", "Allen", "King", 
    "Wright", "Scott", "Green", "Baker", "Adams", "Nelson", "Carter"
];

const EMAIL_DOMAINS: &[&str] = &[
    "gmail.com", "yahoo.com", "outlook.com", "hotmail.com", 
    "icloud.com", "protonmail.com"
];

/// Generate a random first name
pub fn generate_first_name() -> String {
    let mut rng = rand::thread_rng();
    FIRST_NAMES[rng.gen_range(0..FIRST_NAMES.len())].to_string()
}

/// Generate a random last name
pub fn generate_last_name() -> String {
    let mut rng = rand::thread_rng();
    LAST_NAMES[rng.gen_range(0..LAST_NAMES.len())].to_string()
}

/// Generate a random email address
pub fn generate_email(first_name: &str, last_name: &str) -> String {
    let mut rng = rand::thread_rng();
    let domain = EMAIL_DOMAINS[rng.gen_range(0..EMAIL_DOMAINS.len())];
    let separator = [".", "_", ""][rng.gen_range(0..3)];
    let number: u32 = rng.gen_range(100..9999);
    
    format!(
        "{}{}{}{}@{}",
        first_name.to_lowercase(),
        separator,
        last_name.to_lowercase(),
        number,
        domain
    )
}

/// Generate a random US phone number
pub fn generate_phone() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "{}{}{}",
        rng.gen_range(201..999),
        rng.gen_range(100..999),
        rng.gen_range(1000..9999)
    )
}

/// Generate a random password
pub fn generate_password(length: usize) -> String {
    use rand::distributions::Alphanumeric;
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

/// Generate a random string of specified length
pub fn generate_random_string(length: usize) -> String {
    use rand::distributions::Alphanumeric;
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(|c| c.to_ascii_lowercase())
        .map(char::from)
        .collect()
}

/// Generate a random hex string
pub fn generate_hex_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| format!("{:x}", rng.gen_range(0..16)))
        .collect()
}

/// Generate a GUID/UUID-like string
pub fn generate_guid() -> String {
    format!("guid_{}", generate_random_string(32))
}

/// Generate Stripe-like MID
pub fn generate_stripe_mid() -> String {
    format!("mid_{}", generate_random_string(32))
}

/// Generate Stripe-like SID
pub fn generate_stripe_sid() -> String {
    format!("sid_{}", generate_random_string(32))
}

/// Generate a complete user profile
pub struct UserProfile {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub full_name: String,
}

impl UserProfile {
    pub fn generate() -> Self {
        let first_name = generate_first_name();
        let last_name = generate_last_name();
        let email = generate_email(&first_name, &last_name);
        let phone = generate_phone();
        let full_name = format!("{} {}", first_name, last_name);

        Self {
            first_name,
            last_name,
            email,
            phone,
            full_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_names() {
        let first = generate_first_name();
        let last = generate_last_name();
        assert!(!first.is_empty());
        assert!(!last.is_empty());
    }

    #[test]
    fn test_generate_email() {
        let email = generate_email("John", "Doe");
        assert!(email.contains("@"));
        assert!(email.contains("john"));
        assert!(email.contains("doe"));
    }

    #[test]
    fn test_generate_phone() {
        let phone = generate_phone();
        assert_eq!(phone.len(), 10);
        assert!(phone.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_user_profile() {
        let profile = UserProfile::generate();
        assert!(!profile.first_name.is_empty());
        assert!(!profile.last_name.is_empty());
        assert!(profile.email.contains("@"));
        assert_eq!(profile.phone.len(), 10);
    }
}
