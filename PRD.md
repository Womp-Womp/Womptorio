# Womporio Product Requirements (First Cut)

## Vision

Blend Factorio-scale logistics with Aurora 4X ship design, presented through a Dwarf Fortress style ASCII interface. The entire "base game" ships as a mod that consumes a stable simulation core and scripting API.

## Pillars

1. **Deterministic Simulation:** Fixed tick ECS core that can run headless in CI, on servers, or locally.
2. **ASCII-First Presentation:** A terminal-capable client for immediate play, with room to layer richer renderers later.
3. **Mod-Driven Content:** Data tables and Lua hooks define items, recipes, entities, and events.

## Milestone 1 Scope

* ECS core crate (`womporio-core`) with foundational components for items, inventories, machines, and timing.
* Deterministic `SimWorld` runner with seeded RNG and a simple machine processing loop.
* Headless server binary capable of stepping the simulation and emitting JSON snapshots.
* ASCII client stub bootstrapped with `bracket-lib` (implementation TBD).
* Modding crate stub exposing a future Lua bridge.
* Tools workspace seeded with a placeholder blueprint linter CLI.
* Base mod folder containing a manifest and placeholder data/scripts directories.

## Non-Goals (Yet)

* Graphical/GPU rendering.
* Networking or multiplayer synchronization.
* Comprehensive gameplay content or balancing.
* Stable mod API surface—expect breaking changes pre-v1.

## Success Criteria

* `cargo check` and `cargo test -p womporio-core` succeed from a clean checkout.
* Server can run for an arbitrary number of ticks (`--ticks`) and produce a serialized world snapshot.
* Repository layout communicates separation between simulation, presentation, mods, and tooling.
