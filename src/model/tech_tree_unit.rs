use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use eyre::Result;
use crate::schema::tech_tree_unit;



#[derive(Queryable, Insertable, Associations, Serialize, Deserialize, Debug)]
#[table_name = "tech_tree_unit"]
pub struct TechTreeUnit {
    pub id: i32,
    pub age: i16,
    pub required_tech: Option<i32>,
    pub upper_building: i32,
    pub parent_unit: Option<i32>,
    pub enabling_research: Option<i32>,
}


impl TechTreeUnit {
    pub fn insert(conn: &PgConnection, unit: &TechTreeUnit) -> Result<TechTreeUnit> {
        diesel::insert_into(tech_tree_unit::table)
            .values(unit)
            .get_result(conn)
            .map_err(|err| {
                eyre!(
                    "Error tech_tree_unit with id {} : {}",
                    unit.id,
                    err
                )
            })
    }

    pub fn by_id(conn: &PgConnection, id: i32) -> Result<TechTreeUnit> {
        tech_tree_unit::table
            .find(id)
            .first(conn)
            .map_err(|err| eyre!("Unit with id {} not found : {}", id, err))
    }
}


