use diesel::{PgConnection, RunQueryDsl, QueryResult};
use crate::schema::{technology, building};
use crate::model::building::Building;

#[derive(Queryable, Insertable, Associations, PartialEq, Debug)]
#[table_name="technology"]
#[belongs_to(Building)]
pub struct Tech {
    pub id: i32,
    pub name: String,
    pub name_fr: String,
    pub building_id: i32,
    pub research_time: i32,
    pub wood_cost: i32,
    pub food_cost: i32,
    pub gold_cost: i32,
    pub stone_cost: i32,
}

pub fn insert_tech(conn: &PgConnection, tech: &Tech) {
    diesel::insert_into(technology::table)
        .values(tech)
        .execute(conn);
}