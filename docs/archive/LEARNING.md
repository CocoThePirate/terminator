# Learned modular controller

## Intended behavior

Modules contain trainable policies rather than macros. Given a goal and recent observations, each module proposes primitive controls and estimates their usefulness. The controller evaluates action proposals across modules and emits one coherent control frame. It does not have to commit to a named skill before choosing an action.

The current Rust planner remains a release-all placeholder. This document specifies the future architecture; no learning or search is implemented yet.

## Independently editable components

| Component | Responsibility | Replaceable artifact |
| --- | --- | --- |
| Observation encoder | Turn observations and history into model inputs | Encoder weights and schema |
| Policy modules | Propose goal-conditioned primitive controls | One checkpoint per module |
| Dynamics model | Predict consequences and uncertainty | Physics adapter plus optional learned residual |
| Value model | Estimate longer-term task return | Value checkpoint |
| Controller | Rank proposals using learned values and optional search | Controller checkpoint and search configuration |
| Trainer | Explore, collect trajectories, update parameters | Training configuration and optimizer state |
| Evaluator | Measure performance on held-out scenarios | Scenario definitions and reports |

Start with a shared encoder and separate small policy heads if resources are limited. Fully independent models are easier to replace but can duplicate computation. Checkpoints must declare encoder compatibility; arbitrary weight files are not interchangeable.

## Actions and composition

An action is a primitive control frame, such as movement buttons plus look deltas and interaction buttons. A policy is a learned function that produces those controls. Swapping a policy changes behavior without changing the action vocabulary or requiring a different controller interface.

Policies can specialize during training while sharing the same output schema. Route or blend their proposals through the controller, then resolve incompatible buttons centrally. Avoid averaging unrelated control sequences; evaluate coherent candidate frames or short trajectories instead.

## Monte Carlo loop

1. Sample starting states and goals from the isolated lab or recorded trajectories.
2. Let policies propose actions and explore simulated continuations.
3. Accumulate rewards, terminal outcomes, and uncertainty over a bounded horizon.
4. Train policy/value/controller parameters from these trajectories using an explicitly chosen learning algorithm.
5. Freeze checkpoints and evaluate on unseen maps, conditions, and opponents.

Monte Carlo simulation supplies estimates and experience; it does not itself specify a training algorithm. A first implementation can use policy-gradient updates with a value baseline; optional tree search can later improve action targets for distillation into the controller.

## Repeatable inference

Freeze weights and preprocessing, reset or record recurrent state, use a fixed simulation count and seed, and specify tie breaking. Identical observation histories and configuration should then yield identical actions within the guarantees of the selected numerical backend. Exact reproducibility across hardware may require additional constraints.

## Generalization limits

Use observation history or recurrent memory for partial visibility and opponent behavior. Train on varied conditions and compare predicted transitions to actual lab outcomes so the policy does not merely exploit simulator mistakes. Human-level generality is a research target, not a consequence of modular weights or Monte Carlo search.

## Checkpoint manifest

Each module should declare its ID, revision, input schema, action schema, encoder revision, supported game profiles, training configuration, weight hash, and evaluation results. Keep weights separate from Rust source and training configuration so manual edits and model replacement remain straightforward.
