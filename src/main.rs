mod settings;
mod config;

use config::Config;

fn main() -> Result<(), std::io::Error> {
    let file_path = String::from("foo.txt");
    let mut config = Config::new(&file_path, config::LineEnding::LF, true);
    config.save();

    let key = String::from("new-setting");
    let category = String::from("new-category");
    config.remove(&category, &key);

    config.save();

    Ok(())
}
