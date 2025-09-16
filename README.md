# Womporio

Factorio-scale logistics + Aurora 4X ship design in Dwarf Fortress ASCII. Entire base game is a mod.

## Build & Run
```bash
# ASCII client (local GUI)
cargo run -p womporio-ascii

# Headless server (CI / remote)
cargo run -p womporio-server -- --ticks 5000 --seed 123
```

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
