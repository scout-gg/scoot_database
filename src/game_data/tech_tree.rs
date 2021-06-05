use crate::model::building::Building;
use crate::model::tech::Tech;
use crate::model::tech_tree::{
    CivTechTree, CivTechTreeBuilding, CivTechTreeResearch, CivTechTreeUnit, CivTechTreeUnitUpgrade,
};
use crate::model::unit::Unit;
use diesel::PgConnection;
use eyre::Result;

#[derive(Deserialize, Debug)]
pub struct Ao2CivsTechTree {
    pub civs: Vec<CivTechTreeData>,
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
    pub building_id: i32,
    #[serde(rename = "Picture Index")]
    pub picture_index: i32,
    #[serde(rename = "Node Status")]
    pub node_status: NodeStatus,
    #[serde(rename = "Trigger Tech ID")]
    pub trigger_tech_id: i32,
    #[serde(rename = "Link ID")]
    pub link_id: i32,
    #[serde(rename = "Link Node Type")]
    pub link_type: LinkType,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CivTechTreeUnitData {
    #[serde(rename = "Age ID")]
    pub age_id: u8,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Building ID")]
    pub building_id: i32,
    #[serde(rename = "Picture Index")]
    pub picture_index: i32,
    #[serde(rename = "Node ID")]
    pub node_id: i32,
    #[serde(rename = "Node Status")]
    pub node_status: NodeStatus,
    #[serde(rename = "Node Type")]
    pub node_type: NodeType,
    #[serde(rename = "Link ID")]
    pub link_id: i32,
    #[serde(rename = "Link Node Type")]
    pub link_type: LinkType,
    #[serde(rename = "Trigger Tech ID")]
    pub trigger_tech_id: i32,
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

impl CivTechTreeData {
    pub fn to_tech_tree(&self, id: i32, conn: &PgConnection) -> Result<CivTechTree> {
        let civ_techs_buildings = self
            .civ_techs_buildings
            .iter()
            .map(|tech| tech.to_tech_building(conn, &self).unwrap())
            .collect();

        Ok(CivTechTree {
            civ_id: id,
            civ_base_name: self.civ_id.to_owned(),
            civ_techs_buildings,
        })
    }
}

impl CivTechTreeUnitData {
    pub fn to_tech_unit(
        &self,
        conn: &PgConnection,
        techs: &CivTechTreeData,
    ) -> Result<CivTechTreeUnit> {
        Ok(CivTechTreeUnit {
            age: get_age(self.age_id, conn)?,
            unit: Unit::by_id(conn, self.node_id)?,
            upgrade: self.find_unit_upgrade(techs, conn),
            picture_index: self.picture_index,
        })
    }

    pub fn to_tech_tree_research(&self, conn: &PgConnection) -> Result<CivTechTreeResearch> {
        Ok(CivTechTreeResearch {
            age: get_age(self.age_id, conn)?,
            tech: Tech::by_id(conn, self.node_id)?,
            child: None,
            picture_index: self.picture_index,
        })
    }

    pub fn has_parent(&self) -> bool {
        self.link_id != -1
    }
}

impl CivTechTreeBuildingData {
    pub fn to_tech_building(
        &self,
        conn: &PgConnection,
        techs: &CivTechTreeData,
    ) -> Result<CivTechTreeBuilding> {
        let units = techs
            .civ_techs_units
            .iter()
            .filter(|unit| unit.building_id == self.building_id)
            .filter(|unit| {
                unit.node_type == NodeType::Unit || unit.node_type == NodeType::UniqueUnit
            })
            .map(|unit| unit.to_tech_unit(conn, techs))
            .filter_map(Result::ok)
            .collect();

        let building_researches = techs
            .civ_techs_units
            .iter()
            .filter(|research| self.building_id == research.building_id)
            .filter(|research| research.node_type == NodeType::Research)
            .collect();

        let researches = group_researches_by_child(building_researches, conn)?;

        Ok(CivTechTreeBuilding {
            age: get_age(self.age_id, conn)?,
            picture_index: self.picture_index,
            units,
            name: Building::by_id(conn, self.building_id)?.name,
            researches,
        })
    }
}

fn group_researches_by_child(
    mut researches: Vec<&CivTechTreeUnitData>,
    conn: &PgConnection,
) -> Result<Vec<CivTechTreeResearch>> {
    let mut child_nodes = vec![];
    let mut root_nodes = vec![];

    while let Some(research) = researches.pop() {
        if research.has_parent() {
            child_nodes.push(research);
        } else {
            root_nodes.push(research.to_tech_tree_research(conn)?);
        }
    }

    while !child_nodes.is_empty() {
        child_nodes.retain(|child| {
            let parent = root_nodes
                .iter_mut()
                .find(|parent| parent.is_in_tree(child.link_id));

            if let Some(parent) = parent {
                parent.set_child(child.to_tech_tree_research(conn).unwrap());
                false
            } else {
                true
            }
        });
    }

    Ok(root_nodes)
}

impl CivTechTreeResearch {
    fn is_in_tree(&self, link_id: i32) -> bool {
        if self.tech.id == link_id && self.child.is_none() {
            true
        } else {
            match &self.child {
                Some(child) => child.is_in_tree(link_id),
                None => false,
            }
        }
    }

    fn set_child(&mut self, child: CivTechTreeResearch) {
        if let Some(my_child) = &mut self.child {
            my_child.set_child(child)
        } else {
            self.child = Some(Box::new(child));
        }
    }
}

fn get_age(age_id: u8, conn: &PgConnection) -> Result<String> {
    Ok(if age_id == 1 {
        "Dark Age".to_string()
    } else {
        Tech::by_id(conn, age_id as i32 + 99)?.name
    })
}

impl CivTechTreeUnitData {
    fn find_unit_upgrade<'a>(
        &'a self,
        techs: &'a CivTechTreeData,
        conn: &PgConnection,
    ) -> Option<CivTechTreeUnitUpgrade> {
        techs
            .civ_techs_units
            .iter()
            .filter(|upgrade| upgrade.link_id == self.node_id)
            .filter(|upgrade| upgrade.building_id == self.building_id)
            .find(|upgrade| {
                (upgrade.node_type == NodeType::UnitUpgrade
                    || upgrade.node_type == NodeType::UniqueUnit)
            })
            .map(|upgrade| CivTechTreeUnitUpgrade {
                age: get_age(upgrade.age_id, conn).unwrap(),
                tech: Tech::by_id(conn, upgrade.trigger_tech_id).unwrap(),
                upgrade_to: Box::new(upgrade.to_tech_unit(conn, techs).unwrap()),
                picture_index: 0,
            })
    }
}
