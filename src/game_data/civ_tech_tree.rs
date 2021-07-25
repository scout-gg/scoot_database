use crate::model::links::{CivTech, CivUnit};
use diesel::PgConnection;
use eyre::Result;

#[derive(Deserialize, Debug)]
pub struct Ao2CivsTechTree {
    pub civs: Vec<CivTechTreeData>,
}

impl Ao2CivsTechTree {
    pub fn get_unique_units(&self) -> Vec<(i16, i16)> {
        let mut unique_units = vec![];
        self.civs.iter().enumerate().for_each(|(civ_id, civ)| {
            civ.civ_techs_units
                .iter()
                .filter(|unit| unit.node_type == NodeType::UniqueUnit)
                .for_each(|unit| unique_units.push((civ_id as i16, unit.node_id)))
        });
        unique_units
    }

    pub fn get_civ_enabled_entities(&self) -> Vec<CivEnabledTech> {
        self.civs
            .iter()
            .enumerate()
            .map(|(civ_id, civ)| {
                let mut enabled_entities = vec![];

                civ.civ_techs_units
                    .iter()
                    .filter(|unit| unit.node_status != NodeStatus::NotAvailable)
                    .map(ExtractHelper::from)
                    .for_each(|enabled| enabled_entities.push(enabled));

                civ.civ_techs_buildings
                    .iter()
                    .filter(|building| building.node_status != NodeStatus::NotAvailable)
                    .map(ExtractHelper::from)
                    .for_each(|enabled| enabled_entities.push(enabled));

                // Civ start at index 0 in civTechtree.json, Brits id is 1 in db
                let civ_id = (civ_id + 1) as i16;
                CivEnabledTech {
                    civ_id,
                    entity: enabled_entities,
                }
            })
            .collect()
    }
}

#[derive(Deserialize, Debug)]
pub struct CivTechTreeData {
    pub civ_id: String,
    pub civ_techs_buildings: Vec<CivTechTreeBuildingData>,
    pub civ_techs_units: Vec<CivTechTreeUnitData>,
}

pub enum TechThreeNode<'a> {
    Building(&'a CivTechTreeBuildingData),
    Tech(&'a CivTechTreeUnitData),
}

#[derive(Deserialize, Debug)]
pub struct CivTechTreeBuildingData {
    #[serde(rename = "Age ID")]
    pub age_id: u8,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Building ID")]
    pub building_id: i16,
    #[serde(rename = "Picture Index")]
    pub picture_index: i16,
    #[serde(rename = "Node Status")]
    pub node_status: NodeStatus,
    #[serde(rename = "Trigger Tech ID")]
    pub trigger_tech_id: i16,
    #[serde(rename = "Link ID")]
    pub link_id: i16,
    #[serde(rename = "Link Node Type")]
    pub link_type: LinkType,
    #[serde(rename = "Use Type")]
    pub use_type: UseType,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CivTechTreeUnitData {
    #[serde(rename = "Age ID")]
    pub age_id: u8,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Building ID")]
    pub building_id: i16,
    #[serde(rename = "Picture Index")]
    pub picture_index: i16,
    #[serde(rename = "Node ID")]
    pub node_id: i16,
    #[serde(rename = "Node Status")]
    pub node_status: NodeStatus,
    #[serde(rename = "Node Type")]
    pub node_type: NodeType,
    #[serde(rename = "Link ID")]
    pub link_id: i16,
    #[serde(rename = "Link Node Type")]
    pub link_type: LinkType,
    #[serde(rename = "Trigger Tech ID")]
    pub trigger_tech_id: i16,
    #[serde(rename = "Use Type")]
    pub use_type: UseType,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum NodeStatus {
    ResearchAvailable,
    ResearchedCompleted,
    ResearchRequired,
    NotAvailable,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum LinkType {
    Unit,
    BuildingTech,
    UnitUpgrade,
    BuildingNonTech,
    Research,
    UniqueUnit,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum NodeType {
    Unit,
    BuildingTech,
    UnitUpgrade,
    BuildingNonTech,
    Research,
    UniqueUnit,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum UseType {
    Unit,
    Building,
    Tech,
}

pub struct CivEnabledTech {
    pub civ_id: i16,
    pub entity: Vec<ExtractHelper>,
}

pub enum ExtractHelper {
    UnitAndUpgrade(i16, i16),
    UnitOrBuilding(i16),
    Tech(i16),
}

impl From<&CivTechTreeUnitData> for ExtractHelper {
    fn from(unit_data: &CivTechTreeUnitData) -> Self {
        match unit_data.use_type {
            UseType::Unit => {
                if unit_data.trigger_tech_id != -1 {
                    ExtractHelper::UnitAndUpgrade(unit_data.node_id, unit_data.trigger_tech_id)
                } else {
                    ExtractHelper::UnitOrBuilding(unit_data.node_id)
                }
            }
            UseType::Tech => ExtractHelper::Tech(unit_data.node_id),
            UseType::Building => unreachable!(),
        }
    }
}

impl From<&CivTechTreeBuildingData> for ExtractHelper {
    fn from(building_data: &CivTechTreeBuildingData) -> Self {
        match building_data.use_type {
            UseType::Building => ExtractHelper::UnitOrBuilding(building_data.building_id),
            _ => unreachable!(),
        }
    }
}

impl CivEnabledTech {
    pub fn to_db(&self, conn: &PgConnection) -> Result<()> {
        for entity in &self.entity {
            let civ_id = self.civ_id;
            match entity {
                ExtractHelper::UnitAndUpgrade(unit_id, tech_id) => {
                    let _ = CivTech {
                        civ_id,
                        tech_id: *tech_id,
                    }
                    .insert(conn);

                    let _ = CivUnit {
                        civ_id,
                        unit_id: *unit_id,
                    }
                    .insert(conn);
                }
                ExtractHelper::UnitOrBuilding(unit_id) => {
                    let _ = CivUnit {
                        civ_id,
                        unit_id: *unit_id,
                    }
                    .insert(conn);
                }
                ExtractHelper::Tech(tech_id) => {
                    let _ = CivTech {
                        civ_id,
                        tech_id: *tech_id,
                    }
                    .insert(conn);
                }
            }
        }
        Ok(())
    }
}
