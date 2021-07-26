use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use eyre::Result;

use crate::schema::civ_tech;
use crate::schema::civ_unit;
use crate::schema::tech_required_tech;
use crate::schema::tech_required_unit;
use crate::schema::unit_required_tech;
use crate::schema::unit_required_unit;

#[derive(Queryable, Insertable, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[table_name = "tech_required_tech"]
pub struct TechRequiredTech {
    pub tech: i16,
    pub required_tech: i16,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[table_name = "unit_required_tech"]
pub struct UnitRequiredTech {
    pub unit: i16,
    pub required_tech: i16,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[table_name = "unit_required_unit"]
pub struct UnitRequiredUnit {
    pub unit: i16,
    pub required_unit: i16,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[table_name = "tech_required_unit"]
pub struct TechRequiredUnit {
    pub tech: i16,
    pub required_unit: i16,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[table_name = "civ_tech"]
pub struct CivTech {
    pub civ_id: i16,
    pub tech_id: i16,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[table_name = "civ_unit"]
pub struct CivUnit {
    pub civ_id: i16,
    pub unit_id: i16,
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
            .map(|requirement| {
                println!("{:?}", requirement);
                requirement
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
                    "Error inserting unit_required_tech {:?} : {}",
                    unit_required_tech,
                    err
                )
            })
            .map(|requirement| {
                println!("{:?}", requirement);
                requirement
            })
    }

    pub fn by_id(conn: &PgConnection, id: i16) -> Result<UnitRequiredTech> {
        unit_required_tech::table
            .filter(unit_required_tech::unit.eq(id))
            .first(conn)
            .map_err(|err| eyre!("Tech with id {} not found : {}", id, err))
    }
}

impl UnitRequiredUnit {
    pub fn insert(conn: &PgConnection, unit_required_unit: &Self) -> Result<Self> {
        diesel::insert_into(unit_required_unit::table)
            .values(unit_required_unit)
            .get_result(conn)
            .map_err(|err| {
                eyre!(
                    "Error inserting unit_required_unit {:?} : {}",
                    unit_required_unit,
                    err
                )
            })
            .map(|requirement| {
                println!("{:?}", requirement);
                requirement
            })
    }

    pub fn by_id(conn: &PgConnection, id: i16) -> Result<UnitRequiredUnit> {
        unit_required_unit::table
            .filter(unit_required_unit::unit.eq(id))
            .first(conn)
            .map_err(|err| eyre!("Tech with id {} not found : {}", id, err))
    }
}

impl TechRequiredUnit {
    pub fn insert(conn: &PgConnection, tech_required_unit: &Self) -> Result<Self> {
        diesel::insert_into(tech_required_unit::table)
            .values(tech_required_unit)
            .get_result(conn)
            .map_err(|err| {
                eyre!(
                    "Error inserting tech_required_unit {:?} : {}",
                    tech_required_unit,
                    err
                )
            })
            .map(|requirement| {
                println!("Inserted {:?}", requirement);
                requirement
            })
    }
}

impl CivTech {
    pub fn insert(self, conn: &PgConnection) -> Result<Self> {
        diesel::insert_into(civ_tech::table)
            .values(self.clone())
            .get_result(conn)
            .map_err(|err| eyre!("Error inserting civ_tech {:?} : {}", &self, err))
            .map(|tech| {
                println!("Inserted {:?}", tech);
                tech
            })
    }
}

impl CivUnit {
    pub fn insert(self, conn: &PgConnection) -> Result<Self> {
        diesel::insert_into(civ_unit::table)
            .values(self.clone())
            .get_result(conn)
            .map_err(|err| eyre!("Error inserting civ_tech {:?} : {}", &self, err))
            .map(|unit| {
                println!("Inserted {:?}", unit);
                unit
            })
    }
}
