mod setting;
mod config;

use config::Config;

fn main() -> Result<(), std::io::Error> {
    let file_path = String::from("test/foo.txt");
    let mut config = Config::new(&file_path, config::LineEnding::LF, true);

    let mut value: u128 = 20;
    for i in 0..1000 {
        let category = format!("this-category-{}", i);
        for j in 0..1000 {
            let key = format!("this-key-{}", j);
            config.add(&category, &key, &value);
            value += j;
        }
    }

    for i in 0..1000 {
        let category = format!("this-category-{}", i);
        for j in 0..1000 {
            let key = format!("this-key-{}", j);
            let setting = config.get_mut(&category, &key).unwrap();
            let value = setting.get_value(1u128) + 1;
            setting.set_value(value);
        }
    }

    config.save();
    config.reload();

    Ok(())
}
