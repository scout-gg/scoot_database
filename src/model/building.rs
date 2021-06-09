use crate::schema::building;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use eyre::Result;
use crate::model::help_text::HelpText;
use crate::game_data::key_value::Ao2KeyValues;

#[derive(Queryable, Insertable, Serialize, PartialEq, Debug)]
#[table_name = "building"]
pub struct Building {
    pub id: i32,
    pub internal_name: String,
    pub name: i32,
    pub wood_cost: i32,
    pub food_cost: i32,
    pub gold_cost: i32,
    pub stone_cost: i32,
    pub attack: i32,
    pub melee_armor: i32,
    pub pierce_armor: i32,
    pub hit_points: i32,
    pub line_of_sight: i32,
    pub garrison_capacity: i32,
}

impl Building {
    pub fn insert(conn: &PgConnection, values: &Ao2KeyValues, building: &Building) -> Result<Building> {
        HelpText::insert_from_values(conn, &values, building.name)?;
        diesel::insert_into(building::table)
            .values(building)
            .get_result(conn)
            .map_err(|err| {
                eyre!(
                    "Error inserting building {} with id {} : {}",
                    building.name,
                    building.id,
                    err
                )
            })
    }

    pub fn by_id(conn: &PgConnection, id: i32) -> Result<Building> {
        building::table
            .find(id)
            .first(conn)
            .map_err(|err| eyre!("Building with id {} not found : {}", id, err))
    }
}
