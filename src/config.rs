use std::io::{Read, Write};

use crate::settings::Setting;

pub enum LineEnding {
    LF,
    CRLF,
    CR,
}

pub struct Config {
    file_path: String,
    line_ending: LineEnding,
    padding: bool,
    settings: std::collections::HashMap<String, Setting>,
}

impl Config {
    const TAG: &'static str = "[rs-config]";

    const KEY_VALUE_SEPARATOR: char = '=';
    const CATEGORY_START: char = '[';
    const CATEGORY_END: char = ']';

    pub fn new(file_path: &String, line_ending: LineEnding, padding: bool) -> Config {
        let settings = Self::load(file_path);
        Config {
            file_path: file_path.clone(),
            line_ending,
            padding,
            settings,
        }
    }

    pub fn save(&self) {
        let mut categories: std::collections::HashMap<String, Vec<&Setting>> = std::collections::HashMap::new();
        for setting in self.settings.iter() {
            match categories.get_mut(&setting.1.get_category()) {
                Some(vec) => vec.push(setting.1),
                None => {
                    let mut settings = Vec::<&Setting>::new();
                    settings.push(setting.1);
                    categories.insert(setting.1.get_category(), settings);
                },
            }
        }

        let line_ending = match &self.line_ending {
            LineEnding::LF => String::from("\n"),
            LineEnding::CRLF => String::from("\r\n"),
            LineEnding::CR => String::from("\r"),
        };

        let mut settings_string = String::from("");
        let padding = if self.padding {
            String::from(" ")
        } else {
            String::from("")
        };

        // TODO: Sort categories alphabetically - current order is non-deterministic
        for category in categories {
            let mut category_string = format!("{}{}{}{}", Self::CATEGORY_START, category.0, Self::CATEGORY_END, line_ending);
            for setting in category.1 {
                category_string.push_str(format!("{}{}{}{}{}{}", setting.get_key(), padding, Self::KEY_VALUE_SEPARATOR, padding, setting.get_value_string(), line_ending).as_str());
            }
            category_string.push_str(&line_ending);
            settings_string.push_str(&category_string);
        }

        println!("{} Opening file {} for saving", Self::TAG, &self.file_path);
        let mut file = match Self::open_or_create(&self.file_path, true) {
            Ok(value) => value,
            Err(_) => {
                println!("{} Failed to open config file for saving!", Self::TAG);
                return;
            },
        };
        println!("{} Settings data to be written -\n{}", Self::TAG, &settings_string);

        match file.write_all(settings_string.as_bytes()) {
            Ok(_) => println!("{} Successfully wrote settings to config file.", Self::TAG),
            Err(e) => println!("{} Failed to write settings to config file! - {}", Self::TAG, e),
        }
    }

    pub fn reload(&mut self) {
        self.settings = Self::load(&self.file_path)
    }

    pub fn get(&self, category: &String, key: &String) -> Option<&Setting> {
        self.settings.get(&Self::get_setting_key(&category, &key))
    }

    pub fn add<T: std::fmt::Display>(&mut self, key: String, category: String, value: T) {
        self.settings.insert(Self::get_setting_key(&category, &key), Setting::new(key, category, value));
    }

    pub fn remove(&mut self, category: &String, key: &String) -> Option<Setting> {
        self.settings.remove(&Self::get_setting_key(&category, &key))
    }

    fn open_or_create(file_path: &String, write: bool) -> Result<std::fs::File, std::io::Error> {
        if write {
            std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .append(false)
                .truncate(true)
                .open(file_path)
        } else {
            let file_res = std::fs::File::open(file_path);
            match &file_res {
                Ok(_) => file_res,
                Err(_) => {
                    println!("{} Config file not found, creating new file!", Self::TAG);
                    std::fs::File::create(file_path)
                }
            }
        }
    }

    fn get_setting_key(category: &String, key: &String) -> String {
        format!("{}{}", category, key)
    }

    fn load(file_path: &String) -> std::collections::HashMap<String, Setting> {
        let mut settings = std::collections::HashMap::new();
        let mut settings_file_string = String::from("");
        {
            let mut settings_file = match Self::open_or_create(file_path, false) {
                Ok(value) => value,
                Err(e) => {
                    println!("{} Failed to open/create config file! - {}", Self::TAG, e);
                    return settings;
                },
            };

            let _settings_string_res = match settings_file.read_to_string(&mut settings_file_string) {
                Ok(value) => value,
                Err(_) => {
                    println!("{} Failed to read config file to string!", Self::TAG);
                    return settings
                },
            };
        }

        let lines = if settings_file_string.contains("\r\n") {
            settings_file_string.split("\r\n")
        } else if settings_file_string.contains('\r') {
            settings_file_string.split("\r")
        } else {
            settings_file_string.split("\n")
        };

        let mut line_number = 1;
        let mut category = String::from("");
        for line in lines {
            let mut chars = line.chars();
            let first_char = match chars.nth(0) {
                Some(value) => value,
                None => continue,
            };
            let last_char = match chars.last() {
                Some(value) => value,
                None => continue,
            };

            if first_char == Self::CATEGORY_START && last_char == Self::CATEGORY_END {
                category = line[1..line.len() - 1].to_string();
                continue;
            }

            let key_value: Vec<&str> = line.split(Self::KEY_VALUE_SEPARATOR).collect();
            if key_value.len() != 2 {
                println!("{} Invalid key-value pair on line {}", Self::TAG, line_number);
                continue;
            }

            let key = match key_value.first() {
                Some(value) => value.trim().to_string(),
                None => continue,
            };
            let value = match key_value.last() {
                Some(value) => value.trim().to_string(),
                None => continue,
            };

            settings.insert(Self::get_setting_key(&category, &key), Setting::new(key, category.clone(), value));
            line_number += 1;
        }

        settings
    }
}
