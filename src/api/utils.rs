use regex::Regex;

pub fn validate_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
    email_regex.is_match(&email)
}

#[cfg(test)]
mod tests {
    use crate::api::utils::validate_email;

    #[test]
    fn test_valid_email_format() {
        assert_eq!(validate_email("foo@bar.com"), true);
    }

    #[test]
    fn test_invalid_email_format() {
        assert_eq!(validate_email("invalid email"), false);
    }
}