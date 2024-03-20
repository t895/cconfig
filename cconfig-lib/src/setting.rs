use std::fmt::{Formatter, Display};
use std::str::FromStr;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Setting {
    category: String,
    key: String,
    value: String,
}

impl Setting {
    /// Creates a new Setting object. Copies key and category String data and serializes the value to a String.
    pub fn new<T: Display + FromStr>(key: &String, category: &String, value: &T) -> Setting {
        Self {
            category: category.clone(),
            key: key.clone(),
            value: value.to_string(),
        }
    }

    /// Retrieves a reference to this setting's key.
    pub fn get_key(&self) -> &String {
        &self.key
    }

    // Retrieves a reference to this setting's category.
    pub fn get_category(&self) -> &String {
        &self.category
    }

    /// Copies a String serialized value to this setting's internal value.
    pub fn set_value<T: Display + FromStr>(&mut self, value: T) {
        self.value = value.to_string()
    }

    /// Retrieves a reference to this setting's internal String serialized value.
    pub fn get_value_string(&self) -> &String {
        &self.value
    }

    /// Retrieves a newly allocated object from this setting's value interpreted as T.
    pub fn get_value<T: Display + FromStr>(&self) -> Option<T> {
        match self.value.parse::<T>() {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }
}

impl std::fmt::Display for Setting {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Category - {}, Key - {}, Value - {}", &self.key, &self.category, &self.value)
    }
}
