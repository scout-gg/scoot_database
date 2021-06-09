use crate::game_data::{help_text_offset, short_help_text_offset};
use crate::model::building::Building;
use crate::model::civilization::Civilization;
use crate::model::tech::{Tech};
use crate::model::unit::Unit;
use serde_json::Value;
use std::collections::HashMap;
use crate::game_data::tech_tree::TechTreeDat;

pub const MILITARY_UNITS: i32 = 70;
pub const BUILDING: i32 = 80;
pub const FOOD: i32 = 0;
pub const WOOD: i32 = 1;
pub const STONE: i32 = 2;
pub const GOLD: i32 = 3;

// From unit_buildings.json extracted via aoe2dat
#[derive(Deserialize, Debug)]
pub struct Aoe2Dat {
    pub units_buildings: HashMap<String, Aoe2DatUnit>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AoeFullDat {
    pub graphics: Value,
    pub effects: Value,
    pub tech_tree: TechTreeDat,
    pub civs: Vec<Aoe2Civ>,
    pub techs: Vec<Ao2TechData>,
    pub player_colours: Value,
    pub file_version: Value,
}

// Raw Tech data from full.json
#[derive(Deserialize, Debug)]
pub struct Ao2TechData {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "LanguageDLLName")]
    pub language_dll_name: i32,
    #[serde(rename = "LanguageDLLHelp")]
    pub language_dll_help: i32,
    #[serde(rename = "IconID")]
    pub icon_id: i32,
    #[serde(rename = "ResearchLocation")]
    pub building: i32,
    #[serde(rename = "ResearchTime")]
    pub research_time: i32,
    #[serde(rename = "ResourceCosts")]
    pub cost: Vec<DatResourceCost>,
}

#[derive(Deserialize, Debug)]
pub struct DatResourceCost {
    #[serde(rename = "Amount")]
    pub amount: i32,
    #[serde(rename = "Flag")]
    pub flag: i32,
    #[serde(rename = "Type")]
    pub resource_type: i32,
}

impl Ao2TechData {
    pub fn to_enable_tech(&self, id: i32) -> Tech {
        Tech {
            id,
            name: -1,
            internal_name: self.name.clone(),
            building_id: self.building_id(),
            research_time: self.research_time,
            wood_cost: self.wood(),
            food_cost: self.food(),
            gold_cost: self.gold(),
            stone_cost: self.stone()
        }
    }

    pub fn to_tech(&self, id: i32) -> Tech {
        let food = self.food();
        let wood = self.wood();
        let gold = self.gold();
        let stone = self.stone();

        let building_id = self.building_id();

        Tech {
            id,
            name: self.language_dll_name,
            internal_name: self.name.clone(),
            building_id,
            research_time: self.research_time,
            wood_cost: wood,
            food_cost: food,
            gold_cost: gold,
            stone_cost: stone,
        }
    }

    fn building_id(&self) -> Option<i32> {
        if self.building <= 0 {
            None
        } else {
            Some(self.building)
        }
    }

    fn food(&self) -> i32 {
        self
            .cost
            .iter()
            .find(|cost| cost.resource_type == FOOD)
            .map(|cost| cost.amount)
            .unwrap_or(0)
    }

    fn wood(&self) -> i32 {
        self
            .cost
            .iter()
            .find(|cost| cost.resource_type == WOOD)
            .map(|cost| cost.amount)
            .unwrap_or(0)
    }

    fn gold(&self) -> i32 {
        self
            .cost
            .iter()
            .find(|cost| cost.resource_type == GOLD)
            .map(|cost| cost.amount)
            .unwrap_or(0)
    }

    fn stone(&self) -> i32 {
        self
            .cost
            .iter()
            .find(|cost| cost.resource_type == STONE)
            .map(|cost| cost.amount)
            .unwrap_or(0)
    }
}

#[derive(Deserialize, Debug)]
pub struct Aoe2DatUnit {
    pub cost: Cost,
    pub attack: i32,
    pub melee_armor: i32,
    pub pierce_armor: i32,
    pub base_id: i32,
    pub help_converter: i32,
    pub language_file_name: i32,
    pub language_file_help: i32,
    pub name: String,
    pub hit_points: i32,
    pub line_of_sight: i32,
    pub garrison_capacity: i32,
    #[serde(rename = "type")]
    pub unit_type: i32,
    pub class: i32,
}

impl Aoe2DatUnit {
    pub fn to_unit(&self) -> Unit {
        assert_eq!(self.unit_type, MILITARY_UNITS);
        let short_help_idx = short_help_text_offset(self.language_file_help);
        let help_idx = help_text_offset(self.language_file_help);

        Unit {
            id: self.base_id,
            internal_name: self.name.clone(),
            wood_cost: self.cost.wood,
            food_cost: self.cost.food,
            gold_cost: self.cost.gold,
            stone_cost: self.cost.stone,
            attack: self.attack,
            melee_armor: self.melee_armor,
            pierce_armor: self.pierce_armor,
            hit_points: self.hit_points,
            line_of_sight: self.line_of_sight,
            garrison_capacity: self.garrison_capacity,
            name: self.language_file_name,
            help_text_short: short_help_idx,
            help_text: help_idx,
        }
    }

    pub fn to_building(&self) -> Building {
        assert_eq!(self.unit_type, BUILDING);
        Building {
            id: self.base_id,
            internal_name: self.name.clone(),
            name: self.language_file_name,
            wood_cost: self.cost.wood,
            food_cost: self.cost.food,
            gold_cost: self.cost.wood,
            stone_cost: self.cost.stone,
            attack: self.attack,
            melee_armor: self.melee_armor,
            pierce_armor: self.pierce_armor,
            hit_points: 0,
            line_of_sight: 0,
            garrison_capacity: 0,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Cost {
    pub wood: i32,
    pub food: i32,
    pub gold: i32,
    pub stone: i32,
}

#[derive(Deserialize, Debug)]
pub struct Aoe2Civ {
    #[serde(rename = "Name")]
    pub name: String,
}

impl Aoe2Civ {
    pub fn to_civ(&self, id: i32) -> Civilization {
        let name_location = 10270 + id;
        Civilization {
            id,
            name: name_location,
        }
    }
}
