use std::fmt::Formatter;

pub struct Setting {
    key: String,
    category: String,
    value: String,
}

impl Setting {
    pub fn new<T: std::fmt::Display>(key: String, category: String, value: T) -> Setting {
        let new_key: String = if key.contains("=") {
            let temp_key = key.clone();
            temp_key.replace("=", "")
        } else {
            key
        };

        Setting {
            key: new_key,
            category,
            value: value.to_string(),
        }
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }

    pub fn get_category(&self) -> String {
        self.category.clone()
    }

    pub fn set_value<T: std::fmt::Display>(&mut self, value: T) {
        self.value = value.to_string();
        self.value = self.value.trim().to_string();
    }

    pub fn get_value_string(&self) -> String {
        self.value.clone()
    }

    pub fn get_value<T: std::str::FromStr>(&self, default_value: T) -> T {
        self.value.parse::<T>().unwrap_or_else(|_| default_value)
    }
}

impl std::fmt::Display for Setting {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key - {}, Category - {}, Value - {}", self.key, self.category, self.value)
    }
}
