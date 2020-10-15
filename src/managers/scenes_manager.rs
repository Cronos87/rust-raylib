use crate::scene::Scene;

/// Manage all scenes of the game
pub struct ScenesManager {
    /// The current scene of the game
    pub current: Box<dyn Scene>,
}

impl ScenesManager {
    /// Create an instance of the scenes manager
    pub fn new(current: Box<dyn Scene>) -> ScenesManager {
        ScenesManager { current: current }
    }
}
