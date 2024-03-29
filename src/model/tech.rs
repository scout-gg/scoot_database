use diesel::prelude::*;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use eyre::Result;

use crate::game_data::key_value::Ao2KeyValues;
use crate::model::help_text::HelpText;
use crate::schema::tech_required_tech::dsl::tech_required_tech;
use crate::schema::technology;
use crate::schema::technology::columns::is_root;

#[derive(
    Queryable,
    AsChangeset,
    Insertable,
    Identifiable,
    Serialize,
    Deserialize,
    PartialEq,
    Debug,
    Clone,
)]
#[table_name = "technology"]
pub struct Tech {
    pub id: i16,
    pub age: i16,
    pub internal_name: String,
    pub name: Option<i32>,
    pub research_time: i16,
    pub wood_cost: i16,
    pub food_cost: i16,
    pub gold_cost: i16,
    pub stone_cost: i16,
    pub is_root: bool,
}

impl Tech {
    pub fn insert_with_text(
        conn: &PgConnection,
        values: &Ao2KeyValues,
        tech: &Tech,
    ) -> Result<Tech> {
        let mut tech = tech.clone();
        let name = HelpText::insert_from_values(conn, values, tech.name.unwrap()).map(|h| h.id)?;

        tech.name = Some(name);
        Tech::insert(conn, &tech)
    }

    pub fn insert(conn: &PgConnection, tech: &Tech) -> Result<Tech> {
        diesel::insert_into(technology::table)
            .values(tech)
            .get_result(conn)
            .map_err(|err| {
                eyre!(
                    "Error inserting tech {:?} with id {}: {}",
                    tech.name,
                    tech.id,
                    err
                )
            })
    }

    pub fn by_id(conn: &PgConnection, id: i16) -> Result<Tech> {
        technology::table
            .find(id)
            .first(conn)
            .map_err(|err| eyre!("Tech with id {} not found : {}", id, err))
    }

    pub fn update_root(&self, conn: &PgConnection) -> Result<()> {
        diesel::update(self)
            .set(self::is_root.eq(true))
            .execute(conn)
            .expect("Unble to set root value on entity");
        Ok(())
    }

    pub fn update_root_techs(conn: &PgConnection) -> Result<()> {
        let res: Vec<(i16, Option<i16>)> = technology::table
            .left_outer_join(
                tech_required_tech.on(crate::schema::tech_required_tech::columns::tech
                    .eq(crate::schema::technology::columns::id)),
            )
            .select((
                crate::schema::technology::columns::id,
                crate::schema::tech_required_tech::columns::tech.nullable(),
            ))
            .load(conn)?;

        let root_techs: Vec<&i16> = res
            .iter()
            .filter(|(_, tech)| tech.is_none())
            .map(|(id, _)| id)
            .collect();

        root_techs.iter().for_each(|id| {
            let tech =
                Tech::by_id(conn, **id).expect("Unable to get tech while setting root value");
            tech.update_root(conn).expect("Error setting root tech");
        });
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::db;
    use crate::model::tech::Tech;

    #[test]
    fn should_join_root_tech() {
        let db = db::establish_connection();
        Tech::update_root_techs(&db);
    }
}
