use terminator_models::{ControlFrame, Observation};

pub trait ObservationSource {
    type Error: std::error::Error;

    fn read(&mut self) -> Result<Observation, Self::Error>;
}

pub trait InputExecutor {
    type Error: std::error::Error;

    fn apply(&mut self, controls: &ControlFrame) -> Result<(), Self::Error>;
    fn release_all(&mut self) -> Result<(), Self::Error>;
}

/// Read-only status owned by the live session and consumed by the HUD.
#[derive(Clone, Debug, Default)]
pub struct HudStatus {
    pub observed_tick: Option<u64>,
    pub planned_controls: Option<ControlFrame>,
    pub applied_controls: Option<ControlFrame>,
    pub predicted_path: Vec<[f64; 3]>,
    pub paused: bool,
}
