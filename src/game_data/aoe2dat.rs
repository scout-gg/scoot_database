use crate::game_data::key_value::Ao2KeyValues;
use crate::model::tech::Tech;
use std::collections::HashMap;
use crate::model::unit::Unit;
use std::any::Any;
use crate::model::building::Building;

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
    pub fn to_tech(&self, id: i32, values: &Ao2KeyValues) -> Option<Tech> {
        let name = values.en.get(&self.language_dll_name);
        name.map(|name| {
            let food = self.cost.iter().find(|cost| cost.resource_type == FOOD)
                .map(|cost| cost.amount)
                .unwrap_or(0);

            let wood = self.cost.iter().find(|cost| cost.resource_type == WOOD)
                .map(|cost| cost.amount)
                .unwrap_or(0);

            let gold = self.cost.iter().find(|cost| cost.resource_type == GOLD)
                .map(|cost| cost.amount)
                .unwrap_or(0);

            let stone = self.cost.iter().find(|cost| cost.resource_type == STONE)
                .map(|cost| cost.amount)
                .unwrap_or(0);

            Tech {
                id,
                name: name.to_owned(),
                building_id: self.building,
                research_time: self.research_time,
                name_fr: values.fr.get(&self.language_dll_name).unwrap().to_owned(),
                wood_cost: wood,
                food_cost: food,
                gold_cost: gold,
                stone_cost: stone,
            }
        })
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
    pub fn to_unit(&self, value: &Ao2KeyValues) -> Option<Unit> {
        assert_eq!(self.unit_type, MILITARY_UNITS);
        let name = value.en.get(&self.language_file_name);
        name.map(|name| {
            Unit {
                id: self.base_id,
                wood_cost: self.cost.wood,
                food_cost: self.cost.food,
                gold_cost: self.cost.wood,
                stone_cost: self.cost.stone,
                attack: self.attack,
                melee_armor: self.melee_armor,
                pierce_armor: self.pierce_armor,
                name: name.to_owned(),
                name_fr: value.fr.get(&self.language_file_name).unwrap().to_owned(),
                hit_points: 0,
                line_of_sight: 0,
                garrison_capacity: 0,
            }
        })
    }

    pub fn to_building(&self, value: &Ao2KeyValues) -> Option<Building> {
        assert_eq!(self.unit_type, BUILDING);
        let name = value.en.get(&self.language_file_name);
        name.map(|name| {
            Building {
                id: self.base_id,
                wood_cost: self.cost.wood,
                food_cost: self.cost.food,
                gold_cost: self.cost.wood,
                stone_cost: self.cost.stone,
                attack: self.attack,
                melee_armor: self.melee_armor,
                pierce_armor: self.pierce_armor,
                name: name.to_owned(),
                name_fr: value.fr.get(&self.language_file_name).unwrap().to_owned(),
                hit_points: 0,
                line_of_sight: 0,
                garrison_capacity: 0,
            }
        })
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
pub struct CivData {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "TechTreeID")]
    pub tech_three_id: i32,
}

