#[macro_use]
extern crate eyre;

use eyre::Result;
use scout_gg_backend::db;
use scout_gg_backend::game_data::tech_tree::{
    Ao2CivsTechTree, CivTechTreeData, LinkType, NodeStatus, NodeType, TechThreeNode,
};
use scout_gg_backend::model::tech_tree::CivTechTree;

pub fn main() -> Result<()> {
    let conn = db::establish_connection();

    let civs_tech_tree = get_civs_tech_trees()?;

    for (id, tech_tree_data) in civs_tech_tree.iter().enumerate() {
        let tech_tree = tech_tree_data.to_tech_tree(id as i32, &conn)?;
        tech_tree.insert(&conn, id as i32 + 1)?;
    }
    Ok(())
}

fn get_civs_tech_trees() -> Result<Vec<CivTechTreeData>> {
    let civ_techs = std::fs::read_to_string("resources/civTechTrees.json")?;
    serde_json::from_str::<Ao2CivsTechTree>(&civ_techs)
        .map_err(|err| eyre!("Unable to parse civ tech tree {}", err))
        .map(|civs| civs.civs)
}
