trait Setting {
    fn get_key(&self) -> String;
    fn get_category(&self) -> String;
}

struct StringSetting {
    key: String,
    category: String,
    value: String,
}

impl StringSetting {
    fn get_string(&self) -> String {
        self.value.clone()
    }

    fn set_string(&self, value: String) {
        self.value = value;
    }
}

impl Setting for StringSetting {
    fn get_key(&self) -> String {
        self.key.clone()
    }

    fn get_category(&self) -> String {
        self.category.clone()
    }
}
