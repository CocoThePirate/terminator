# Terminator ML handoff

Build a goal-conditioned Minecraft agent for an isolated, permissioned
research environment. Initial training and evaluation use actual Minecraft
Java Edition running headlessly. The model chooses controls one game tick at a
time. The environment supplies telemetry and verifies task outcomes. The ML
contributor owns model design and training methods.

## Code and ownership

- [Model interface](../models/src/lib.rs) defines reset, observation, and
  one-tick decision. It is a Rust integration boundary, not a requirement
  to train in Rust. A separate trainer can export weights or expose
  inference through an adapter implementing this interface.
- [Training protocol](../training/src/lib.rs) defines worker handshake,
  reset, step, verified result, and recording hooks. The Java/Fabric bridge,
  transport, run manager, and persistent trajectory writer are engineering work.
- [External reader](../external-reader/src/lib.rs) defines future Windows
  observation, keyboard and mouse execution, and HUD interfaces. Integrate
  these after the agent demonstrates reliable headless behavior.

## What the ML contributor can start now

Implement preprocessing, policy inference, training, and checkpoint loading
independently of Minecraft process management. The current boundary uses an
observation schema ID and a JSON payload. No concrete game telemetry dataset
or schema is frozen yet. Treat the feature families below as requirements to
align with engineering before training. Keep model-specific tensor shapes and
normalization behind the adapter.

- Inputs should support player state, inventory, entities, nearby terrain,
  goal, and observation history, with source, freshness, and missing-data
  masks where applicable. Keep privileged server labels separate from policy
  observations so the evaluation can match the intended live inputs.
- Outputs are one control frame per tick, with concurrent buttons and
  continuous yaw/pitch deltas in degrees. The current frame covers movement,
  jump, sprint, sneak, attack, and use. Hotbar and inventory UI interactions
  remain to be specified.
- Shared state provides context for separately trainable learned skill
  weights. The latest design explored a shared base with LoRA-style experts
  and a learned router. This is a research direction. Architecture, losses,
  routing, checkpoint format, and training algorithm remain open.
- Version-specific algorithmic physics is a separate component. Validate
  it against real-game transitions. Monte Carlo may generate candidate
  controls and outcome estimates for the learned controller to rank.
  Training the policy and adding search are separable decisions.

## Real-game integration and acceptance

1. Pin compatible Minecraft, Fabric, Java, and headless runtime versions.
   Build a Java/Fabric bridge for scenario setup, authoritative telemetry,
   action application, and synchronized tick advancement. HeadlessMc is a
   candidate launcher. Verify stepping on the actual client/server pair.
2. Validate reset, observation, action, and resulting observation on the
   real game. Check that step IDs identify exactly one application and that
   retries do not double-step. Verify tick alignment and separate runtime
   errors from gameplay outcomes.
3. Add isolated parallel workers, configurable resource limits, resumable
   runs, versioned scenario generation, and persistent trajectories.
   Record requested and applied controls, observations, terminal outcome,
   metrics, timing, and scenario/checkpoint identity. For selected episodes,
   record world snapshots and deltas for a future 3D replay viewer.
4. Connect model inference and train across varied generated scenarios.
   Evaluate checkpoints by goal success and efficiency on fresh variants.
   Use independent game-state checks, with progress signals where useful.
   Exact path reproduction is a diagnostic, not a model success metric.
5. After successful tick-stepped evaluation, measure real-time latency and
   transfer. Add the external reader, KBM, and HUD only for the authorized
   private test environment.

The repository currently provides crate boundaries and contracts. It does
not include a working game bridge, trained checkpoint, trajectory dataset,
or end-to-end validation. Start with the real-game step contract and
telemetry schema as the integration milestones.
