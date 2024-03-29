use std::collections::HashMap;
use std::io::{Read, Write};
use std::fmt::Display;
use std::str::FromStr;

use crate::line_ending::LineEnding;
use crate::setting::Setting;

pub struct Config {
    file_path: String,
    line_ending: LineEnding,
    padding: bool,
    settings: HashMap<String, Setting>,
}

impl Config {
    const TAG: &'static str = "[cconfig]";

    const KEY_VALUE_SEPARATOR: char = '=';
    const CATEGORY_START: char = '[';
    const CATEGORY_END: char = ']';

    /// Creates a new Config object.
    pub fn new(file_path: &String, line_ending: LineEnding, padding: bool) -> Config {
        let settings = Self::load(file_path, None);
        Self {
            file_path: file_path.clone(),
            line_ending,
            padding,
            settings,
        }
    }

    /// Serializes and saves all settings stored in Config's internal HashMap to disk.
    pub fn save(&self) {
        let mut sorted_settings: Vec<&Setting> = Vec::<&Setting>::new();
        for setting in self.settings.iter() {
            sorted_settings.push(setting.1);
        }

        let line_ending = match &self.line_ending {
            LineEnding::LF => String::from("\n"),
            LineEnding::CRLF => String::from("\r\n"),
            LineEnding::CR => String::from("\r"),
        };

        let mut settings_string = String::from("");
        let mut padding = String::from("");
        if self.padding {
            padding.push(' ')
        }

        sorted_settings.sort_by(|a, b| a.cmp(b));
        let mut current_category = String::from("");
        for setting in sorted_settings {
            if current_category != *setting.get_category().replace("=", "") {
                if !current_category.is_empty() {
                    settings_string.push_str(&line_ending);
                }

                current_category.clear();
                current_category.push_str(setting.get_category());
                settings_string.push_str(&format!("{}{}{}{line_ending}", Self::CATEGORY_START, current_category, Self::CATEGORY_END));
            }

            let formatted_key = setting.get_key().replace("=", "");
            let formatted_value = setting.get_key().replace("=", "");
            settings_string.push_str(&format!("{}{padding}{}{padding}{}{line_ending}", formatted_key, Self::KEY_VALUE_SEPARATOR, formatted_value));
        }

        println!("{} Opening file {} for saving", Self::TAG, &self.file_path);
        let mut file = match Self::open_or_create(&self.file_path, true) {
            Ok(value) => value,
            Err(_) => {
                println!("{} Failed to open config file for saving!", Self::TAG);
                return;
            },
        };

        match file.write_all(settings_string.as_bytes()) {
            Ok(_) => println!("{} Successfully wrote settings to config file.", Self::TAG),
            Err(e) => println!("{} Failed to write settings to config file! - {}", Self::TAG, e),
        }
    }

    /// Clears Config's internal HashMap and reloads the settings from disk.
    pub fn reload(&mut self) {
        let previous_size = Some(self.settings.len());
        self.settings.clear();
        self.settings = Self::load(&self.file_path, previous_size);
    }

    /// Gets a reference to a given setting from the Config's internal HashMap and creates a new setting if it doesn't exist.
    pub fn get<T: Display + FromStr>(&mut self, category: &String, key: &String, default_value: &T) -> &Setting {
        if !self.has_setting(category, key) {
            self.add(category, key, default_value);
        }
        self.settings.get(&Self::get_setting_key(&category, &key)).unwrap()
    }

    /// Gets a mutable reference to a given setting from the Config's internal HashMap and creates a new setting if it doesn't exist.
    pub fn get_mut<T: Display + FromStr>(&mut self, category: &String, key: &String, default_value: &T) -> &mut Setting {
        if !self.has_setting(category, key) {
            self.add(category, key, default_value);
        }
        self.settings.get_mut(&Self::get_setting_key(&category, &key)).unwrap()
    }

    /// Assembles a new Setting to add to the Config's internal HashMap.
    pub fn add<T: Display + FromStr>(&mut self, category: &String, key: &String, value: &T) {
        self.settings.insert(Self::get_setting_key(&category, &key), Setting::new(key, category, value));
    }

    /// Adds a Setting to the Config's internal HashMap.
    pub fn add_setting(&mut self, setting: Setting) {
        self.settings.insert(Self::get_setting_key(setting.get_category(), setting.get_key()), setting);
    }

    /// Removes a setting from the Config's internal HashMap.
    pub fn remove(&mut self, category: &String, key: &String) -> Option<Setting> {
        self.settings.remove(&Self::get_setting_key(&category, &key))
    }

    /// Checks the Config's internal HashMap if it contains a given setting.
    pub fn has_setting(&self, category: &String, key: &String) -> bool {
        self.settings.contains_key(&Self::get_setting_key(category, key))
    }

    /// Tries to open an existing file and creates one if it doesn't exist.
    fn open_or_create(file_path: &String, write: bool) -> Result<std::fs::File, std::io::Error> {
        let mut file_dir: &str = &file_path;
        match file_path.rfind('/') {
            Some(unix_pos) => {
                file_dir = &file_path[0..unix_pos];
            }
            None => {
                match file_path.rfind('\\') {
                    Some (win_pos) => {
                        file_dir = &file_path[0..win_pos];
                    }
                    None => { /* No-op */ },
                }
            }
        }

        match std::fs::create_dir_all(file_dir) {
            Ok(_) => println!("{} Successfully created directories to config file.", Self::TAG),
            Err(e) => {
                println!("{} Failed to create directories to config file! - {}", Self::TAG, e);
                return Err(e);
            }
        }

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

    /// Gets the key used to insert/find a setting in the Config's internal HashMap
    fn get_setting_key(category: &String, key: &String) -> String {
        format!("{}{}", category, key)
    }

    /// Reads the file from file_path and creates a new HashMap of settings with key-value pairs.
    fn load(file_path: &String, previous_size: Option<usize>) -> HashMap<String, Setting> {
        let mut settings = HashMap::new();
        if previous_size.is_some() {
            settings.reserve(previous_size.unwrap());
        }

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
                    // Typically, this only fails when creating a new file, so it's best not to alert the user here.
                    // TODO: Use a debug logging crate
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
            let first_char = match chars.next() {
                Some(value) => value,
                None => {
                    // Don't warn user about failing to read this line. It's most likely empty.
                    line_number += 1;
                    continue
                },
            };
            let last_char = match chars.last() {
                Some(value) => value,
                None => {
                    println!("{} Failed to parse last character on line {}, skipping!", Self::TAG, line_number);
                    line_number += 1;
                    continue
                },
            };

            if first_char == Self::CATEGORY_START && last_char == Self::CATEGORY_END {
                category = line[1..line.len() - 1].to_string();
                line_number += 1;
                continue;
            }

            let key_value: Vec<&str> = line.split(Self::KEY_VALUE_SEPARATOR).collect();
            if key_value.len() != 2 {
                println!("{} Invalid key-value pair on line {}, skipping!", Self::TAG, line_number);
                line_number += 1;
                continue;
            }

            let key = match key_value.first() {
                Some(value) => value.trim().to_string(),
                None => {
                    println!("{} Failed to parse key on line {}, skipping!", Self::TAG, line_number);
                    line_number += 1;
                    continue
                },
            };
            let value = match key_value.last() {
                Some(value) => value.trim().to_string(),
                None => {
                    println!("{} Failed to parse value on line {}, skipping!", Self::TAG, line_number);
                    line_number += 1;
                    continue
                },
            };

            settings.insert(Self::get_setting_key(&category, &key), Setting::new(&category, &key, &value));
            line_number += 1;
        }

        settings
    }
}
