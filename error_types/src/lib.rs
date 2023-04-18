pub use chrono::{Utc, NaiveDate};

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}
impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> FormError {
        FormError { 
            form_values: (field_name, field_value),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(), 
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub birth_location: String,
    pub password: String
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        birth_location: String,
        password: String,
    ) -> Form {
        Form {
            first_name,
            last_name,
            birth,
            birth_location,
            password,
        }
    }
    
    pub fn validate(&self) -> Result<Vec<&str>, FormError> {
        if self.first_name.is_empty() {
            return Err(FormError::new("first_name".to_string(), self.first_name.clone(), "No user name".to_string()));
        }   

        let mut has_alphabetic = false;
        let mut has_numeric = false;
        let mut has_non_alphanumeric = false;

        if self.password.chars().any(|c| c.is_alphabetic()) {
            has_alphabetic = true; 
        } else if self.password.chars().any(|c| c.is_numeric()) {
            has_numeric = true;
        } else if self.password.chars().any(|c| c.is_ascii_punctuation()) {
            has_non_alphanumeric = true;
        }

        if self.password.len() < 8 {
            return Err(FormError::new("password".to_string(), self.password.clone(), "At least 8 characters".to_string()));
        } else if !(has_alphabetic && has_numeric && has_non_alphanumeric) {
            return Err(FormError::new("password".to_string(), self.password.clone(), "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)".to_string()));
        }
        Ok(vec!["Valid first name", "Valid password"])
    }
}