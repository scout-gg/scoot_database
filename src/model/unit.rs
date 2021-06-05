use crate::schema::unit;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use eyre::Result;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "unit"]
pub struct Unit {
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

    pub help_text_short: Option<String>,
    pub help_text_short_fr: Option<String>,
    pub help_text_short_br: Option<String>,
    pub help_text_short_de: Option<String>,
    pub help_text_short_es: Option<String>,
    pub help_text_short_hi: Option<String>,
    pub help_text_short_it: Option<String>,
    pub help_text_short_jp: Option<String>,
    pub help_text_short_ko: Option<String>,
    pub help_text_short_ms: Option<String>,
    pub help_text_short_mx: Option<String>,
    pub help_text_short_ru: Option<String>,
    pub help_text_short_tr: Option<String>,
    pub help_text_short_tw: Option<String>,
    pub help_text_short_vi: Option<String>,
    pub help_text_short_zh: Option<String>,

    pub help_text: Option<String>,
    pub help_text_fr: Option<String>,
    pub help_text_br: Option<String>,
    pub help_text_de: Option<String>,
    pub help_text_es: Option<String>,
    pub help_text_hi: Option<String>,
    pub help_text_it: Option<String>,
    pub help_text_jp: Option<String>,
    pub help_text_ko: Option<String>,
    pub help_text_ms: Option<String>,
    pub help_text_mx: Option<String>,
    pub help_text_ru: Option<String>,
    pub help_text_tr: Option<String>,
    pub help_text_tw: Option<String>,
    pub help_text_vi: Option<String>,
    pub help_text_zh: Option<String>,

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

impl Unit {
    pub fn insert(conn: &PgConnection, unit: &Unit) -> Result<Unit> {
        diesel::insert_into(unit::table)
            .values(unit)
            .get_result(conn)
            .map_err(|err| {
                eyre!(
                    "Error inserting unit {} with id {} : {}",
                    unit.name,
                    unit.id,
                    err
                )
            })
    }

    pub fn by_id(conn: &PgConnection, id: i32) -> Result<Unit> {
        unit::table
            .find(id)
            .first(conn)
            .map_err(|err| eyre!("Unit with id {} not found : {}", id, err))
    }
}
