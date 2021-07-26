#[macro_use]
extern crate diesel_migrations;

use scout_gg_backend::game_data::key_value::Ao2KeyValues;

use diesel::PgConnection;
use diesel_migrations::embed_migrations;
use eyre::Result;
use scout_gg_backend::db;
use scout_gg_backend::game_data::aoe2dat::DAT;
use scout_gg_backend::game_data::aoe2dat::{DatCiv, DatTech};
use scout_gg_backend::game_data::civ_tech_tree::Ao2CivsTechTree;
use scout_gg_backend::model::civilization::insert_civilization;
use scout_gg_backend::model::links::{
    TechRequiredTech, TechRequiredUnit, UnitRequiredTech, UnitRequiredUnit,
};
use scout_gg_backend::model::tech::Tech;
use scout_gg_backend::model::unit::Unit;

embed_migrations!();

fn main() -> Result<()> {
    copy_game_files()?;
    let conn = db::establish_connection();
    embedded_migrations::run(&conn).unwrap();
    let values = Ao2KeyValues::create();

    DAT.0
        .civilization_table
        .civilizations
        .iter()
        .enumerate()
        .for_each(|(id, civ)| {
            let civ = DatCiv(&civ).to_civ(id as i16);
            insert_civilization(&conn, &values, &civ).unwrap();
        });

    // Default help text
    units_to_db(&conn, &values);
    tech_to_db(&conn, &values);
    links_to_db(&conn)?;
    Ok(())
}

fn units_to_db(conn: &PgConnection, values: &Ao2KeyValues) {
    // Collect building ids referenced in the tech tree
    let buildings = DAT
        .0
        .tech_tree
        .building_connections
        .iter()
        .map(|building| building.id as i16)
        .collect::<Vec<i16>>();

    // Collect unit ids referenced in the tech tree
    let units = DAT
        .0
        .tech_tree
        .unit_connections
        .iter()
        .map(|unit| unit.id as i16)
        .collect::<Vec<i16>>();

    // Collect uniques unit or building ids referenced in civilisation tech trees and
    // store them in the database
    DAT.get_unit_buildings()
        .iter()
        .filter(|unit| units.contains(&unit.0.id) || buildings.contains(&unit.0.id))
        .map(|unit| unit.to_unit())
        .for_each(|unit| match Unit::insert(conn, values, &unit) {
            Ok(unit) => println!("Inserted : {:?}", unit),
            Err(err) => eprintln!("{}", err),
        });
}

fn links_to_db(conn: &PgConnection) -> Result<()> {
    for unit_tech_link in DAT.get_unit_techs_link() {
        if !(unit_tech_link.required_tech == 101
            || unit_tech_link.required_tech == 102
            || unit_tech_link.required_tech == 103
            || unit_tech_link.required_tech == 104)
        {
            let _ = UnitRequiredTech::insert(conn, &unit_tech_link);
        }
    }

    for tech_tech_link in DAT.get_tech_required_tech() {
        // Ignore duplicate violation and keep going
        if !(tech_tech_link.required_tech == 101
            || tech_tech_link.required_tech == 102
            || tech_tech_link.required_tech == 103
            || tech_tech_link.required_tech == 104)
        {
            let _ = TechRequiredTech::insert(conn, &tech_tech_link);
        }
    }

    for unit_unit_link in DAT.get_unit_unit_link() {
        // Ignore duplicate violation and keep going
        let _ = UnitRequiredUnit::insert(conn, &unit_unit_link);
    }

    for tech_required_unit in DAT.get_tech_unit_links() {
        let _ = TechRequiredUnit::insert(conn, &tech_required_unit);
    }

    DAT.update_root_units(conn);
    DAT.update_root_buildings(conn);
    Tech::update_root_techs(conn).expect("An error occured updating root techs");

    // tag unique unit with their civ id
    let civ_tech_tree_data = std::fs::read_to_string("resources/civTechTrees.json")?;
    let civ_tech_tree_data: Ao2CivsTechTree = serde_json::from_str(&civ_tech_tree_data)?;

    civ_tech_tree_data
        .get_unique_units()
        .iter()
        .for_each(|(civ, unit)| {
            if let Ok(unit) = Unit::by_id(conn, *unit) {
                unit.set_unique(conn, *civ)
                    .expect("Unable to update unique unit");
            }
        });

    civ_tech_tree_data
        .get_civ_enabled_entities()
        .iter()
        .for_each(|civ_enabled_units| civ_enabled_units.to_db(&conn).unwrap());
    Ok(())
}

fn tech_to_db(conn: &PgConnection, values: &Ao2KeyValues) {
    DAT.0
        .tech_table
        .techs
        .iter()
        .enumerate()
        .for_each(|(idx, tech)| {
            let dat_tech = DatTech(tech);
            let tech = dat_tech.to_tech(idx as i16);
            match Tech::insert_with_text(conn, values, &tech) {
                Ok(tech) => println!("Inserted : {:?}", tech),
                Err(err) => eprintln!("Failed to insert tech : {}", err),
            };
        });
}

// Extract game data before hydrating  the database
fn copy_game_files() -> Result<()> {
    let home = std::env!("HOME");
    let resources_path = format!("{}/.steam/steam/steamapps/common/AoE2DE/resources/", home);
    let crate_resources = "resources/keyvalues";
    let key_value_locale = "strings/key-value/key-value-strings-utf8.txt";

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
