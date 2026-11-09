# Terminator

Rust engineering scaffold for an isolated Minecraft training environment.

Start with the [ML handoff](docs/ML-HANDOFF.md) for the project brief,
integration contract, and work split. The [archived designs](docs/archive/)
document earlier proposals. Where they differ, follow the handoff and
current Rust interfaces.

- `training/` owns the headless worker protocol, scenarios, results, and recording interfaces.
- `models/` owns the shared observation and control types and the model interface.
- `external-reader/` owns the future live observation, input execution, and HUD interfaces.

Training and live code depend on the model interface independently. The ML implementation can be added to `models/` without changing the worker protocol. Observation payloads carry a schema ID so wire data can evolve independently of tensor layout.

## Real-game integration

The planned Java/Fabric bridge exchanges versioned requests and responses with Rust. Reset loads a fresh episode from an immutable template, applies and verifies the resolved setup, then returns the initial observation. A step carries the episode ID, expected starting tick, step ID, and controls. The bridge applies the controls once, advances one coordinated game tick, and reports the resulting observation and applied inputs. The bridge must reject stale requests and handle duplicate step IDs without advancing twice.

The next integration work is to pin a Minecraft/Fabric version, implement that bridge against the real headless game, then wire in the Rust supervisor and recorder. Parallel scheduling, durable run storage, 3D replay, live Windows adapters, and HUD rendering are subsequent engineering work. This workspace currently provides contracts and crate boundaries.

Check the workspace with `cargo check --workspace`.

The [conversation transcript](private/TRANSCRIPT.md) is included for private
repository collaborators. Connect a GitHub remote when ready to publish.
