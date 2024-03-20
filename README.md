# cconfig
Basic config reader/writer written in Rust.

## Usage
This library provides a Setting struct to hold individual setting data and a Config struct that manages them and a corresponding file. Simply create a Config instance and add all of your settings to it. 
```rust
use cconfig::setting::Setting;
use cconfig::config::Config;
use cconfig::line_ending::LineEnding;

fn main() {
    // Create a new setting
    let key = String::from("my_key");
    let category = String::from("my_category");

    // A Setting's value type must implement the traits std::format::Display
    // and std::str::FromStr for serialization and deserialization respectively
    let value = 123u8;
    let my_setting = Setting::new(&category, &key, &value);

    // Relative or absolute path
    let path = String::from("foo.txt");

    // Load your config
    let config = Config::new(&path, LineEnding::LF, true);

    // Add a setting
    config.add_setting(&my_setting);

    // Add a setting with loose data
    config.add(&category, &key, &value);

    // Save your config
    config.save();
}
```

This is mostly an abstraction on a HashMap so there are some additional methods exposed through the Config impl to get information about your settings.
```rust
use cconfig::config::Config;
use cconfig::setting::Setting;

fn main() {
    let path = String::from("foo.txt");
    let config = Config::new(&path, LineEnding::LF, true);

    let key = String::from("my_key");
    let category = String::from("my_category");
    let default_value = 123u8;

    let my_mutable_setting = config.get_mut(&category, &key, &default_value);
    let new_value = 124u8;
    my_setting.set_value(new_value);

    let my_immutable_setting = config.get(&category, &key, &default_value);
    let deserialized_value = my_immutable_setting.get_value_or(default_value);

    let removed_setting = match config.remove(&key, &category) {
        Some(value) => value, // Removed setting value
        None => return, // Setting was not in HashMap
    };
}
```
