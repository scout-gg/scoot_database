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
