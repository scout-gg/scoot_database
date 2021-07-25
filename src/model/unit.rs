use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use eyre::Result;

use crate::game_data::key_value::Ao2KeyValues;
use crate::model::help_text::HelpText;
use crate::schema::unit;
use crate::schema::unit::columns::belongs_to_civ;
use crate::schema::unit::columns::is_root;

#[derive(
    Queryable, Insertable, AsChangeset, Identifiable, Serialize, Deserialize, Debug, Clone,
)]
#[table_name = "unit"]
pub struct Unit {
    pub id: i16,
    pub age: i16,
    pub unit_type: i32,
    pub internal_name: String,
    pub name: Option<i32>,
    pub help_text_short: Option<i32>,
    pub help_text: Option<i32>,
    pub wood_cost: i16,
    pub food_cost: i16,
    pub gold_cost: i16,
    pub stone_cost: i16,
    pub attack: i16,
    pub melee_armor: i16,
    pub pierce_armor: i16,
    pub hit_points: i16,
    pub line_of_sight: i16,
    pub garrison_capacity: i16,
    pub is_root: bool,
    pub belongs_to_civ: Option<i16>,
}

impl Unit {
    pub fn insert(conn: &PgConnection, values: &Ao2KeyValues, unit: &Unit) -> Result<Unit> {
        let mut unit = unit.clone();

        unit.name = HelpText::insert_from_values(conn, values, unit.name.unwrap())
            .ok()
            .map(|h| h.id);
        unit.help_text = HelpText::insert_from_values(conn, values, unit.help_text.unwrap())
            .ok()
            .map(|h| h.id);
        unit.help_text_short =
            HelpText::insert_from_values(conn, values, unit.help_text_short.unwrap())
                .ok()
                .map(|h| h.id);

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

    pub fn by_id(conn: &PgConnection, id: i16) -> Result<Unit> {
        unit::table
            .find(id)
            .first(conn)
            .map_err(|err| eyre!("Unit with id {} not found : {}", id, err))
    }

    pub fn set_root(&self, conn: &PgConnection) -> Result<()> {
        diesel::update(self)
            .set(self::is_root.eq(true))
            .execute(conn)
            .expect("Unble to set root value on entity");
        Ok(())
    }

    pub fn set_unique(&self, conn: &PgConnection, civ_id: i16) -> Result<()> {
        diesel::update(self)
            .set(self::belongs_to_civ.eq(Some(civ_id)))
            .execute(conn)
            .expect("Unble to set root value on entity");
        Ok(())
    }
}
