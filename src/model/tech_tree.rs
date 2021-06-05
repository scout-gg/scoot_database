use crate::model::tech::Tech;
use crate::model::unit::Unit;
use crate::schema::tech_tree;
use crate::schema::tech_tree::civ_tech_tree;
use crate::schema::tech_tree::dsl::id;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use eyre::Result;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct CivTechTree {
    pub civ_id: i32,
    pub civ_base_name: String,
    pub civ_techs_buildings: Vec<CivTechTreeBuilding>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CivTechTreeBuilding {
    pub age: String,
    pub name: String,
    pub picture_index: i32,
    pub units: Vec<CivTechTreeUnit>,
    pub researches: Vec<CivTechTreeResearch>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CivTechTreeResearch {
    pub age: String,
    pub tech: Tech,
    pub child: Option<Box<CivTechTreeResearch>>,
    pub picture_index: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CivTechTreeUnitUpgrade {
    pub age: String,
    pub tech: Tech,
    pub upgrade_to: Box<CivTechTreeUnit>,
    pub picture_index: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CivTechTreeUnit {
    pub age: String,
    pub unit: Unit,
    pub upgrade: Option<CivTechTreeUnitUpgrade>,
    pub picture_index: i32,
}

impl CivTechTree {
    pub fn insert(&self, conn: &PgConnection, civ_id: i32) -> Result<Value> {
        let json = serde_json::to_value(&self)?;
        diesel::insert_into(tech_tree::table)
            .values((id.eq(&civ_id), civ_tech_tree.eq(&json)))
            .returning(civ_tech_tree)
            .get_result(conn)
            .map_err(|err| eyre!("Error inserting tech tree with id {} : {}", civ_id, err))
    }

    pub fn by_id(conn: &PgConnection, civ_id: i32) -> Result<Value> {
        tech_tree::table
            .select(civ_tech_tree)
            .find(civ_id)
            .first(conn)
            .map_err(|err| eyre!("TechTree with id {} not found : {}", civ_id, err))
    }
}
