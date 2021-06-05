#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate eyre;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;

use scout_gg_backend::game_data::key_value::Ao2KeyValues;

use scout_gg_backend::db;
use std::collections::HashMap;
use scout_gg_backend::model::tech::{Tech, insert_tech};
use eyre::Result;
use diesel::{QueryDsl, PgConnection};
use serde_json::Value;
use std::path::Path;
use scout_gg_backend::game_data::aoe2dat::{CivData, Aoe2DatUnit, Aoe2Dat, MILITARY_UNITS, BUILDING, Ao2TechData};
use std::any::Any;
use scout_gg_backend::model::unit::insert_unit;
use scout_gg_backend::model::building::insert_building;
use diesel_migrations::embed_migrations;
use std::fs::File;
use scout_gg_backend::model::civilisation::insert_civilisation;

embed_migrations!("migrations");

fn main() -> Result<()> {
    let conn = db::establish_connection();
    embedded_migrations::run(&conn);
    let values = Ao2KeyValues::create();

   // let civs_data = std::fs::read_to_string("resources/game_data/Civs.json")?;
   // let civs_data: Vec<CivData> = serde_json::from_str(&civs_data)?;
   // let civ = civs_data.get(1).unwrap();
   // println!("Civ Name: {}", civ.name);

    let unit_file = std::fs::read_to_string("resources/units_buildings_techs.json")?;
    let data: Aoe2Dat = serde_json::from_str(&unit_file)?;
    &units_to_db(&conn, &values, &data);
    &building_to_db(&conn, &values, &data);

    let techs = std::fs::read_to_string("resources/game_data/Techs.json")?;
    let data: Vec<Ao2TechData> = serde_json::from_str(&techs)?;
    &tech_to_db(&conn, &values, data);

    Ok(())
}

fn building_to_db(conn: &PgConnection, values: &Ao2KeyValues, data: &Aoe2Dat) {
    data.units_buildings.values()
        .filter(|data| data.unit_type == BUILDING)
        .map(|building| building.to_building(&values))
        .filter_map(|building| building)
        .for_each(|building| {
            insert_building(&conn, &building);
        });
}

fn units_to_db(conn: &PgConnection, values: &Ao2KeyValues, data: &Aoe2Dat) {
    data.units_buildings.values()
        .filter(|data| data.unit_type == MILITARY_UNITS)
        .map(|unit| unit.to_unit(&values))
        .filter_map(|unit| unit)
        .for_each(|unit| {
            insert_unit(&conn, &unit);
        });
}

fn tech_to_db(conn: &PgConnection, values: &Ao2KeyValues, data: Vec<Ao2TechData>) {
    data.iter().enumerate()
        .for_each(|(idx, tech)| {
            let tech = tech.to_tech(idx as i32, values);
            if let Some(tech) = tech {
                insert_tech(conn, &tech);
            }
        });
}


fn create_civs(values: &Ao2KeyValues, conn: &PgConnection) {
    let civs = values.get_civs();
    civs.iter().enumerate()
        .for_each(|(id, civ)| {
            insert_civilisation(conn, id as i32, civ);
        });
}