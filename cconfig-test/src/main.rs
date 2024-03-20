use cconfig::config::Config;
use cconfig::setting::Setting;
use cconfig::line_ending::LineEnding;

fn main() {
    let file_path = String::from("test/foo.txt");
    let mut config = Config::new(&file_path, LineEnding::LF, true);

    let mut value: u128 = 20;
    for i in 0..1000 {
        let category = format!("this-category-{}", i);
        for j in 0..1000 {
            let key = format!("this-key-{}", j);
            config.add(&category, &key, &value);
            value += j;
        }
    }

    let default_value = 1u128;
    for i in 0..1000 {
        let category = format!("this-category-{}", i);
        for j in 0..1000 {
            let key = format!("this-key-{}", j);
            let setting = config.get_mut(&category, &key, &default_value);
            let value = setting.get_value().unwrap_or(default_value) + 1;
            setting.set_value(value);
        }
    }

    config.save();
    config.reload();
}
