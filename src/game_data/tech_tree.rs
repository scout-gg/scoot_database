use crate::model::tech_tree_unit::{TechTreeUnit};
use crate::model::tech_tree_tech::TechTreeResearch;
use crate::model::tech_tree_building::TechTreeBuilding;
use std::convert::TryInto;

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
    pub fn get_buildings(&self) -> Vec<TechTreeBuilding> {
        self.building_connections.iter().map(|building| {
            let required_building = self.building_connections.iter()
                .find(|other| other.buildings.contains(&building.building_id))
                .map(|building| building.building_id);

            let required_tech: Vec<i32> = self.research_connections.iter()
                .filter(|research| filter_ages_tech(&research.tech_id))
                .filter(|research| research.buildings.contains(&building.building_id))
                .map(|research| research.tech_id)
                .collect();

            assert!(required_tech.len() <= 1);
            let required_tech = required_tech.first().cloned();

            let age = self.tech_tree_ages.iter()
                .find(|age| age.buildings.contains(&building.building_id))
                .map(|age| age.age_id)
                .unwrap();

            let enabling_research = if building.enabling_research == -1 {
                None
            } else {
                Some(building.enabling_research)
            };

            TechTreeBuilding {
                id: building.building_id,
                enabling_research,
                required_building,
                required_tech,
                age: age.try_into().unwrap(),
            }
        }).collect()
    }

    pub fn get_units(&self) -> Vec<TechTreeUnit> {
        self.unit_connections.iter().map(|unit| {
            let age = self.tech_tree_ages.iter()
                .find(|age| age.units.contains(&unit.unit_id)).map(|age| age.age_id);

            match age {
                None => {
                    println!("Skipping unit with no age {}", unit.unit_id);
                    None
                }
                Some(age_id) => {
                    let parent_unit = self.unit_connections.iter()
                        .find(|parent| parent.units.contains(&unit.unit_id))
                        .map(|parent| parent.unit_id);

                    let required_tech = self.research_connections.iter()
                        .filter(|tech| filter_ages_tech(&tech.tech_id))
                        .find(|tech| tech.units.contains(&unit.unit_id))
                        .map(|tech| tech.tech_id);

                    let enabling_research = if unit.enabling_research == -1 {
                        None
                    } else {
                        Some(unit.enabling_research)
                    };

                    Some(TechTreeUnit {
                        age: age_id.try_into().unwrap(),
                        id: unit.unit_id,
                        required_tech,
                        upper_building: unit.upper_building,
                        parent_unit,
                        enabling_research,
                    })
                }
            }
        })
            .flatten()
            .collect()
    }

    pub fn get_techs(&self) -> Vec<TechTreeResearch> {
        self.research_connections.iter().map(|tech| {
            let age = self.tech_tree_ages.iter()
                .find(|age| age.techs.contains(&tech.tech_id)).map(|age| age.age_id);

            match age {
                None => {
                    println!("Skipping tech tree tech with no age {}", tech.tech_id);
                    None
                }
                Some(age_id) => {
                    let tech_required_tech = self.research_connections.iter()
                        .filter(|parent_tech| filter_ages_tech(&parent_tech.tech_id))
                        .find(|parent_tech| parent_tech.techs.contains(&tech.tech_id))
                        .map(|parent_tech| parent_tech.tech_id);

                    Some(TechTreeResearch {
                        age: age_id.try_into().unwrap(),
                        id: tech.tech_id,
                        required_tech: tech_required_tech,
                        upper_building: tech.upper_building,
                    })
                }
            }
        })
            .flatten()
            .collect()
    }
}

fn filter_ages_tech(id: &i32) -> bool {
    ![101, 102, 103, 104].contains(id)
}


#[cfg(test)]
mod test {
    use crate::game_data::tech_tree::{BuildingConnectionDat, TechTreeDat, ResearchConnectionDat, UnitConnectionDat};

    use eyre::Result;

    #[test]
    fn a_building_should_have_only_one_parent() -> Result<()> {
        let data = std::fs::read_to_string("resources/tech.json")?;
        let data: TechTreeDat = serde_json::from_str(&data)?;

        data.building_connections.iter().for_each(|building| {
            let required_building: Vec<&BuildingConnectionDat> = data.building_connections.iter()
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
            let required_search: Vec<&ResearchConnectionDat> = data.research_connections.iter()
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
            let parent_units: Vec<&UnitConnectionDat> = data.unit_connections.iter()
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
            let required_tech: Vec<&ResearchConnectionDat> = data.research_connections.iter()
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
            let bodkin_arrow = data.research_connections.iter()
                .find(|tech| tech.tech_id == bodkin_arrow)
                .unwrap();

            assert!(bodkin_arrow.techs.contains(&bracers_id));
        });

        Ok(())
    }
}