use std::collections::HashMap;
use serde_json::Value;
use eyre::Result;

// split the very large full.json game data file in multiple ones
fn main() -> Result<()> {
    let data = std::fs::read_to_string("resources/game_data/Civs.json")?;
    let v: HashMap<String, Value> = serde_json::from_str(&data)?;

    v.iter().for_each(|(key, value) | {
        let value = serde_json::to_string(value).unwrap();
        let path = format!("resources/game_data/civs/{}.json", key);
        std::fs::write(path, &value).unwrap();
    });

    Ok(())
}