use diesel::{PgConnection, RunQueryDsl, QueryDsl};
use crate::schema::tech_tree_tech;
use eyre::Result;

#[derive(Queryable, Insertable, Associations, Serialize, Deserialize, Debug)]
#[table_name = "tech_tree_tech"]
pub struct TechTreeResearch {
    pub id: i32,
    pub age: i16,
    pub required_tech: Option<i32>,
    pub upper_building: i32,
}


impl TechTreeResearch {
    pub fn insert(conn: &PgConnection, tech: &TechTreeResearch) -> Result<TechTreeResearch> {
        diesel::insert_into(tech_tree_tech::table)
            .values(tech)
            .get_result(conn)
            .map_err(|err| {
                eyre!(
                    "Error tech_tree_tech with id {} : {}",
                    tech.id,
                    err
                )
            })
    }

    pub fn by_id(conn: &PgConnection, id: i32) -> Result<TechTreeResearch> {
        tech_tree_tech::table
            .find(id)
            .first(conn)
            .map_err(|err| eyre!("Error tech_tree_tech with id {} not found : {}", id, err))
    }
}