use diesel::{PgConnection, RunQueryDsl};
use crate::schema::civilisation;

#[derive(Queryable, Insertable)]
#[table_name="civilisation"]
pub struct Civilisation {
    pub id: i32,
    pub name: String,
}

pub fn insert_civilisation(conn: &PgConnection, id: i32, name: &str) -> Civilisation {
    use crate::schema::civilisation;
    let civ = Civilisation {
        id,
        name: name.to_string(),
    };

    diesel::insert_into(civilisation::table)
        .values(&civ)
        .get_result(conn)
        .expect("Error saving civilisation")
}
