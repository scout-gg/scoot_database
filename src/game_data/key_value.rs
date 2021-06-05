use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Ao2KeyValues {
    pub en: HashMap<i32, String>,
    pub fr: HashMap<i32, String>,
    pub br: HashMap<i32, String>,
    pub de: HashMap<i32, String>,
    pub es: HashMap<i32, String>,
    pub hi: HashMap<i32, String>,
    pub it: HashMap<i32, String>,
    pub jp: HashMap<i32, String>,
    pub ko: HashMap<i32, String>,
    pub ms: HashMap<i32, String>,
    pub mx: HashMap<i32, String>,
    pub ru: HashMap<i32, String>,
    pub tr: HashMap<i32, String>,
    pub tw: HashMap<i32, String>,
    pub vi: HashMap<i32, String>,
    pub zh: HashMap<i32, String>,
}

impl Ao2KeyValues {
    pub fn create() -> Self {
        Ao2KeyValues {
            en: Ao2KeyValues::load("en"),
            fr: Ao2KeyValues::load("fr"),
            br: Ao2KeyValues::load("br"),
            de: Ao2KeyValues::load("de"),
            es: Ao2KeyValues::load("es"),
            hi: Ao2KeyValues::load("hi"),
            it: Ao2KeyValues::load("it"),
            jp: Ao2KeyValues::load("jp"),
            ko: Ao2KeyValues::load("ko"),
            ms: Ao2KeyValues::load("ms"),
            mx: Ao2KeyValues::load("mx"),
            ru: Ao2KeyValues::load("ru"),
            tr: Ao2KeyValues::load("tr"),
            tw: Ao2KeyValues::load("tw"),
            vi: Ao2KeyValues::load("vi"),
            zh: Ao2KeyValues::load("zh"),
        }
    }

    fn load(lang: &str) -> HashMap<i32, String> {
        let mut key_value = HashMap::new();
        let path = &format!("resources/keyvalues/{}", lang);
        let key_value_file = File::open(path).unwrap();
        let buf_reader = BufReader::new(key_value_file);
        buf_reader.lines().for_each(|line| {
            let line = line.unwrap();
            if !line.starts_with("//") {
                let split_idx = line.find(' ');
                if let Some(idx) = split_idx {
                    let key = line[..idx].parse::<i32>();
                    if let Ok(key) = key {
                        let value = line[idx..].to_string();
                        let value = value.trim_start();
                        let value = value.replace("\"", "");
                        key_value.insert(key, value);
                    }
                }
            }
        });

        key_value
    }
}
