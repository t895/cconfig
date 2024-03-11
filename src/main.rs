mod setting;
mod config;

use config::Config;

fn main() -> Result<(), std::io::Error> {
    let file_path = String::from("test/stuff/foo.txt");
    let mut config = Config::new(&file_path, config::LineEnding::LF, true);

    let key = String::from("new-setting");
    let other_key = String::from("a-setting");
    let category = String::from("new-category");
    let my_category = String::from("a-category");
    let value = 20;
    config.add(&category, &key, &value);
    config.add(&my_category, &key, &value);
    config.add(&category, &other_key, &value);

    config.save();

    Ok(())
}
