use std::fmt::Formatter;

pub struct Setting {
    key: String,
    category: String,
    value: String,
}

impl Setting {
    pub fn new<T: std::fmt::Display>(key: &String, category: &String, value: &T) -> Setting {
        let new_key: String = if key.contains("=") {
            let temp_key = key.clone();
            temp_key.replace("=", "")
        } else {
            key.clone()
        };

        Setting {
            key: new_key,
            category: category.clone(),
            value: value.to_string(),
        }
    }

    pub fn get_key(&self) -> &String {
        &self.key
    }

    pub fn get_category(&self) -> &String {
        &self.category
    }

    pub fn set_value<T: std::fmt::Display>(&mut self, value: T) {
        self.value = value.to_string()
    }

    pub fn get_value_string(&self) -> &String {
        &self.value
    }

    pub fn get_value<T: std::str::FromStr>(&self, default_value: T) -> T {
        self.value.parse::<T>().unwrap_or(default_value)
    }
}

impl std::fmt::Display for Setting {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Category - {}, Key - {}, Value - {}", &self.key, &self.category, &self.value)
    }
}
