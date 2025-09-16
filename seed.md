# Womporio — First Cut

## Direct Answer

**Pick Rust + Cargo workspaces with an ECS core (Bevy ECS) + ASCII renderer.**

* **Runtime:** Rust `1.89` (on your box now).
* **Core:** `bevy_ecs` (just ECS crate, not full Bevy engine) for a **deterministic, headless-friendly** simulation.
* **Rendering:** swapable: start with **ASCII** using `bracket-lib` (aka RLTK) for a terminal/SDL view; later, you can plug in a Bevy/GPU view.
* **Modding:** data-driven via **TOML/JSON** + **Lua** scripting (`mlua`) behind a stable API. Treat “base game” as a mod loaded from `/mods/base`.
* **Build layout:** Cargo **workspace**: `core/` (sim), `ascii-client/`, `server/`, `tools/`, `mods/`.
* **Determinism:** fixed-tick loop, integer time, seeded RNG (PCG).
  This satisfies: runs headless remotely (no GUI), runs locally on laptop, ASCII-first like Dwarf Fortress, Factorio-scale perf, Aurora-style depth, and open-source friendliness.

---

## Brief Reasoning

* **Performance & determinism:** ECS + Rust gives predictable perf and tight memory; fixed-tick avoids desync and simplifies headless testing. (Factorio and DF both rely on strict tick logic.)
* **Headless first:** separating **core sim** from **clients** lets you CI the game logic with no windowing; you can even fuzz or property-test it.
* **Moddability:** data tables + Lua hooks minimize recompiles; making **base game a mod** enforces API boundaries from day one.
* **Portability:** Rust produces tiny, dependency-light binaries that run fine on a laptop; ASCII via bracket-lib is battle-tested in roguelikes.

---

## Proposed Repo Seed (drop-in)

**Directory layout**

```
womporio/
├─ Cargo.toml                 # workspace
├─ README.md
├─ PRD.md
├─ LICENSE
├─ crates/
│  ├─ core/                   # pure sim, no rendering
│  │  ├─ src/
│  │  │  ├─ lib.rs
│  │  │  ├─ ecs.rs            # components/systems prelude
│  │  │  ├─ tick.rs           # fixed-step loop
│  │  │  ├─ blueprint.rs
│  │  │  ├─ logistics.rs
│  │  │  ├─ ships.rs
│  │  │  └─ save.rs
│  │  └─ Cargo.toml
│  ├─ server/                 # headless exe driving core
│  │  ├─ src/main.rs
│  │  └─ Cargo.toml
│  ├─ ascii-client/           # ASCII view + input
│  │  ├─ src/main.rs
│  │  └─ Cargo.toml
│  └─ api/                    # mod API + Lua bridge
│     ├─ src/lib.rs
│     └─ Cargo.toml
├─ mods/
│  ├─ base/                   # “vanilla game” implemented as a mod
│  │  ├─ mod.toml
│  │  ├─ data/                # items, entities, recipes, tech trees
│  │  └─ scripts/             # Lua hooks
│  └─ examples/
└─ tools/
   └─ blueprint-linter/
      ├─ src/main.rs
      └─ Cargo.toml
```

**Workspace manifest**

```toml
# Cargo.toml (workspace root)
[workspace]
members = [
  "crates/core",
  "crates/server",
  "crates/ascii-client",
  "crates/api",
  "tools/blueprint-linter"
]
resolver = "2"
```

**Core crate basics**

```toml
# crates/core/Cargo.toml
[package]
name = "womporio-core"
version = "0.1.0"
edition = "2021"

[dependencies]
bevy_ecs = "0.14"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"
rand = "0.8"
rand_pcg = "0.3"
anyhow = "1"
thiserror = "1"
```

```rust
// crates/core/src/lib.rs
pub mod ecs;
pub mod tick;
pub mod logistics;
pub mod ships;
pub mod blueprint;
pub mod save;

pub use tick::{SimConfig, SimWorld};

// Re-exports to give mods a small surface area later
pub mod prelude {
    pub use bevy_ecs::prelude::*;
}
```

```rust
// crates/core/src/tick.rs
use bevy_ecs::prelude::*;
use rand_pcg::Pcg64Mcg;

pub struct SimConfig {
    pub seed: u64,
    pub tick_ms: u64, // fixed timestep
}

pub struct SimWorld {
    pub world: World,
    pub schedule: Schedule,
    pub rng: Pcg64Mcg,
    pub tick: u64,
    pub cfg: SimConfig,
}

impl SimWorld {
    pub fn new(cfg: SimConfig) -> Self {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        // TODO: add core systems in deterministic order
        Self { world, schedule, rng: Pcg64Mcg::new(cfg.seed), tick: 0, cfg }
    }
    pub fn step(&mut self) {
        self.schedule.run(&mut self.world);
        self.tick += 1;
    }
}
```

**ASCII client stub**

```toml
# crates/ascii-client/Cargo.toml
[package]
name = "womporio-ascii"
version = "0.1.0"
edition = "2021"
[dependencies]
bracket-lib = "0.8"
womporio-core = { path = "../core" }
```

```rust
// crates/ascii-client/src/main.rs
use bracket_lib::prelude::*;
use womporio_core::tick::{SimConfig, SimWorld};

struct State { sim: SimWorld }
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.sim.step();
        ctx.cls();
        ctx.print(1, 1, format!("Womporio tick {}", self.sim.tick));
        // TODO: render map/entities from ECS queries
    }
}
fn main() -> BError {
    let sim = SimWorld::new(SimConfig { seed: 42, tick_ms: 50 });
    let context = BTermBuilder::simple80x50().with_title("Womporio ASCII").build()?;
    main_loop(context, State { sim })
}
```

**Server (headless)**

```toml
# crates/server/Cargo.toml
[package]
name = "womporio-server"
version = "0.1.0"
edition = "2021"
[dependencies]
womporio-core = { path = "../core" }
clap = { version = "4", features = ["derive"] }
```

```rust
// crates/server/src/main.rs
use clap::Parser;
use womporio_core::tick::{SimConfig, SimWorld};

#[derive(Parser)]
struct Args { #[arg(long, default_value_t = 42)] seed: u64, #[arg(long, default_value_t = 50)] tick_ms: u64,
              #[arg(long, default_value_t = 1000)] ticks: u64 }
fn main() {
    let args = Args::parse();
    let mut sim = SimWorld::new(SimConfig { seed: args.seed, tick_ms: args.tick_ms });
    for _ in 0..args.ticks { sim.step(); }
    println!("Done at tick={}", sim.tick);
}
```

**Mod manifest example**

```toml
# mods/base/mod.toml
name = "base"
version = "0.1.0"
description = "Vanilla Womporio ruleset."
[content]
data = ["data/items.toml", "data/recipes.toml", "data/tech.toml"]
scripts = ["scripts/on_tick.lua", "scripts/on_build.lua"]
```

---

## Seed PRD (drop straight into `PRD.md`)

```markdown
# Womporio — PRD v0.1

## Vision
Dwarf Fortress ASCII clarity + Factorio-scale logistics + Aurora 4X strategic depth. Entire base game shipped as a *mod*, with a stable API enabling total conversion.

## Primary User Stories (P0)
1) As a player, I can generate a deterministic world and run the simulation headless at a fixed tick.
2) I can place **assemblers**, define **blueprints**, and build **logistics chains** (miners → belts → assemblers → storage).
3) I can research tech to unlock new recipes and ship hull components.
4) I can design ships from unlocked components and schedule production.
5) I can mod **items**, **recipes**, **entities**, **techs**, and **events** with TOML + Lua.

## Non-Goals (v0.1)
- No real-time GPU graphics; ASCII only.
- No multiplayer.
- No complex pathfinding beyond Manhattan + congestion penalty.

## System Requirements
- Deterministic sim (fixed 20 TPS default).
- Save/load via serde (snapshot every N ticks).
- Seeded RNG per-subsystem for replayability.
- Performance target: 60k entities with <30% CPU on a mid laptop.

## Architecture
- **Core ECS** (crates/core): components, systems, schedulers.
- **Server** (crates/server): headless control (run, save, batch tests).
- **Clients**: ASCII (crates/ascii-client), later GPU client.
- **Mod Layer** (crates/api): TOML schemas, Lua bridge, safe callbacks.

## Data Model (initial)
- `Item` (id, stack, mass, volume).
- `Recipe` (id, inputs[], outputs[], time, station_type).
- `Machine` (id, footprint, power, speed, allowed_recipes[]).
- `Belt` (throughput, latency).
- `Blueprint` (grid, entities[], i/o ports).
- `Tech` (prereqs[], cost[], unlocks[]).
- `Hull` & `Component` for ships (mass, power, slots, hardpoints).

## Simulation Loops
- **Factory Loop:** pull-based logistics, per-belt queues, inserter rules.
- **Research Loop:** consumes science packs into tech DAG.
- **Shipyard Loop:** material reservation, assembly stages, QA.
- **Event Loop:** time-based events (quakes, market, discoveries) via mod hooks.

## Modding API (MVP)
- **Data** in `mods/<name>/data/*.toml`.
- **Scripts** in `mods/<name>/scripts/*.lua`:
  - `on_tick(ctx)`, `on_place(entity, ctx)`, `on_research(tech, ctx)`.
- Sandboxed via `mlua` with exposed handles (no file I/O by default).

## Milestones
M1: Core ECS + headless ticks + save/load + base mod with 3 items, 2 machines, 4 recipes.  
M2: Belts/logistics + ASCII renderer of factory grid.  
M3: Tech tree + ship components + shipyard loop.  
M4: Lua hooks + mod loader + “base as mod” parity.  
M5: Blueprint import/export + linter tool.

## Risks
- Scope creep (Aurora depth): fence with milestones.
- Determinism bugs: enforce fixed schedule & RNG streams; snapshot tests.
- Mod API churn: stabilize after M3; version mods.
```

---

## README seed (drop into `README.md`)

````markdown
# Womporio

Factorio-scale logistics + Aurora 4X ship design in Dwarf Fortress ASCII. Entire base game is a mod.

## Build & Run
```bash
# ASCII client (local GUI)
cargo run -p womporio-ascii

# Headless server (CI / remote)
cargo run -p womporio-server -- --ticks 5000 --seed 123
````

## Project Layout

* `crates/core` — deterministic ECS simulation
* `crates/ascii-client` — ASCII renderer
* `crates/server` — headless driver
* `crates/api` — modding API (TOML + Lua)
* `mods/base` — base game content (shipped as a mod)

## Modding

Put mods in `mods/<name>/`. Each mod needs a `mod.toml`. See `mods/base/`.

## Contributing

* Rust 1.89+, `cargo fmt`, `cargo clippy`.
* Tests: `cargo test -p womporio-core`
* Snapshot determinism tests live in `crates/core/tests/`.

## License

MIT (subject to change before v1).

```

---

## “What package would work best?”
**Use Rust workspaces with these crates:**
```

bevy\_ecs        # ECS core without windowing
bracket-lib     # ASCII renderer
mlua            # Lua scripting for mods
serde, toml     # data-driven content
rand\_pcg        # deterministic RNG
anyhow/thiserror# error ergonomics
clap            # CLI for server/tools

````
**Why not full Bevy?** You can add it later for GPU. Starting with `bevy_ecs` keeps headless deterministic builds tiny and remote-friendly.

**Build/Test commands**
```bash
# Fast checks (remote-friendly)
cargo check
cargo test -p womporio-core
cargo run -p womporio-server -- --ticks 1000

# Local ASCII test
cargo run -p womporio-ascii
````

---

## 2–3 Alternatives (with trade-offs)

| Stack                | Pros                                                                    | Cons                                                                        | When to pick                               |
| -------------------- | ----------------------------------------------------------------------- | --------------------------------------------------------------------------- | ------------------------------------------ |
| **Godot 4 + C#**     | Editor UX, built-in scenes/UI, easy prototyping; headless export exists | Heavier runtime; determinism trickier; modding via GDExt/GDScript API churn | If you want faster UI tools now            |
| **Python + tcod**    | Fast iteration, simple ASCII, rich ecosystem                            | Perf ceilings for Factorio-scale; packaging pain                            | If scope is small or for prototyping logic |
| **Rust + Full Bevy** | ECS + renderer + assets; easy future 2D/3D                              | Adds GPU/window deps to everything; larger binaries                         | If you want sprites/tiles soon             |

**My pick remains:** Rust `bevy_ecs` + bracket-lib now; add full Bevy client later.

---

## Assumptions

* You’re OK with Rust as primary; you can review/guide AI-written code.
* Remote envs may be headless; local laptop can open a window.
* ASCII is acceptable for months; GPU visuals later.

---

## Next Steps (actionable)

1. **Initialize repo** with the workspace + files above.
2. **Implement M1**: components for Item, Machine, Inventory; deterministic tick; JSON save/load; 5 unit tests + 1 snapshot test.
3. **Set CI**: run server for 1k ticks and assert identical snapshot artifacts.
4. **Define base mod** with 3 items (ore, plate, gear), 2 machines (miner, assembler), 4 recipes.
5. **Expose first Lua hook**: `on_tick(ctx)` with read-only world query; ship simple example mod.
6. **ASCII rendering** of a 128×128 grid: belts, machines, items as glyphs.

If you want, I’ll generate the actual repo skeleton and first pass of code inline next.

---

## TL;DR

* **Stack:** Rust + `bevy_ecs` core, `bracket-lib` ASCII, `mlua` for mods.
* **Shape:** headless deterministic sim + pluggable clients; **base game is a mod**.
* **Deliverables now:** use the provided PRD + README + Cargo workspace to seed `Womporio`.
* **Why:** Runs on your laptop, remote-friendly, performant, moddable, and future-proof toward GPU.

**One quick question (optional):** any hard requirement to use a language other than Rust? If not, I’ll spit out the full repo skeleton with initial systems next message.
