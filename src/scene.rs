extern crate raylib;

// External uses
use raylib::drawing::RaylibDrawHandle;
use raylib::RaylibHandle;

/// Trait representing game's scene
pub trait Scene {
    /// Update the scene on every frame
    ///
    /// # Arguments
    ///
    /// * `rl` - Raylib draw instance
    fn update(&mut self, rl: &mut RaylibHandle);

    /// Draw the scene on every frame
    ///
    /// # Arguments
    ///
    /// * `rl` - Raylib draw instance
    fn draw(&self, d: &mut RaylibDrawHandle);
}
