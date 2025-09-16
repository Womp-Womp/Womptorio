use crate::tick::TimeState;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::Schedule;
use serde::{Deserialize, Serialize};

/// Definition for a hull that can be constructed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShipDesign {
    pub name: String,
    pub hull_class: HullClass,
    pub tonnage: u32,
}

impl ShipDesign {
    pub fn new(name: impl Into<String>, hull_class: HullClass, tonnage: u32) -> Self {
        Self {
            name: name.into(),
            hull_class,
            tonnage,
        }
    }
}

/// Enumeration of stock hull sizes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum HullClass {
    Corvette,
    Frigate,
    Destroyer,
    Custom(String),
}

/// Registry of ship designs sourced from mods.
#[derive(Resource, Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(default)]
pub struct ShipCatalog {
    #[serde(default)]
    pub designs: Vec<ShipDesign>,
}

/// Component representing a single shipyard entity.
#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Shipyard {
    pub name: String,
    pub build_rate: f32,
    pub progress: f32,
    pub active_project: Option<String>,
}

impl Default for Shipyard {
    fn default() -> Self {
        Self {
            name: "Orbital Shipyard".to_string(),
            build_rate: 5.0,
            progress: 0.0,
            active_project: None,
        }
    }
}

pub fn init_resources(world: &mut World) {
    world.insert_resource(ShipCatalog::default());
}

pub fn seed_world(world: &mut World) {
    world.spawn((Shipyard::default(),));
}

pub fn configure_schedule(schedule: &mut Schedule) {
    schedule.add_systems(progress_shipyards);
}

fn progress_shipyards(mut yards: Query<&mut Shipyard>, time: Res<TimeState>) {
    for mut yard in yards.iter_mut() {
        if yard.active_project.is_some() {
            yard.progress += yard.build_rate * time.fixed_time_step as f32;
            if yard.progress >= 100.0 {
                yard.progress = 0.0;
                yard.active_project = None;
            }
        }
    }
}
