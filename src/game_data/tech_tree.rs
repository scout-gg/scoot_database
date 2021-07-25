use crate::game_data::aoe2dat::DatFileWrapper;
use crate::model::links::{TechRequiredTech, TechRequiredUnit, UnitRequiredTech, UnitRequiredUnit};
use crate::model::unit::Unit;
use diesel::PgConnection;

impl DatFileWrapper {
    pub fn get_tech_age(&self, tech_id: i16) -> i16 {
        self.0
            .tech_tree
            .tech_tree_ages
            .iter()
            .find(|age| age.techs.contains(&(tech_id as i32)))
            .map(|age| age.id as i16)
            .unwrap_or_else(|| {
                eprintln!("Age not found for tech {}", tech_id);
                0
            })
    }

    pub fn get_unit_age(&self, unit_id: i16) -> i16 {
        self.0
            .tech_tree
            .tech_tree_ages
            .iter()
            .find(|age| age.units.contains(&(unit_id as i32)))
            .map(|age| age.id as i16)
            .unwrap_or_else(|| {
                eprintln!("Age not found for unit {}", unit_id);
                0
            })
    }

    pub fn get_tech_required_tech(&self) -> Vec<TechRequiredTech> {
        self.0
            .tech_tree
            .research_connections
            .iter()
            .flat_map(|tech| {
                tech.techs.iter().map(move |child_tech| TechRequiredTech {
                    tech: *child_tech as i16,
                    required_tech: tech.id as i16,
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
        self.0
            .tech_tree
            .unit_connections
            .iter()
            .filter(|unit| unit.required_research == -1)
            .for_each(|unit| {
                let unit = Unit::by_id(conn, unit.id as i16).unwrap();
                unit.set_root(conn)
                    .expect("Unble to set root value on entity");
            });
    }

    fn get_tech_required_unit(&self) -> Vec<TechRequiredUnit> {
        let units: Vec<(i32, i32)> = self
            .0
            .tech_tree
            .unit_connections
            .iter()
            .map(|unit| (unit.id, unit.units.get(0)))
            .filter(|(_, child)| child.is_some())
            .map(|(unit, child)| (unit, *child.unwrap()))
            .collect();

        units
            .iter()
            .map(|(parent, child)| {
                self.0
                    .tech_tree
                    .unit_connections
                    .iter()
                    .find(|u| &u.id == child)
                    .map(|u| u.required_research as i16)
                    .map(|tech| TechRequiredUnit {
                        tech,
                        required_unit: *parent as i16,
                    })
            })
            .flatten()
            .collect()
    }

    fn get_tech_required_building(&self) -> Vec<TechRequiredUnit> {
        self.0
            .tech_tree
            .building_connections
            .iter()
            .map(|building| {
                building.techs.iter().map(move |tech| TechRequiredUnit {
                    tech: *tech as i16,
                    required_unit: building.id as i16,
                })
            })
            .flatten()
            .collect()
    }

    fn get_buildings_required_building(&self) -> Vec<UnitRequiredUnit> {
        self.0
            .tech_tree
            .building_connections
            .iter()
            .map(|building| {
                self.0
                    .tech_tree
                    .building_connections
                    .iter()
                    .find(|other| other.buildings.contains(&building.id))
                    .map(|building| building.id as i16)
                    .map(|required_building| UnitRequiredUnit {
                        unit: building.id as i16,
                        required_unit: required_building,
                    })
            })
            .flatten()
            .collect()
    }

    fn get_unit_required_building(&self) -> Vec<UnitRequiredUnit> {
        self.0
            .tech_tree
            .unit_connections
            .iter()
            .map(|unit| UnitRequiredUnit {
                unit: unit.id as i16,
                required_unit: unit.upper_building as i16,
            })
            .collect()
    }

    fn get_units_required_tech(&self) -> Vec<UnitRequiredTech> {
        self.0
            .tech_tree
            .unit_connections
            .iter()
            .filter(|unit| unit.required_research != -1)
            .map(|unit| UnitRequiredTech {
                unit: unit.id as i16,
                required_tech: unit.required_research as i16,
            })
            .collect()
    }

    fn get_building_required_tech(&self) -> Vec<UnitRequiredTech> {
        self.0
            .tech_tree
            .building_connections
            .iter()
            .filter(|building| building.enabling_research != -1)
            .map(|building| UnitRequiredTech {
                unit: building.id as i16,
                required_tech: building.enabling_research as i16,
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::game_data::aoe2dat::DAT;
    use aoe_djin::dat::tech_tree::{BuildingConnection, ResearchConnection, UnitConnection};
    use eyre::Result;

    #[test]
    fn a_building_should_have_only_one_parent() -> Result<()> {
        DAT.0
            .tech_tree
            .building_connections
            .iter()
            .for_each(|building| {
                let required_building: Vec<&BuildingConnection> = DAT
                    .0
                    .tech_tree
                    .building_connections
                    .iter()
                    .filter(|other| other.buildings.contains(&(building.id as i32)))
                    .collect();
                assert!(required_building.len() <= 1);
            });

        Ok(())
    }

    #[test]
    fn a_building_should_have_only_one_required_tech() -> Result<()> {
        DAT.0
            .tech_tree
            .building_connections
            .iter()
            .for_each(|building| {
                let required_search: Vec<&ResearchConnection> = DAT
                    .0
                    .tech_tree
                    .research_connections
                    .iter()
                    .filter(|research| ![101, 102, 103, 104].contains(&research.id))
                    .filter(|research| research.buildings.contains(&(building.id as u32)))
                    .collect();

                println!("{:?}", required_search);
                assert!(required_search.len() <= 1);
            });

        Ok(())
    }

    #[test]
    fn a_unit_should_have_only_one_parent() -> Result<()> {
        DAT.0.tech_tree.unit_connections.iter().for_each(|unit| {
            let parent_units: Vec<&UnitConnection> = DAT
                .0
                .tech_tree
                .unit_connections
                .iter()
                .filter(|parent| parent.units.contains(&unit.id))
                .collect();

            assert!(parent_units.len() <= 1);
        });

        Ok(())
    }

    #[test]
    fn a_unit_should_have_only_one_required_tech() -> Result<()> {
        DAT.0.tech_tree.unit_connections.iter().for_each(|unit| {
            let required_tech: Vec<&ResearchConnection> = DAT
                .0
                .tech_tree
                .research_connections
                .iter()
                .filter(|research| ![101, 102, 103, 104].contains(&research.id))
                .filter(|tech| tech.units.contains(&(unit.id as u32)))
                .collect();

            assert!(required_tech.len() <= 1);
        });

        Ok(())
    }

    #[test]
    fn bracer_should_require_bodkin_arrow() -> Result<()> {
        let bracers_id = 201;
        let bodkin_arrow = 200;

        DAT.0.tech_tree.unit_connections.iter().for_each(|unit| {
            let bodkin_arrow = DAT
                .0
                .tech_tree
                .research_connections
                .iter()
                .find(|tech| tech.id == bodkin_arrow)
                .unwrap();

            assert!(bodkin_arrow.techs.contains(&bracers_id));
        });

        Ok(())
    }
}
