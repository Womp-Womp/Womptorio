use crate::ecs::{Inventory, Machine, Position};
use crate::tick::TimeState;
use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

/// Serialized snapshot of the world state for determinism checks and saves.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(default)]
pub struct WorldSnapshot {
    pub tick: u64,
    pub machines: Vec<MachineSnapshot>,
}

impl WorldSnapshot {
    pub fn capture(world: &World) -> Self {
        let mut snapshot = WorldSnapshot::default();
        if let Some(time) = world.get_resource::<TimeState>() {
            snapshot.tick = time.tick;
        }

        for entity in world.iter_entities() {
            let Some(machine) = entity.get::<Machine>() else {
                continue;
            };
            let inventory = entity.get::<Inventory>().cloned().unwrap_or_default();
            let position = entity.get::<Position>().map(|p| (p.x, p.y));
            snapshot.machines.push(MachineSnapshot {
                position,
                machine: machine.clone(),
                inventory,
            });
        }

        snapshot
            .machines
            .sort_by(|a, b| a.position.cmp(&b.position));
        snapshot
    }
}

/// Snapshot representation for a single machine entity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct MachineSnapshot {
    pub position: Option<(i32, i32)>,
    pub machine: Machine,
    pub inventory: Inventory,
}

impl Default for MachineSnapshot {
    fn default() -> Self {
        Self {
            position: None,
            machine: Machine::default(),
            inventory: Inventory::default(),
        }
    }
}
