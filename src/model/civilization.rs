use crate::game_data::key_value::Ao2KeyValues;
use crate::model::help_text::HelpText;
use crate::schema::civilization;
use diesel::{PgConnection, RunQueryDsl};
use eyre::Result;

#[derive(Queryable, Insertable)]
#[table_name = "civilization"]
pub struct Civilization {
    pub id: i32,
    pub name: i32,
}

pub fn insert_civilization(
    conn: &PgConnection,
    values: &Ao2KeyValues,
    civilization: &Civilization,
) -> Result<Civilization> {
    HelpText::insert_from_values(conn, &values, civilization.name)?;

    diesel::insert_into(civilization::table)
        .values(civilization)
        .get_result(conn)
        .map_err(|err| eyre!(err))
}
