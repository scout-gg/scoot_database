#[macro_use]
extern crate diesel_migrations;

use scout_gg_backend::game_data::key_value::Ao2KeyValues;

use diesel::PgConnection;
use diesel_migrations::embed_migrations;
use eyre::Result;
use scout_gg_backend::db;
use scout_gg_backend::game_data::aoe2dat::{
    Ao2TechData, Aoe2Dat, AoeFullDat, BUILDING, MILITARY_UNITS,
};
use scout_gg_backend::model::building::Building;
use scout_gg_backend::model::civilization::insert_civilization;
use scout_gg_backend::model::tech::Tech;
use scout_gg_backend::model::unit::Unit;

embed_migrations!();

fn main() -> Result<()> {
    copy_game_files()?;
    let conn = db::establish_connection();
    embedded_migrations::run(&conn).unwrap();
    let values = Ao2KeyValues::create();

    let full_data = std::fs::read_to_string("resources/full.json")?;
    let full_data: AoeFullDat = serde_json::from_str(&full_data)?;

    full_data.civs.iter().enumerate().for_each(|(id, civ)| {
        let civ = civ.to_civ(id as i32, &values);
        insert_civilization(&conn, &civ);
    });

    let unit_file = std::fs::read_to_string("resources/units_buildings_techs.json")?;
    let data: Aoe2Dat = serde_json::from_str(&unit_file)?;
    &units_to_db(&conn, &values, &data);
    &building_to_db(&conn, &values, &data);
    &tech_to_db(&conn, &values, full_data.techs);

    Ok(())
}

fn building_to_db(conn: &PgConnection, values: &Ao2KeyValues, data: &Aoe2Dat) {
    data.units_buildings
        .values()
        .filter(|data| data.unit_type == BUILDING)
        .map(|building| building.to_building(&values))
        .flatten()
        .for_each(|building| {
            if let Err(err) = Building::insert(&conn, &building) {
                eprintln!("{}", err)
            }
        });
}

fn units_to_db(conn: &PgConnection, values: &Ao2KeyValues, data: &Aoe2Dat) {
    data.units_buildings
        .values()
        .filter(|data| data.unit_type == MILITARY_UNITS)
        .map(|unit| unit.to_unit(&values))
        .flatten()
        .for_each(|unit| {
            if let Err(err) = Unit::insert(&conn, &unit) {
                eprintln!("{}", err);
            }
        });
}

fn tech_to_db(conn: &PgConnection, values: &Ao2KeyValues, data: Vec<Ao2TechData>) {
    data.iter().enumerate().for_each(|(idx, tech)| {
        let tech = tech.to_tech(idx as i32, values);
        if let Err(err) = Tech::insert(conn, &tech) {
            eprintln!("{}", err);
        }
    });
}

fn copy_game_files() -> Result<()> {
    let home = std::env!("HOME");
    let resources_path = format!("{}/.steam/steam/steamapps/common/AoE2DE/resources/", home);
    let crate_resources = "resources/keyvalues";
    let key_value_locale = "/strings/key-value/key-value-strings-utf8.txt";

    for entry in std::fs::read_dir(resources_path)? {
        let entry = entry?;
        match entry.file_name().to_str().unwrap() {
            "br" => std::fs::copy(
                format!("{}/{}", entry.path().to_str().unwrap(), key_value_locale),
                format!("{}/br", crate_resources),
            )?,
            "de" => std::fs::copy(
                format!("{}/{}", entry.path().to_str().unwrap(), key_value_locale),
                format!("{}/de", crate_resources),
            )?,
            "en" => std::fs::copy(
                format!("{}/{}", entry.path().to_str().unwrap(), key_value_locale),
                format!("{}/en", crate_resources),
            )?,
            "es" => std::fs::copy(
                format!("{}/{}", entry.path().to_str().unwrap(), key_value_locale),
                format!("{}/es", crate_resources),
            )?,
            "fr" => std::fs::copy(
                format!("{}/{}", entry.path().to_str().unwrap(), key_value_locale),
                format!("{}/fr", crate_resources),
            )?,
            "hi" => std::fs::copy(
                format!("{}/{}", entry.path().to_str().unwrap(), key_value_locale),
                format!("{}/hi", crate_resources),
            )?,
            "it" => std::fs::copy(
                format!("{}/{}", entry.path().to_str().unwrap(), key_value_locale),
                format!("{}/it", crate_resources),
            )?,
            "jp" => std::fs::copy(
                format!("{}/{}", entry.path().to_str().unwrap(), key_value_locale),
                format!("{}/jp", crate_resources),
            )?,
            "ko" => std::fs::copy(
                format!("{}/{}", entry.path().to_str().unwrap(), key_value_locale),
                format!("{}/ko", crate_resources),
            )?,
            "ms" => std::fs::copy(
                format!("{}/{}", entry.path().to_str().unwrap(), key_value_locale),
                format!("{}/ms", crate_resources),
            )?,
            "mx" => std::fs::copy(
                format!("{}/{}", entry.path().to_str().unwrap(), key_value_locale),
                format!("{}/mx", crate_resources),
            )?,
            "ru" => std::fs::copy(
                format!("{}/{}", entry.path().to_str().unwrap(), key_value_locale),
                format!("{}/ru", crate_resources),
            )?,
            "tr" => std::fs::copy(
                format!("{}/{}", entry.path().to_str().unwrap(), key_value_locale),
                format!("{}/tr", crate_resources),
            )?,
            "tw" => std::fs::copy(
                format!("{}/{}", entry.path().to_str().unwrap(), key_value_locale),
                format!("{}/tw", crate_resources),
            )?,
            "vi" => std::fs::copy(
                format!("{}/{}", entry.path().to_str().unwrap(), key_value_locale),
                format!("{}/vi", crate_resources),
            )?,
            "zh" => std::fs::copy(
                format!("{}/{}", entry.path().to_str().unwrap(), key_value_locale),
                format!("{}/zh", crate_resources),
            )?,
            _ => 0,
        };
    }
    Ok(())
}
