use std::collections::HashMap;

fn open_or_create(file_path: &String) -> Result<std::fs::File, std::io::Error> {
    let create_res = std::fs::File::create(file_path);
    match create_res {
        Ok(_) => create_res,
        Err(_) => {
            std::fs::File::open(file_path)
        }
    }
}

fn create_ini_string<T: std::fmt::Display>(settings_map: HashMap<String, &dyn Setting>) -> String {
    String::from("TODO")
}

fn main() -> Result<(), std::io::Error> {
    let key1 = String::from("my_key");
    let value1 = 32;
    let category1: String = String::from("my_category");
    let my_setting_1 = StringSetting {
        key: key1,
        category: category1,
        value: value1,
    };

    let key2 = String::from("my_key");
    let value2: u128 = 128;
    let category2: String = String::from("my_category");
    let my_setting_2 = StringSetting {
        key: key2,
        value: value2,
        category: category2,
    };

    let mut settings_map: HashMap<String, &Setting> = HashMap::new();
    settings_map.insert(my_setting_1.key, &my_setting_1);
    settings_map.insert(my_setting_2.key, &my_setting_2);

    // let path = String::from("foo.txt");
    // let mut file = open_or_create(&path)?;

    // let file_data = create_ini_string(settings_map);
    // file.write_all(file_data.as_bytes())?;

    Ok(())
}
