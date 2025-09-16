use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Grid position within the simulation map.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Logical identifier for an item kind.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ItemId(String);

impl ItemId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ItemId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

/// Logical identifier for a recipe.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecipeId(String);

impl RecipeId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for RecipeId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

/// Stack of items used by recipes and inventories.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ItemStack {
    pub item: ItemId,
    pub quantity: u32,
}

impl ItemStack {
    pub fn new(item: ItemId, quantity: u32) -> Self {
        Self { item, quantity }
    }
}

/// Storage component representing loose items.
#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Inventory {
    slots: HashMap<ItemId, u32>,
    capacity: u32,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            slots: HashMap::new(),
            capacity: 1_024,
        }
    }
}

impl Inventory {
    pub fn with_capacity(capacity: u32) -> Self {
        Self {
            capacity,
            ..Default::default()
        }
    }

    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    pub fn amount(&self, item: &ItemId) -> u32 {
        self.slots.get(item).copied().unwrap_or_default()
    }

    pub fn add_item(&mut self, item: &ItemId, quantity: u32) {
        let entry = self.slots.entry(item.clone()).or_default();
        *entry = entry.saturating_add(quantity);
    }

    pub fn remove_item(&mut self, item: &ItemId, quantity: u32) -> bool {
        let current = self.amount(item);
        if current < quantity {
            return false;
        }
        if let Some(entry) = self.slots.get_mut(item) {
            *entry -= quantity;
            if *entry == 0 {
                self.slots.remove(item);
            }
        }
        true
    }

    pub fn can_fulfill(&self, stacks: &[ItemStack]) -> bool {
        stacks
            .iter()
            .all(|stack| self.amount(&stack.item) >= stack.quantity)
    }

    pub fn consume(&mut self, stacks: &[ItemStack]) -> bool {
        if !self.can_fulfill(stacks) {
            return false;
        }
        for stack in stacks {
            self.remove_item(&stack.item, stack.quantity);
        }
        true
    }

    pub fn produce(&mut self, stacks: &[ItemStack]) {
        for stack in stacks {
            self.add_item(&stack.item, stack.quantity);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ItemId, &u32)> {
        self.slots.iter()
    }
}

/// Machine archetype used for crafting or resource extraction.
#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Machine {
    pub kind: MachineKind,
    pub recipe: Option<RecipeId>,
    pub progress_ticks: u32,
    pub work_required: u32,
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            kind: MachineKind::Assembler,
            recipe: None,
            progress_ticks: 0,
            work_required: 1,
        }
    }
}

impl Machine {
    pub fn new(kind: MachineKind) -> Self {
        Self {
            kind,
            ..Default::default()
        }
    }

    pub fn with_recipe(mut self, recipe: RecipeId) -> Self {
        self.recipe = Some(recipe);
        self
    }
}

/// Enumerates the current machine archetypes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MachineKind {
    Miner,
    Assembler,
    Custom(String),
}

impl Default for MachineKind {
    fn default() -> Self {
        MachineKind::Assembler
    }
}
