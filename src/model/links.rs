use diesel::{PgConnection, RunQueryDsl};
use eyre::Result;

use crate::schema::tech_required_tech;
use crate::schema::tech_required_unit;
use crate::schema::unit_required_tech;
use crate::schema::unit_required_unit;

#[derive(Queryable, Insertable, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[table_name = "tech_required_tech"]
pub struct TechRequiredTech {
    pub tech: i32,
    pub required_tech: i32,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[table_name = "unit_required_tech"]
pub struct UnitRequiredTech {
    pub unit: i32,
    pub required_tech: i32,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[table_name = "unit_required_unit"]
pub struct UnitRequiredUnit {
    pub unit: i32,
    pub required_unit: i32,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[table_name = "tech_required_unit"]
pub struct TechRequiredUnit {
    pub tech: i32,
    pub required_unit: i32,
}

impl TechRequiredTech {
    pub fn insert(conn: &PgConnection, tech_required_tech: &Self) -> Result<Self> {
        diesel::insert_into(tech_required_tech::table)
            .values(tech_required_tech)
            .get_result(conn)
            .map_err(|err| {
                eyre!(
                    "Error inserting tech_required_tech {:?} : {}",
                    tech_required_tech,
                    err
                )
            })
    }
}

impl UnitRequiredTech {
    pub fn insert(conn: &PgConnection, unit_required_tech: &Self) -> Result<Self> {
        diesel::insert_into(unit_required_tech::table)
            .values(unit_required_tech)
            .get_result(conn)
            .map_err(|err| {
                eyre!(
                    "Error inserting tech_required_tech {:?} : {}",
                    unit_required_tech,
                    err
                )
            })
    }
}

impl UnitRequiredUnit {
    pub fn insert(conn: &PgConnection, unit_required_unit: &Self) -> Result<Self> {
        diesel::insert_into(unit_required_unit::table)
            .values(unit_required_unit)
            .get_result(conn)
            .map_err(|err| {
                eyre!(
                    "Error inserting tech_required_tech {:?} : {}",
                    unit_required_unit,
                    err
                )
            })
    }
}

impl TechRequiredUnit {
    pub fn insert(conn: &PgConnection, tech_required_unit: &Self) -> Result<Self> {
        diesel::insert_into(tech_required_unit::table)
            .values(tech_required_unit)
            .get_result(conn)
            .map_err(|err| {
                eyre!(
                    "Error inserting tech_required_tech {:?} : {}",
                    tech_required_unit,
                    err
                )
            })
    }
}
