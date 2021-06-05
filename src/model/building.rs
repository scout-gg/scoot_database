use crate::schema::building;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use eyre::Result;

#[derive(Identifiable, Queryable, Insertable, Serialize, PartialEq, Debug)]
#[table_name = "building"]
pub struct Building {
    pub id: i32,
    pub name: String,
    pub name_fr: String,
    pub name_br: String,
    pub name_de: String,
    pub name_es: String,
    pub name_hi: String,
    pub name_it: String,
    pub name_jp: String,
    pub name_ko: String,
    pub name_ms: String,
    pub name_mx: String,
    pub name_ru: String,
    pub name_tr: String,
    pub name_tw: String,
    pub name_vi: String,
    pub name_zh: String,
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
    pub fn insert(conn: &PgConnection, building: &Building) -> Result<Building> {
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
