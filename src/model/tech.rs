use crate::model::building::Building;
use crate::schema::technology;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use eyre::Result;

#[derive(Queryable, Insertable, Associations, Serialize, Deserialize, PartialEq, Debug)]
#[table_name = "technology"]
#[belongs_to(Building)]
pub struct Tech {
    pub id: i32,
    pub name: String,
    pub name_fr: Option<String>,
    pub name_br: Option<String>,
    pub name_de: Option<String>,
    pub name_es: Option<String>,
    pub name_hi: Option<String>,
    pub name_it: Option<String>,
    pub name_jp: Option<String>,
    pub name_ko: Option<String>,
    pub name_ms: Option<String>,
    pub name_mx: Option<String>,
    pub name_ru: Option<String>,
    pub name_tr: Option<String>,
    pub name_tw: Option<String>,
    pub name_vi: Option<String>,
    pub name_zh: Option<String>,
    pub building_id: Option<i32>,
    pub research_time: i32,
    pub wood_cost: i32,
    pub food_cost: i32,
    pub gold_cost: i32,
    pub stone_cost: i32,
}

impl Tech {
    pub fn insert(conn: &PgConnection, tech: &Tech) -> Result<()> {
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
