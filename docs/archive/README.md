# Minecraft research lab — architecture scaffold

This is a dependency-free Rust library design, not a working game agent. It has typed observations, goals, actions, adapter interfaces, and a deterministic placeholder planner that releases inputs.

## Editing map

| File | Purpose |
| --- | --- |
| `src/model.rs` | Shared types; start here when changing the data model |
| `src/ports.rs` | Boundaries for sensors, physics, input, and recordings |
| `src/planner.rs` | Decision logic; currently returns ReleaseAll |
| `DESIGN.md` | Planned behavior and implementation order |

Keep modules small and explicit. Add dependencies only when a concrete adapter needs them. No game version, process offsets, input driver, model API, or network connection is selected yet.

With Rust installed, run `cargo check` in this directory.
