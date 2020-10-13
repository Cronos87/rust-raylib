/// Manage all scenes of the game
pub struct ScenesManager<T> {
    /// Contains all the game scenes
    pub scenes: Vec<T>,

    /// The current scene of the game
    pub current: T,
}

impl<T> ScenesManager<T> {
    /// Create an instance of the scenes manager
    pub fn new(scenes: Vec<T>, current: T) -> ScenesManager<T> {
        ScenesManager {
            scenes: scenes,
            current: current,
        }
    }
}
