use crate::ecs::{ItemStack, MachineKind, Position, RecipeId};
use serde::{Deserialize, Serialize};

/// Identifier for a blueprint.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlueprintId(String);

impl BlueprintId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for BlueprintId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl Default for BlueprintId {
    fn default() -> Self {
        Self(String::new())
    }
}

/// Declarative description of a factory slice.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Blueprint {
    pub id: BlueprintId,
    pub label: String,
    #[serde(default)]
    pub instructions: Vec<BlueprintInstruction>,
}

impl Blueprint {
    pub fn new(id: BlueprintId) -> Self {
        Self {
            id,
            label: String::new(),
            instructions: Vec::new(),
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn push_instruction(&mut self, instruction: BlueprintInstruction) {
        self.instructions.push(instruction);
    }
}

/// An instruction the builder applies to the world when stamping a blueprint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BlueprintInstruction {
    PlaceMachine {
        kind: MachineKind,
        recipe: Option<RecipeId>,
        position: Position,
    },
    SeedInventory {
        position: Position,
        #[serde(default)]
        contents: Vec<ItemStack>,
    },
}
