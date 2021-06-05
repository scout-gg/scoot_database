use crate::schema::civilization;
use diesel::{PgConnection, RunQueryDsl};

#[derive(Queryable, Insertable)]
#[table_name = "civilization"]
pub struct Civilization {
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
}

pub fn insert_civilization(conn: &PgConnection, civilization: &Civilization) -> Civilization {
    diesel::insert_into(civilization::table)
        .values(civilization)
        .get_result(conn)
        .expect("Error saving civilisation")
}
