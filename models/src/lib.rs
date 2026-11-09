use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Wire data from one completed game tick. Payload schema is independent
/// of the model's internal representation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Observation {
    pub episode_id: String,
    pub tick: u64,
    pub schema: String,
    pub payload: Value,
}

/// Desired inputs for one game tick. Look deltas are applied once.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ControlFrame {
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub sprint: bool,
    pub sneak: bool,
    pub attack: bool,
    pub use_item: bool,
    pub yaw_delta_degrees: f32,
    pub pitch_delta_degrees: f32,
}

pub trait Model {
    type Error: std::error::Error;

    fn reset(&mut self, initial: &Observation) -> Result<(), Self::Error>;
    fn decide(&mut self, observation: &Observation) -> Result<ControlFrame, Self::Error>;
}
