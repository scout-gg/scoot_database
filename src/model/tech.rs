use crate::model::building::Building;
use crate::schema::technology;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use eyre::{Result, Report};
use crate::model::help_text::HelpText;
use crate::game_data::key_value::Ao2KeyValues;

#[derive(Queryable, Insertable, Associations, Serialize, Deserialize, PartialEq, Debug)]
#[table_name = "technology"]
#[belongs_to(Building, TechTreeUnit, TechTreeResearch, TechTreeBuilding)]
pub struct Tech {
    pub id: i32,
    pub internal_name: String,
    pub name: i32,
    pub building_id: Option<i32>,
    pub research_time: i32,
    pub wood_cost: i32,
    pub food_cost: i32,
    pub gold_cost: i32,
    pub stone_cost: i32,
}

impl Tech {
    pub fn insert_with_text(conn: &PgConnection, values: &Ao2KeyValues, tech: &Tech) -> Result<()> {
        HelpText::insert_from_values(conn, values, tech.name)?;

        Tech::insert(conn, tech)
    }

    pub fn insert(conn: &PgConnection, tech: &Tech) -> Result<(), Report> {
        diesel::insert_into(technology::table)
            .values(tech)
            .execute(conn)
            .map(|_| ())
            .map_err(|err| {
                eyre!(
                    "Error inserting tech {} with id {}: {}",
                    tech.name,
                    tech.id,
                    err
                )
            })
    }

    pub fn by_id(conn: &PgConnection, id: i32) -> Result<Tech> {
        technology::table
            .find(id)
            .first(conn)
            .map_err(|err| eyre!("Tech with id {} not found : {}", id, err))
    }
}
