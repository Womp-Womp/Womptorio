use crate::ecs::{Inventory, ItemId, ItemStack, Machine, MachineKind, RecipeId};
use crate::tick::TimeState;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::Schedule;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Definition for an item available in the simulation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemDefinition {
    pub id: ItemId,
    pub display: String,
}

/// Catalog of all registered items.
#[derive(Resource, Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(default)]
pub struct ItemCatalog {
    items: HashMap<ItemId, ItemDefinition>,
}

impl ItemCatalog {
    pub fn with_base_items() -> Self {
        let mut catalog = Self::default();
        catalog.register(ItemDefinition {
            id: ItemId::from("iron-ore"),
            display: "Iron Ore".to_string(),
        });
        catalog.register(ItemDefinition {
            id: ItemId::from("iron-plate"),
            display: "Iron Plate".to_string(),
        });
        catalog.register(ItemDefinition {
            id: ItemId::from("gear"),
            display: "Gear".to_string(),
        });
        catalog
    }

    pub fn register(&mut self, definition: ItemDefinition) {
        self.items.insert(definition.id.clone(), definition);
    }

    pub fn get(&self, id: &ItemId) -> Option<&ItemDefinition> {
        self.items.get(id)
    }
}

/// Crafting recipe used by machines.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Recipe {
    pub id: RecipeId,
    #[serde(default)]
    pub inputs: Vec<ItemStack>,
    #[serde(default)]
    pub outputs: Vec<ItemStack>,
    pub duration: u32,
}

impl Recipe {
    pub fn new(id: RecipeId, duration: u32) -> Self {
        Self {
            id,
            inputs: Vec::new(),
            outputs: Vec::new(),
            duration,
        }
    }

    pub fn with_inputs(mut self, inputs: Vec<ItemStack>) -> Self {
        self.inputs = inputs;
        self
    }

    pub fn with_outputs(mut self, outputs: Vec<ItemStack>) -> Self {
        self.outputs = outputs;
        self
    }
}

/// Registry of available recipes.
#[derive(Resource, Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(default)]
pub struct RecipeBook {
    recipes: HashMap<RecipeId, Recipe>,
}

impl RecipeBook {
    pub fn with_base_recipes() -> Self {
        let mut book = Self::default();
        book.register(
            Recipe::new(RecipeId::from("smelt-iron"), 4)
                .with_inputs(vec![ItemStack::new(ItemId::from("iron-ore"), 1)])
                .with_outputs(vec![ItemStack::new(ItemId::from("iron-plate"), 1)]),
        );
        book.register(
            Recipe::new(RecipeId::from("gear"), 2)
                .with_inputs(vec![ItemStack::new(ItemId::from("iron-plate"), 2)])
                .with_outputs(vec![ItemStack::new(ItemId::from("gear"), 1)]),
        );
        book
    }

    pub fn register(&mut self, recipe: Recipe) {
        self.recipes.insert(recipe.id.clone(), recipe);
    }

    pub fn get(&self, id: &RecipeId) -> Option<&Recipe> {
        self.recipes.get(id)
    }
}

/// Insert baseline resources for the logistics simulation.
pub fn init_resources(world: &mut World) {
    world.insert_resource(ItemCatalog::with_base_items());
    world.insert_resource(RecipeBook::with_base_recipes());
}

/// Seed the world with a minimal factory footprint.
pub fn seed_world(world: &mut World) {
    let entity = world
        .spawn((
            Machine::new(MachineKind::Miner).with_recipe(RecipeId::from("smelt-iron")),
            Inventory::with_capacity(128),
            crate::ecs::Position::new(0, 0),
        ))
        .id();

    if let Some(mut inventory) = world.entity_mut(entity).get_mut::<Inventory>() {
        let ore = ItemId::from("iron-ore");
        inventory.add_item(&ore, 8);
    }
}

/// Register deterministic logistics systems with the global schedule.
pub fn configure_schedule(schedule: &mut Schedule) {
    schedule.add_systems(progress_machines);
}

/// Progress active machine recipes one tick at a time.
fn progress_machines(
    mut machines: Query<(&mut Machine, &mut Inventory)>,
    recipes: Res<RecipeBook>,
    time: Res<TimeState>,
) {
    let _tick = time.tick;
    for (mut machine, mut inventory) in &mut machines {
        let recipe_id = match machine.recipe.clone() {
            Some(id) => id,
            None => continue,
        };
        let Some(recipe) = recipes.get(&recipe_id) else {
            continue;
        };

        if !inventory.can_fulfill(&recipe.inputs) {
            machine.progress_ticks = 0;
            continue;
        }

        machine.progress_ticks += 1;
        if machine.progress_ticks >= recipe.duration {
            inventory.consume(&recipe.inputs);
            inventory.produce(&recipe.outputs);
            machine.progress_ticks = 0;
        }
    }
}
