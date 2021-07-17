#[macro_use]
extern crate diesel_migrations;

use scout_gg_backend::game_data::key_value::Ao2KeyValues;

use diesel::PgConnection;
use diesel_migrations::embed_migrations;
use eyre::Result;
use scout_gg_backend::db;
use scout_gg_backend::game_data::aoe2dat::{Ao2TechData, Aoe2Dat, AoeFullDat};
use scout_gg_backend::game_data::civ_tech_tree::Ao2CivsTechTree;
use scout_gg_backend::model::civilization::insert_civilization;
use scout_gg_backend::model::links::{
    TechRequiredTech, TechRequiredUnit, UnitRequiredTech, UnitRequiredUnit,
};
use scout_gg_backend::model::tech::Tech;
use scout_gg_backend::model::unit::Unit;
use scout_gg_backend::TECHS;

embed_migrations!();

fn main() -> Result<()> {
    copy_game_files()?;
    let conn = db::establish_connection();
    embedded_migrations::run(&conn).unwrap();
    let values = Ao2KeyValues::create();

    let full_data = std::fs::read_to_string("resources/full.json")?;
    let full_data: AoeFullDat = serde_json::from_str(&full_data)?;

    full_data.civs.iter().enumerate().for_each(|(id, civ)| {
        let civ = civ.to_civ(id as i32);
        insert_civilization(&conn, &values, &civ).unwrap();
    });

    // Default help text
    let unit_file = std::fs::read_to_string("resources/units_buildings_techs.json")?;
    let data: Aoe2Dat = serde_json::from_str(&unit_file)?;
    units_to_db(&conn, &values, &data);
    tech_to_db(&conn, &values, full_data.techs);
    links_to_db(&conn)?;
    Ok(())
}

fn units_to_db(conn: &PgConnection, values: &Ao2KeyValues, data: &Aoe2Dat) {
    data.units_buildings
        .values()
        .map(|unit| unit.to_unit())
        .for_each(|unit| {
            println!(
                "Unit or building converter to entity : {} - {:?}, {}",
                unit.id, unit.name, unit.unit_type
            );
            if let Err(err) = Unit::insert(conn, values, &unit) {
                eprintln!("{}", err);
            }
        });
}

fn links_to_db(conn: &PgConnection) -> Result<()> {
    for unit_tech_link in TECHS.get_unit_techs_link() {
        UnitRequiredTech::insert(conn, &unit_tech_link)?;
    }

    for tech_tech_link in TECHS.get_tech_required_tech() {
        // Ignore duplicate violation and keep going
        if !(tech_tech_link.required_tech == 101
            || tech_tech_link.required_tech == 102
            || tech_tech_link.required_tech == 103
            || tech_tech_link.required_tech == 104)
        {
            let _ = TechRequiredTech::insert(conn, &tech_tech_link);
        }
    }

    for unit_unit_link in TECHS.get_unit_unit_link() {
        // Ignore duplicate violation and keep going
        let _ = UnitRequiredUnit::insert(conn, &unit_unit_link);
    }

    for tech_required_unit in TECHS.get_tech_unit_links() {
        let _ = TechRequiredUnit::insert(conn, &tech_required_unit);
    }

    TECHS.update_root_units(conn);
    Tech::update_root_techs(conn).expect("An error occured updating root techs");

    // tag unique unit with their civ id
    let civ_tech_tree_data = std::fs::read_to_string("resources/civTechTrees.json")?;
    let civ_tech_tree_data: Ao2CivsTechTree = serde_json::from_str(&civ_tech_tree_data)?;
    civ_tech_tree_data
        .get_unique_units()
        .iter()
        .for_each(|(civ, unit)| {
            Unit::by_id(conn, *unit)
                .expect("Unable to get unique unit")
                .set_unique(conn, *civ)
                .expect("Unable to update unique unit");
        });

    civ_tech_tree_data
        .get_civ_enabled_entities()
        .iter()
        .for_each(|civ_enabled_units| civ_enabled_units.to_db(&conn).unwrap());
    Ok(())
}

fn tech_to_db(conn: &PgConnection, values: &Ao2KeyValues, data: Vec<Ao2TechData>) {
    data.iter().enumerate().for_each(|(idx, tech)| {
        let db_tech = tech.to_tech(idx as i32);
        if let Err(err) = Tech::insert_with_text(conn, values, &db_tech) {
            eprintln!("Found tech with no text helper, inserting \"enable tech\" instead :  tech.id {}, tech.name_helper {:?}, err {}", db_tech.id, db_tech.name, err);
            let no_text_tech = tech.to_enable_tech(idx as i32);
            Tech::insert(conn, &no_text_tech).unwrap();
        }
    });
}

// Extract game data before hydrating  the database
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
