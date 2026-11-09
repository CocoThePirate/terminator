# Design

## Decision loop

Observation source → validated typed observation → candidate actions → physics predictions → goal scoring → decision → lab action sink. Record the observation, candidates, selected action, and actual outcome for analysis.

Evaluate a bounded set of actions over a short horizon, execute the first action, then observe again. Exhaustively mapping every possible future is infeasible; distance to the goal alone also misses hazards and necessary detours. Score progress, collision risk, damage risk, time, and uncertainty with explicit weights.

## Determinism

Determinism means identical inputs, configuration, adapter versions, and random seed yield identical decisions. It does not mean the live game or an opponent is predictable. Specify candidate ordering and tie breaking, record sensor timestamps separately from game ticks, and reject nonfinite numbers before scoring.

## Version adapters

Give each adapter an exact game/build identifier and explicit capabilities. Keep physics parameters and observation decoding behind adapters; reject unsupported builds instead of guessing. Updates may require code changes as well as configuration changes, especially with JVM object layouts.

The observation boundary can support external process observations later. Memory access and layout discovery are intentionally unimplemented; screen observations and lab telemetry can use the same interface. Keep privileged training labels out of the evaluated policy's observations.

## Runtime requirements

Before adding live execution, define a test-session handshake, selected process/window identity, emergency stop, stale-observation timeout, and unconditional input release on errors or loss of focus. An allowlisted process alone cannot establish which server it joined.

## Learning

Use replaceable learned policy modules from the beginning of the learning design. A module is architecture plus trained weights and a compatible input/output schema; it proposes primitive actions conditioned on observations, history, and goals. It is not a recorded macro or fixed action sequence. See `LEARNING.md` for the proposed module boundaries.

The controller learns action selection from experience. A deterministic inference configuration produces repeatable decisions with frozen weights, fixed search budgets, and defined tie breaking. Training can use stochastic exploration and Monte Carlo rollouts. An optional language model can suggest typed goals at a slower rate; validate those goals before accepting them.

## Evaluation

Use separate training and held-out maps, opponents, and latency profiles. Measure task completion, prediction error, and detector precision/recall against human recordings. A stronger agent does not establish coverage of all cheats; maintain diverse scenarios and examine false positives on skilled players.

## Implementation order

1. Recorded observation source and replay format with schema/configuration versions.
2. Basic movement physics for one explicitly pinned game build.
3. Learned policy interfaces, checkpoint manifests, and action/value proposals.
4. Isolated test-session adapter and input lifecycle handling.
5. Outcome recorder and anticheat evaluation reports.
6. Monte Carlo training/search pipeline and optional structured language-model goals.

Combat, navigation, inventory, and building should become separate policy modules once the basic loop works. No live automation or memory reader is implemented in this scaffold.
