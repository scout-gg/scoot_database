use crate::game_data::key_value::Ao2KeyValues;
use crate::game_data::{help_text_offset, short_help_text_offset};
use crate::model::building::Building;
use crate::model::civilization::Civilization;
use crate::model::tech::Tech;
use crate::model::unit::Unit;
use serde_json::Value;
use std::collections::HashMap;

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
    pub tech_tree: Value,
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
    pub fn to_tech(&self, id: i32, values: &Ao2KeyValues) -> Tech {
        let food = self
            .cost
            .iter()
            .find(|cost| cost.resource_type == FOOD)
            .map(|cost| cost.amount)
            .unwrap_or(0);

        let wood = self
            .cost
            .iter()
            .find(|cost| cost.resource_type == WOOD)
            .map(|cost| cost.amount)
            .unwrap_or(0);

        let gold = self
            .cost
            .iter()
            .find(|cost| cost.resource_type == GOLD)
            .map(|cost| cost.amount)
            .unwrap_or(0);

        let stone = self
            .cost
            .iter()
            .find(|cost| cost.resource_type == STONE)
            .map(|cost| cost.amount)
            .unwrap_or(0);

        let building_id = if self.building <= 0 {
            None
        } else {
            Some(self.building)
        };

        Tech {
            id,
            name: values
                .en
                .get(&self.language_dll_name)
                .cloned()
                .unwrap_or_else(|| self.name.clone()),
            building_id,
            research_time: self.research_time,
            name_fr: values.fr.get(&self.language_dll_name).cloned(),
            name_br: values.br.get(&self.language_dll_name).cloned(),
            name_de: values.de.get(&self.language_dll_name).cloned(),
            name_es: values.es.get(&self.language_dll_name).cloned(),
            name_hi: values.hi.get(&self.language_dll_name).cloned(),
            name_it: values.it.get(&self.language_dll_name).cloned(),
            name_jp: values.jp.get(&self.language_dll_name).cloned(),
            name_ko: values.ko.get(&self.language_dll_name).cloned(),
            name_ms: values.ms.get(&self.language_dll_name).cloned(),
            name_mx: values.mx.get(&self.language_dll_name).cloned(),
            name_ru: values.ru.get(&self.language_dll_name).cloned(),
            name_tr: values.tr.get(&self.language_dll_name).cloned(),
            name_tw: values.tw.get(&self.language_dll_name).cloned(),
            name_vi: values.vi.get(&self.language_dll_name).cloned(),
            name_zh: values.zh.get(&self.language_dll_name).cloned(),
            wood_cost: wood,
            food_cost: food,
            gold_cost: gold,
            stone_cost: stone,
        }
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
        let short_help_idx = short_help_text_offset(self.language_file_help);
        let help_idx = help_text_offset(self.language_file_help);

        name.map(|name| Unit {
            id: self.base_id,
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

            name: name.to_owned(),
            name_fr: value.fr.get(&self.language_file_name).unwrap().to_owned(),
            name_br: value.br.get(&self.language_file_name).unwrap().to_owned(),
            name_de: value.de.get(&self.language_file_name).unwrap().to_owned(),
            name_es: value.es.get(&self.language_file_name).unwrap().to_owned(),
            name_hi: value.hi.get(&self.language_file_name).unwrap().to_owned(),
            name_it: value.it.get(&self.language_file_name).unwrap().to_owned(),
            name_jp: value.jp.get(&self.language_file_name).unwrap().to_owned(),
            name_ko: value.ko.get(&self.language_file_name).unwrap().to_owned(),
            name_ms: value.ms.get(&self.language_file_name).unwrap().to_owned(),
            name_mx: value.mx.get(&self.language_file_name).unwrap().to_owned(),
            name_ru: value.ru.get(&self.language_file_name).unwrap().to_owned(),
            name_tr: value.tr.get(&self.language_file_name).unwrap().to_owned(),
            name_tw: value.tw.get(&self.language_file_name).unwrap().to_owned(),
            name_vi: value.vi.get(&self.language_file_name).unwrap().to_owned(),
            name_zh: value.zh.get(&self.language_file_name).unwrap().to_owned(),

            help_text_short: value.en.get(&short_help_idx).cloned(),
            help_text_short_fr: value.fr.get(&short_help_idx).cloned(),
            help_text_short_br: value.br.get(&short_help_idx).cloned(),
            help_text_short_de: value.de.get(&short_help_idx).cloned(),
            help_text_short_es: value.es.get(&short_help_idx).cloned(),
            help_text_short_hi: value.hi.get(&short_help_idx).cloned(),
            help_text_short_it: value.it.get(&short_help_idx).cloned(),
            help_text_short_jp: value.jp.get(&short_help_idx).cloned(),
            help_text_short_ko: value.ko.get(&short_help_idx).cloned(),
            help_text_short_ms: value.ms.get(&short_help_idx).cloned(),
            help_text_short_mx: value.mx.get(&short_help_idx).cloned(),
            help_text_short_ru: value.ru.get(&short_help_idx).cloned(),
            help_text_short_tr: value.tr.get(&short_help_idx).cloned(),
            help_text_short_tw: value.tw.get(&short_help_idx).cloned(),
            help_text_short_vi: value.vi.get(&short_help_idx).cloned(),
            help_text_short_zh: value.zh.get(&short_help_idx).cloned(),

            help_text: value.en.get(&help_idx).cloned(),
            help_text_fr: value.fr.get(&help_idx).cloned(),
            help_text_br: value.br.get(&help_idx).cloned(),
            help_text_de: value.de.get(&help_idx).cloned(),
            help_text_es: value.es.get(&help_idx).cloned(),
            help_text_hi: value.hi.get(&help_idx).cloned(),
            help_text_it: value.it.get(&help_idx).cloned(),
            help_text_jp: value.jp.get(&help_idx).cloned(),
            help_text_ko: value.ko.get(&help_idx).cloned(),
            help_text_ms: value.ms.get(&help_idx).cloned(),
            help_text_mx: value.mx.get(&help_idx).cloned(),
            help_text_ru: value.ru.get(&help_idx).cloned(),
            help_text_tr: value.tr.get(&help_idx).cloned(),
            help_text_tw: value.tw.get(&help_idx).cloned(),
            help_text_vi: value.vi.get(&help_idx).cloned(),
            help_text_zh: value.zh.get(&help_idx).cloned(),
        })
    }

    pub fn to_building(&self, value: &Ao2KeyValues) -> Option<Building> {
        assert_eq!(self.unit_type, BUILDING);
        let name = value.en.get(&self.language_file_name);
        name.map(|name| Building {
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
            name_br: value.br.get(&self.language_file_name).unwrap().to_owned(),
            name_de: value.de.get(&self.language_file_name).unwrap().to_owned(),
            name_es: value.es.get(&self.language_file_name).unwrap().to_owned(),
            name_hi: value.hi.get(&self.language_file_name).unwrap().to_owned(),
            name_it: value.it.get(&self.language_file_name).unwrap().to_owned(),
            name_jp: value.jp.get(&self.language_file_name).unwrap().to_owned(),
            name_ko: value.ko.get(&self.language_file_name).unwrap().to_owned(),
            name_ms: value.ms.get(&self.language_file_name).unwrap().to_owned(),
            name_mx: value.mx.get(&self.language_file_name).unwrap().to_owned(),
            name_ru: value.ru.get(&self.language_file_name).unwrap().to_owned(),
            name_tr: value.tr.get(&self.language_file_name).unwrap().to_owned(),
            name_tw: value.tw.get(&self.language_file_name).unwrap().to_owned(),
            name_vi: value.vi.get(&self.language_file_name).unwrap().to_owned(),
            name_zh: value.zh.get(&self.language_file_name).unwrap().to_owned(),
            hit_points: 0,
            line_of_sight: 0,
            garrison_capacity: 0,
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
pub struct Aoe2Civ {
    #[serde(rename = "Name")]
    pub name: String,
}

impl Aoe2Civ {
    pub fn to_civ(&self, id: i32, value: &Ao2KeyValues) -> Civilization {
        let name_location = 10270 + id;
        Civilization {
            id,
            name: value.en.get(&name_location).unwrap().to_owned(),
            name_fr: value.fr.get(&name_location).unwrap().to_owned(),
            name_br: value.br.get(&name_location).unwrap().to_owned(),
            name_de: value.de.get(&name_location).unwrap().to_owned(),
            name_es: value.es.get(&name_location).unwrap().to_owned(),
            name_hi: value.hi.get(&name_location).unwrap().to_owned(),
            name_it: value.it.get(&name_location).unwrap().to_owned(),
            name_jp: value.jp.get(&name_location).unwrap().to_owned(),
            name_ko: value.ko.get(&name_location).unwrap().to_owned(),
            name_ms: value.ms.get(&name_location).unwrap().to_owned(),
            name_mx: value.mx.get(&name_location).unwrap().to_owned(),
            name_ru: value.ru.get(&name_location).unwrap().to_owned(),
            name_tr: value.tr.get(&name_location).unwrap().to_owned(),
            name_tw: value.tw.get(&name_location).unwrap().to_owned(),
            name_vi: value.vi.get(&name_location).unwrap().to_owned(),
            name_zh: value.zh.get(&name_location).unwrap().to_owned(),
        }
    }
}
