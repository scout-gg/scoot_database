use crate::model::links::{TechRequiredTech, TechRequiredUnit, UnitRequiredTech, UnitRequiredUnit};
use crate::model::unit::Unit;
use diesel::PgConnection;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TechTreeDat {
    pub building_connections: Vec<BuildingConnectionDat>,
    pub research_connections: Vec<ResearchConnectionDat>,
    pub tech_tree_ages: Vec<TechTreeAgeDat>,
    pub unit_connections: Vec<UnitConnectionDat>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BuildingConnectionDat {
    #[serde(rename = "ID")]
    pub building_id: i32,
    pub enabling_research: i32,
    pub buildings: Vec<i32>,
    pub units: Vec<i32>,
    pub techs: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ResearchConnectionDat {
    #[serde(rename = "ID")]
    pub tech_id: i32,
    pub buildings: Vec<i32>,
    pub units: Vec<i32>,
    pub techs: Vec<i32>,
    pub upper_building: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TechTreeAgeDat {
    #[serde(rename = "ID")]
    pub age_id: i32,
    pub buildings: Vec<i32>,
    pub units: Vec<i32>,
    pub techs: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UnitConnectionDat {
    #[serde(rename = "ID")]
    pub unit_id: i32,
    pub required_research: i32,
    pub enabling_research: i32,
    pub upper_building: i32,
    pub units: Vec<i32>,
}

impl TechTreeDat {
    pub fn get_tech_age(&self, tech_id: i32) -> i16 {
        self.tech_tree_ages
            .iter()
            .find(|age| age.techs.contains(&tech_id))
            .map(|age| age.age_id as i16)
            .unwrap_or_else(|| {
                eprintln!("Age not found for tech {}", tech_id);
                0
            })
    }

    pub fn get_unit_age(&self, unit_id: i32) -> i16 {
        self.tech_tree_ages
            .iter()
            .find(|age| age.units.contains(&unit_id))
            .map(|age| age.age_id as i16)
            .unwrap_or_else(|| {
                eprintln!("Age not found for unit {}", unit_id);
                0
            })
    }

    pub fn get_tech_required_tech(&self) -> Vec<TechRequiredTech> {
        self.research_connections
            .iter()
            .flat_map(|tech| {
                tech.techs.iter().map(move |child_tech| TechRequiredTech {
                    tech: *child_tech,
                    required_tech: tech.tech_id,
                })
            })
            .collect()
    }

    pub fn get_unit_techs_link(&self) -> Vec<UnitRequiredTech> {
        let mut unit_tech_links = self.get_building_required_tech();
        unit_tech_links.extend(self.get_units_required_tech());
        unit_tech_links
    }

    pub fn get_unit_unit_link(&self) -> Vec<UnitRequiredUnit> {
        let mut unit_unit_links = self.get_unit_required_building();
        unit_unit_links.extend(self.get_buildings_required_building());
        unit_unit_links
    }

    pub fn get_tech_unit_links(&self) -> Vec<TechRequiredUnit> {
        let mut tech_unit_links = self.get_tech_required_unit();
        tech_unit_links.extend(self.get_tech_required_building());
        tech_unit_links
    }

    pub fn update_root_units(&self, conn: &PgConnection) {
        self.unit_connections
            .iter()
            .filter(|unit| unit.required_research == -1)
            .for_each(|unit| {
                let unit = Unit::by_id(conn, unit.unit_id).unwrap();
                unit.set_root(conn)
                    .expect("Unble to set root value on entity");
            });
    }

    fn get_tech_required_unit(&self) -> Vec<TechRequiredUnit> {
        let units: Vec<(i32, i32)> = self
            .unit_connections
            .iter()
            .map(|unit| (unit.unit_id, unit.units.get(0)))
            .filter(|(_, child)| child.is_some())
            .map(|(unit, child)| (unit, *child.unwrap()))
            .collect();

        units
            .iter()
            .map(|(parent, child)| {
                self.unit_connections
                    .iter()
                    .find(|u| &u.unit_id == child)
                    .map(|u| u.required_research)
                    .map(|tech| TechRequiredUnit {
                        tech,
                        required_unit: *parent,
                    })
            })
            .flatten()
            .collect()
    }

    fn get_tech_required_building(&self) -> Vec<TechRequiredUnit> {
        self.building_connections
            .iter()
            .map(|building| {
                building.techs.iter().map(move |tech| TechRequiredUnit {
                    tech: *tech,
                    required_unit: building.building_id,
                })
            })
            .flatten()
            .collect()
    }

    fn get_buildings_required_building(&self) -> Vec<UnitRequiredUnit> {
        self.building_connections
            .iter()
            .map(|building| {
                self.building_connections
                    .iter()
                    .find(|other| other.buildings.contains(&building.building_id))
                    .map(|building| building.building_id)
                    .map(|required_building| UnitRequiredUnit {
                        unit: building.building_id,
                        required_unit: required_building,
                    })
            })
            .flatten()
            .collect()
    }

    fn get_unit_required_building(&self) -> Vec<UnitRequiredUnit> {
        self.unit_connections
            .iter()
            .map(|unit| UnitRequiredUnit {
                unit: unit.unit_id,
                required_unit: unit.upper_building,
            })
            .collect()
    }

    fn get_units_required_tech(&self) -> Vec<UnitRequiredTech> {
        self.unit_connections
            .iter()
            .filter(|unit| unit.required_research != -1)
            .map(|unit| UnitRequiredTech {
                unit: unit.unit_id,
                required_tech: unit.required_research,
            })
            .collect()
    }

    fn get_building_required_tech(&self) -> Vec<UnitRequiredTech> {
        self.building_connections
            .iter()
            .filter(|building| building.enabling_research != -1)
            .map(|building| UnitRequiredTech {
                unit: building.building_id,
                required_tech: building.enabling_research,
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use eyre::Result;

    use crate::game_data::tech_tree::{
        BuildingConnectionDat, ResearchConnectionDat, TechTreeDat, UnitConnectionDat,
    };

    #[test]
    fn a_building_should_have_only_one_parent() -> Result<()> {
        let data = std::fs::read_to_string("resources/tech.json")?;
        let data: TechTreeDat = serde_json::from_str(&data)?;

        data.building_connections.iter().for_each(|building| {
            let required_building: Vec<&BuildingConnectionDat> = data
                .building_connections
                .iter()
                .filter(|other| other.buildings.contains(&building.building_id))
                .collect();
            assert!(required_building.len() <= 1);
        });

        Ok(())
    }

    #[test]
    fn a_building_should_have_only_one_required_tech() -> Result<()> {
        let data = std::fs::read_to_string("resources/tech.json")?;
        let data: TechTreeDat = serde_json::from_str(&data)?;

        data.building_connections.iter().for_each(|building| {
            let required_search: Vec<&ResearchConnectionDat> = data
                .research_connections
                .iter()
                .filter(|research| ![101, 102, 103, 104].contains(&research.tech_id))
                .filter(|research| research.buildings.contains(&building.building_id))
                .collect();

            println!("{:?}", required_search);
            assert!(required_search.len() <= 1);
        });

        Ok(())
    }

    #[test]
    fn a_unit_should_have_only_one_parent() -> Result<()> {
        let data = std::fs::read_to_string("resources/tech.json")?;
        let data: TechTreeDat = serde_json::from_str(&data)?;

        data.unit_connections.iter().for_each(|unit| {
            let parent_units: Vec<&UnitConnectionDat> = data
                .unit_connections
                .iter()
                .filter(|parent| parent.units.contains(&unit.unit_id))
                .collect();

            assert!(parent_units.len() <= 1);
        });

        Ok(())
    }

    #[test]
    fn a_unit_should_have_only_one_required_tech() -> Result<()> {
        let data = std::fs::read_to_string("resources/tech.json")?;
        let data: TechTreeDat = serde_json::from_str(&data)?;

        data.unit_connections.iter().for_each(|unit| {
            let required_tech: Vec<&ResearchConnectionDat> = data
                .research_connections
                .iter()
                .filter(|research| ![101, 102, 103, 104].contains(&research.tech_id))
                .filter(|tech| tech.units.contains(&unit.unit_id))
                .collect();

            assert!(required_tech.len() <= 1);
        });

        Ok(())
    }

    #[test]
    fn bracer_should_require_bodkin_arrow() -> Result<()> {
        let bracers_id = 201;
        let bodkin_arrow = 200;
        let data = std::fs::read_to_string("resources/tech.json")?;
        let data: TechTreeDat = serde_json::from_str(&data)?;

        data.unit_connections.iter().for_each(|unit| {
            let bodkin_arrow = data
                .research_connections
                .iter()
                .find(|tech| tech.tech_id == bodkin_arrow)
                .unwrap();

            assert!(bodkin_arrow.techs.contains(&bracers_id));
        });

        Ok(())
    }
}
