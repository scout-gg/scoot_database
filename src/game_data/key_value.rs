use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

const CIV_START: i32 = 10270;
const CIV_END: i32 = 10318;
const BUILDING_OFFSET: i32 = 4780;

pub struct Ao2KeyValues {
    pub en: HashMap<i32, String>,
    pub fr: HashMap<i32, String>,
}

impl Ao2KeyValues {
    pub fn create() -> Self {
        Ao2KeyValues {
            en: Ao2KeyValues::load("en"),
            fr: Ao2KeyValues::load("fr")
        }

    }

    fn load(lang: &str) -> HashMap<i32, String> {
        let mut key_value = HashMap::new();
        let path = &format!("/home/okno/.steam/steam/steamapps/common/AoE2DE/resources/{}/strings/key-value/key-value-strings-utf8.txt", lang);
        let key_value_file = File::open(path).unwrap();
        let buf_reader = BufReader::new(key_value_file);
        buf_reader.lines().for_each(|line| {
            let line = line.unwrap();
            if !line.starts_with("//") {
                let split_idx = line.find(" ");
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

    pub fn get_civs(&self) -> Vec<String> {
        let mut civs = Vec::new();
        for i in CIV_START..CIV_END {
            civs.push(self.en.get(&i).unwrap().clone());
        }

        civs
    }
}
