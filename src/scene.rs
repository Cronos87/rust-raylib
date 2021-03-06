extern crate raylib;

// External uses
use raylib::core::RaylibThread;
use raylib::drawing::RaylibDrawHandle;
use raylib::RaylibHandle;

/// Trait representing game's scene
pub trait Scene {
    /// Init the scene. Invoked when starting
    /// a new scene
    ///
    /// # Arguments
    ///
    /// * `thread` - Raylib thread
    /// * `rl` - Raylib instance
    fn init(&mut self, _thread: &RaylibThread, _rl: &mut RaylibHandle) {}

    /// Update the scene on every frame
    ///
    /// # Arguments
    ///
    /// * `rl` - Raylib instance
    ///
    /// # Returns
    ///
    /// Returns a instance of a new scene
    fn update(&mut self, rl: &mut RaylibHandle) -> Option<Box<dyn Scene>>;

    /// Draw the scene on every frame
    ///
    /// # Arguments
    ///
    /// * `rl` - Raylib draw instance
    fn draw(&self, d: &mut RaylibDrawHandle);
}
