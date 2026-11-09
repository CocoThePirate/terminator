# Initial observation, control, and skill contract

This is a proposed training contract. Existing Rust types remain minimal placeholders; no live collector, model, or input execution is implemented.

## Observation frame

| Field | Meaning |
| --- | --- |
| schema_version, game_profile | Exact data contract and supported game rules |
| episode_id, observation_id | Replay identity and stable ordering |
| received_at_monotonic | Local arrival timestamp |
| estimated_server_tick, tick_uncertainty | Estimated server time and uncertainty |
| latency | Round-trip estimate, jitter, sample age; one-way delay is uncertain |
| player | Position, velocity, view direction, grounded state, movement mode |
| condition | Health, hunger, status effects, cooldowns; unknown values remain unknown |
| inventory | Observed slots, counts, durability, selected slot, observation ages |
| local_world | Observed block geometry and entities, visibility, source, age |
| recent_controls | Requested KBM frames and their execution acknowledgements |
| goal | Typed objective and its parameters |

Every sensor-derived value needs validity and freshness information. Do not equate missing data with zero. Separate privileged lab labels from policy inputs so evaluation reflects the intended observation access.

The base model encodes these observations and history into shared state. Physical quantities remain explicit where useful; latent features supplement them. A skill's learned parameters are weights, while shared state is changing input and model memory.

## KBM control frame

Fields: control_id, based_on_observation_id, intended_tick, hold_ticks, held_keys, held_mouse_buttons, mouse_delta_x, mouse_delta_y, and optional scroll_delta. Start with hold_ticks = 1; supported controls come from the game profile.

Key and button fields describe desired held state, rather than repeated press events. The executor tracks transitions and records actual execution times; mouse deltas are applied once per frame. Release all controls when observations expire or execution stops. Log sensitivity and input settings because a mouse delta does not have a universal angular meaning.

The action schema remains constant across skill modules. One controller emits the final frame so independently trained modules cannot issue conflicting OS inputs.

## Learned skill contract

A module receives shared state and the relevant objective. Its learned weights produce a fixed-size skill representation plus an applicability estimate. The shared controller combines these representations with base state to produce candidate control frames. Representations are numerical model features, not executable input sequences.

Each checkpoint declares the base encoder revision, representation size, normalization, controller compatibility, and training configuration. Replacing weights requires compatible interfaces and renewed combination evaluation; it is not automatically safe because tensor sizes match.

## First skill: reach and stop

Goal: reach a marked position and remain within its tolerance for a configured dwell interval in an obstacle-free lab arena. Start with horizontal movement and turning; add jumping and obstacles only after this behavior is measured.

Inputs include position/velocity estimates, view direction, target displacement, recent controls, and timing uncertainty. The skill representation influences movement and look control through the shared controller. Physics prediction is trained on actual subsequent observations; privileged labels may supervise prediction without becoming policy inputs.

Training measures goal completion, completion time, overshoot, and unwanted motion after stopping. Reward progress and completion, with explicit penalties for leaving the arena and excessive control changes. Keep reward weights in configuration and check that standing still or oscillating cannot earn sustained reward.

Evaluate separately across starting orientations, distances, latency/jitter profiles, and held-out arena conditions. Report success rate, stopping error, and prediction error with sample counts. Numerical pass thresholds must be selected before training, based on the scenario scale and baseline measurements.

## Prediction and deterministic decisions

Train next-state prediction first using short recorded transitions; increase rollout horizons only as prediction quality supports it. Predict a distribution or uncertainty estimate rather than assuming opponents and network timing are exact.

Use a fixed number of Monte Carlo rollouts and an explicitly recorded seed to compare candidate controls. Score task return and uncertainty with configured weights. Define tie breaking and record model/configuration hashes; deterministic decisions mean repeatability under those conditions, not guaranteed optimality.

## Replay record

Store the observation, requested and executed controls, subsequent observation, episode termination reason, training labels, seed, game profile, and checkpoint/configuration identifiers. Preserve original sensor timing so latency experiments can be reconstructed. Split training and evaluation by episode and scenario, not neighboring frames.
