/// Manage all scenes of the game
pub struct ScenesManager<'a, T> {
    /// Contains all the game scenes
    pub scenes: Vec<T>,

    /// The current scene of the game
    pub current: &'a mut T,
}

impl<'a, T> ScenesManager<'a, T> {
    /// Create an instance of the scenes manager
    pub fn new(scenes: Vec<T>, current: &mut T) -> ScenesManager<T> {
        ScenesManager {
            scenes: scenes,
            current: current,
        }
    }
}
