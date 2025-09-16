use anyhow::{anyhow, Result};
use clap::Parser;
use std::path::PathBuf;
use womporio_core::blueprint::{Blueprint, BlueprintInstruction};

#[derive(Parser, Debug)]
#[command(name = "blueprint-linter", about = "Validate Womporio blueprint files")]
struct Args {
    /// Path to a TOML or JSON blueprint document.
    #[arg(value_name = "BLUEPRINT")]
    input: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let raw = std::fs::read_to_string(&args.input)?;
    let blueprint: Blueprint = if args
        .input
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
    {
        serde_json::from_str(&raw)?
    } else {
        toml::from_str(&raw)?
    };

    validate_blueprint(&blueprint)?;
    println!(
        "Blueprint '{}' is valid ({} instructions).",
        blueprint.id.as_str(),
        blueprint.instructions.len()
    );
    Ok(())
}

fn validate_blueprint(blueprint: &Blueprint) -> Result<()> {
    if blueprint.instructions.is_empty() {
        return Err(anyhow!(
            "blueprint '{}' contains no instructions",
            blueprint.id.as_str()
        ));
    }

    for (index, instruction) in blueprint.instructions.iter().enumerate() {
        match instruction {
            BlueprintInstruction::PlaceMachine { position, .. } => {
                if !(-512..=512).contains(&position.x) || !(-512..=512).contains(&position.y) {
                    return Err(anyhow!(
                        "instruction {} places a machine outside the supported bounds",
                        index
                    ));
                }
            }
            BlueprintInstruction::SeedInventory { contents, .. } => {
                if contents.is_empty() {
                    return Err(anyhow!(
                        "instruction {} seeds an empty inventory; drop the instruction or add items",
                        index
                    ));
                }
            }
        }
    }

    Ok(())
}
