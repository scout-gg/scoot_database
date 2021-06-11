use crate::schema::unit;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use eyre::Result;
use crate::model::help_text::HelpText;
use crate::game_data::key_value::Ao2KeyValues;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, Clone)]
#[table_name = "unit"]
pub struct Unit {
    pub id: i32,
    pub unit_type: i32,
    pub internal_name: String,
    pub name: Option<i32>,
    pub help_text_short: Option<i32>,
    pub help_text: Option<i32>,
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
    pub fn insert(conn: &PgConnection, values: &Ao2KeyValues, unit: &Unit) -> Result<Unit> {
        let mut unit = unit.clone();

        unit.name = HelpText::insert_from_values(conn, &values, unit.name.unwrap()).ok().map(|h| h.id);
        unit.help_text = HelpText::insert_from_values(conn, &values, unit.help_text.unwrap()).ok().map(|h| h.id);
        unit.help_text_short = HelpText::insert_from_values(conn, &values, unit.help_text_short.unwrap()).ok().map(|h| h.id);

        diesel::insert_into(unit::table)
            .values(&unit)
            .get_result(conn)
            .map_err(|err| {
                eyre!(
                    "Error inserting unit {:?} with id {:?} : {}",
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
