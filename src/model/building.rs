use diesel::{PgConnection, RunQueryDsl, QueryResult};
use crate::schema::building;

#[derive(Identifiable, Queryable, Insertable, PartialEq, Debug)]
#[table_name = "building"]
pub struct Building {
    pub id: i32,
    pub name: String,
    pub name_fr: String,

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

pub fn insert_building(conn: &PgConnection, building: &Building) -> QueryResult<Building> {
    use crate::schema::building;
    diesel::insert_into(building::table)
        .values(building)
        .get_result(conn)}