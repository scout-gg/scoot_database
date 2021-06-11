#[macro_use]
extern crate eyre;

use eyre::Result;
use scout_gg_backend::game_data::civ_tech_tree::{Ao2CivsTechTree, CivTechTreeData};

pub fn main() {

}

fn get_civs_tech_trees() -> Result<Vec<CivTechTreeData>> {
    let civ_techs = std::fs::read_to_string("resources/civTechTrees.json")?;
    serde_json::from_str::<Ao2CivsTechTree>(&civ_techs)
        .map_err(|err| eyre!("Unable to parse civ tech tree {}", err))
        .map(|civs| civs.civs)
}
