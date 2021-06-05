use diesel::{PgConnection, RunQueryDsl, QueryResult};
use crate::schema::unit;

#[derive(Queryable, Insertable)]
#[table_name = "unit"]
pub struct Unit {
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

pub fn insert_unit(conn: &PgConnection, unit: &Unit) -> QueryResult<Unit> {
    use crate::schema::unit;
    diesel::insert_into(unit::table)
        .values(unit)
        .get_result(conn)
}