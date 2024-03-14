use std::fmt::{Formatter, Display};
use std::str::FromStr;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Setting {
    category: String,
    key: String,
    value: String,
}

impl Setting {
    pub fn new<T: Display + FromStr>(key: &String, category: &String, value: &T) -> Setting {
        let new_key = key.replace('=', "");
        Self {
            category: category.clone(),
            key: new_key,
            value: value.to_string(),
        }
    }

    pub fn get_key(&self) -> &String {
        &self.key
    }

    pub fn get_category(&self) -> &String {
        &self.category
    }

    pub fn set_value<T: Display + FromStr>(&mut self, value: T) {
        self.value = value.to_string()
    }

    pub fn get_value_string(&self) -> &String {
        &self.value
    }

    pub fn get_value<T: Display + FromStr>(&self, default_value: T) -> T {
        self.value.parse::<T>().unwrap_or(default_value)
    }
}

impl std::fmt::Display for Setting {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Category - {}, Key - {}, Value - {}", &self.key, &self.category, &self.value)
    }
}
