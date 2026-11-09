use serde::{Deserialize, Serialize};
use serde_json::Value;
use terminator_models::{ControlFrame, Observation};

pub const PROTOCOL_VERSION: u32 = 1;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Scenario {
    pub episode_id: String,
    pub seed: u64,
    pub game_profile: String,
    pub world_template: String,
    pub generator_version: String,
    pub resolved_setup: Value,
    pub tick_limit: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Success,
    Failure,
    Draw,
    Timeout,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StepResult {
    pub observation: Observation,
    pub applied_controls: ControlFrame,
    pub outcome: Option<Outcome>,
    pub metrics: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Request {
    Hello { protocol_version: u32 },
    Reset { scenario: Scenario },
    Step {
        episode_id: String,
        step_id: u64,
        from_tick: u64,
        controls: ControlFrame,
    },
    Shutdown,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Response {
    Hello { protocol_version: u32, game_profile: String },
    Ready { observation: Observation },
    Stepped { step_id: u64, result: StepResult },
    Error { code: String, message: String },
    Stopped,
}

/// Implemented by a transport connected to the real headless game bridge.
pub trait Worker {
    type Error: std::error::Error;

    fn request(&mut self, request: Request) -> Result<Response, Self::Error>;
}

/// Infrastructure errors are returned separately from gameplay outcomes.
pub trait EpisodeRecorder {
    type Error: std::error::Error;

    fn begin(&mut self, scenario: &Scenario, initial: &Observation) -> Result<(), Self::Error>;
    fn append(
        &mut self,
        step_id: u64,
        requested: &ControlFrame,
        result: &StepResult,
    ) -> Result<(), Self::Error>;
}
