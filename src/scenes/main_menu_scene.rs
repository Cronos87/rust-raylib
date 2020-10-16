use raylib::prelude::*;

// Use local
use crate::scene::Scene;

pub struct MainMenuScene;

impl MainMenuScene {
    /// Create the main scene
    pub fn new() -> Self {
        MainMenuScene {}
    }
}

impl Scene for MainMenuScene {
    fn update(&mut self, _rl: &mut RaylibHandle) -> Option<Box<dyn Scene>> {
        None
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_text("Main Menu Scene!", 10, 10, 20, Color::BLACK);
    }
}
