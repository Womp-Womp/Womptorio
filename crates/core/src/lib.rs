//! Core deterministic simulation for Womporio.

pub mod blueprint;
pub mod ecs;
pub mod logistics;
pub mod save;
pub mod ships;
pub mod tick;

pub use tick::{SimConfig, SimWorld};

/// Small prelude intended for mods and auxiliary crates.
pub mod prelude {
    pub use crate::blueprint::{Blueprint, BlueprintId};
    pub use crate::ecs::{Inventory, ItemId, ItemStack, Machine, MachineKind, Position, RecipeId};
    pub use crate::logistics::{ItemCatalog, Recipe, RecipeBook};
    pub use crate::tick::{RandomState, SimConfig, SimWorld, TimeState};
    pub use bevy_ecs::prelude::*;
}
