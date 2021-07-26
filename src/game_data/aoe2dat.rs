use crate::game_data::{help_text_offset, short_help_text_offset};
use crate::model::civilization::Civilization;
use crate::model::tech::Tech;
use crate::model::unit::Unit;
use aoe_djin::dat;
use aoe_djin::dat::civilization::{Combatant, UnitType};
use aoe_djin::dat::tech::ResourceCostType;
use aoe_djin::dat::{DatFile, ResourceUsageType};
use std::collections::HashSet;

pub struct DatFileWrapper(pub DatFile);

pub struct DatTech<'a>(pub &'a dat::tech::Tech);

pub struct DatUnit<'a>(pub &'a dat::civilization::Unit);

pub struct DatUnitType<'a>(pub &'a dat::civilization::UnitType);

pub struct DatCiv<'a>(pub &'a dat::civilization::Civilization);

lazy_static! {
    pub static ref DAT: DatFileWrapper = {
        let dat = DatFile::from_file("resources/empires2_x2_p1.dat").unwrap();
        DatFileWrapper(dat)
    };
}

impl DatFileWrapper {
    pub fn get_unit_buildings(&self) -> Vec<DatUnit> {
        let mut units = vec![];
        let mut ids = HashSet::new();
        self.0.civilization_table.civilizations[1..] // skip gaia
            .iter()
            .flat_map(|civ| civ.units.iter())
            .for_each(|unit| {
                if ids.insert(unit.id) {
                    units.push(DatUnit(unit))
                }
            });

        units
    }
}

impl DatTech<'_> {
    pub fn to_enable_tech(&self, id: i16) -> Tech {
        Tech {
            id,
            name: None,
            internal_name: self.0.name.content.clone(),
            research_time: self.0.research_time,
            wood_cost: self.cost(ResourceCostType::Food),
            food_cost: self.cost(ResourceCostType::Wood),
            gold_cost: self.cost(ResourceCostType::Gold),
            stone_cost: self.cost(ResourceCostType::Stone),
            age: DAT.get_tech_age(id),
            is_root: false,
        }
    }

    pub fn to_tech(&self, id: i16) -> Tech {
        Tech {
            id,
            name: Some(self.0.language_dll_name as i32),
            internal_name: self.0.name.content.clone(),
            research_time: self.0.research_time,
            wood_cost: self.cost(ResourceCostType::Food),
            food_cost: self.cost(ResourceCostType::Wood),
            gold_cost: self.cost(ResourceCostType::Gold),
            stone_cost: self.cost(ResourceCostType::Stone),
            age: DAT.get_tech_age(id),
            is_root: false,
        }
    }

    fn cost(&self, cost_type: ResourceCostType) -> i16 {
        self.0
            .research_resource_cost
            .iter()
            .find(|cost| cost.resource_type == cost_type)
            .map(|cost| cost.amount)
            .unwrap_or(0)
    }
}

impl DatUnit<'_> {
    pub fn to_unit(&self) -> Unit {
        let short_help_idx = short_help_text_offset(self.0.language_dll_help);
        let help_idx = help_text_offset(self.0.language_dll_help);

        Unit {
            id: self.0.id,
            age: DAT.get_unit_age(self.0.base_id),
            unit_type: DatUnitType(&self.0.unit_type).into(),
            internal_name: self.0.name.content.clone(),
            wood_cost: self.cost(ResourceUsageType::Wood),
            food_cost: self.cost(ResourceUsageType::Food),
            gold_cost: self.cost(ResourceUsageType::Gold),
            stone_cost: self.cost(ResourceUsageType::Stone),
            attack: self.combatant().displayed_attack,
            melee_armor: self.combatant().displayed_melee_armour,
            pierce_armor: self.combatant().base_armor,
            hit_points: self.0.hit_points,
            line_of_sight: self.0.line_of_sight as i16,
            garrison_capacity: self.0.garrison_capacity as i16,
            name: Some(self.0.language_dll_name),
            help_text_short: Some(short_help_idx),
            help_text: Some(help_idx),
            is_root: false,
            belongs_to_civ: None,
        }
    }

    fn cost(&self, cost_type: ResourceUsageType) -> i16 {
        self.0
            .creatable
            .as_ref()
            .expect("Unit should be creatable")
            .resources_costs
            .iter()
            .find(|cost| cost.attribute == cost_type)
            .map(|cost| cost.amount)
            .unwrap_or(0)
    }

    fn combatant(&self) -> &Combatant {
        self.0.type_50.as_ref().expect("Unit should be combatant")
    }
}

impl DatCiv<'_> {
    pub fn to_civ(&self, id: i16) -> Civilization {
        let name_location = 10270 + id;
        Civilization {
            id,
            name: name_location as i32,
        }
    }
}

impl From<DatUnitType<'_>> for i32 {
    fn from(dat: DatUnitType<'_>) -> Self {
        match dat.0 {
            UnitType::EyeCandy => 10,
            UnitType::Trees => 15,
            UnitType::Flag => 20,
            UnitType::Dopl => 25,
            UnitType::DeadFish => 30,
            UnitType::Bird => 40,
            UnitType::Combatant => 50,
            UnitType::Projectile => 60,
            UnitType::Creatable => 70,
            UnitType::Building => 80,
            UnitType::AoeTrees => 90,
        }
    }
}

#[cfg(test)]
mod test {
    use super::DAT;
    use eyre::Result;

    #[test]
    fn should_get_unit_buildings() -> Result<()> {
        assert_eq!(DAT.get_unit_buildings().len(), 1700);
        Ok(())
    }
}
