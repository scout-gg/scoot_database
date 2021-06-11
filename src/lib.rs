#![recursion_limit = "256"]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derives;
#[macro_use]
extern crate eyre;
#[macro_use]
extern crate serde_derive;

use std::fs::File;

use eyre::Result;
use genie::RecordedGame;

pub mod db;
pub mod game_data;
pub mod model;
mod schema;

fn _load_game_file(path: &str) -> Result<RecordedGame<File>> {
    let file = File::open(path)?;
    RecordedGame::new(file).map_err(|err| eyre!(err))
}

#[cfg(test)]
mod test {
    use std::fs::File;

    use eyre::Result;
    use genie::rec::actions::{Action, Command};
    use genie::RecordedGame;

    use crate::_load_game_file;

    const TEST_RECORD: &str =
        "resources/MP Replay v101.101.47820.0 @2021.06.01 064229 (1).aoe2record";

    #[test]
    fn print_game_data() -> Result<()> {
        let mut game = _load_game_file(TEST_RECORD);
        print_actions(&mut game?);

        Ok(())
        //let name_and_civ: Vec<String> = game.header().unwrap().players()
        //    .skip(1)
        //    .map(|player| ( player.name(), player.civilization_id))
        //    .map(|(name, civ_id)| (name, &civs[usize::from(civ_id)]))
        //    .map(|(name, civ) | format!("{} Playing as the {}", name, civ))
        //    .collect();
    }

    fn print_actions(r: &mut RecordedGame<File>) {
        for act in r.actions().unwrap() {
            match act.unwrap() {
                Action::Command(cmd) => match cmd {
                    Command::Research(r) => {}
                    Command::Build(b) => println!("{:?}", b),
                    Command::Create(c) => println!("{:?}", c),
                    Command::Queue(c) => println!("{:?}", c),
                    _ => {}
                },
                _ => {}
            }
        }
    }


    use crate::game_data::tech_tree::{TechTreeDat, BuildingConnectionDat};

    #[test]
    fn test() -> Result<()> {
        let result = std::fs::read_to_string("resources/tech.json")?;
        let data: TechTreeDat = serde_json::from_str(&result)?;
        let tech_with_units: Vec<i32> = data.research_connections.iter()
            .filter(|tech| !tech.units.is_empty())
            .map(|tech| tech.tech_id)
            .collect();

        println!("{:?}", tech_with_units);


        let tech_with_techs: Vec<i32> = data.research_connections.iter()
            .filter(|tech| !tech.techs.is_empty())
            .map(|tech| tech.tech_id)
            .collect();


        println!("{:?}", tech_with_techs);


        Ok(())
    }
}
