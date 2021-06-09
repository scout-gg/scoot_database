use diesel::{PgConnection, RunQueryDsl, QueryDsl};
use eyre::Result;
use crate::schema::tech_tree_building;

#[derive(Queryable, Associations, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "tech_tree_building"]
pub struct TechTreeBuilding {
    pub age: i16,
    pub building_id: i32,
    pub enabling_research: Option<i32>,
    pub required_building: Option<i32>,
    pub required_tech: Option<i32>,
}

impl TechTreeBuilding {
    pub fn insert(conn: &PgConnection, building: &TechTreeBuilding) -> Result<TechTreeBuilding> {
        diesel::insert_into(tech_tree_building::table)
            .values(building)
            .get_result(conn)
            .map_err(|err| {
                eyre!(
                    "Error tech_tree_building with id {} : {}",
                    building.building_id,
                    err
                )
            })
    }

    pub fn by_id(conn: &PgConnection, id: i32) -> Result<TechTreeBuilding> {
        tech_tree_building::table
            .find(id)
            .first(conn)
            .map_err(|err| eyre!("Unit with id {} not found : {}", id, err))
    }
}
