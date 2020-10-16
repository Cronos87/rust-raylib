use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;

// Use local
use crate::scene::Scene;
use crate::scenes::main_menu_scene::MainMenuScene;

struct Position {
    x: i32,
    y: i32,
}

pub struct MainScene {
    text_position: Position,
}

impl MainScene {
    /// Create the main scene
    pub fn new() -> Self {
        MainScene {
            text_position: Position { x: 20, y: 20 },
        }
    }
}

impl Scene for MainScene {
    fn update(&mut self, rl: &mut RaylibHandle) -> Option<Box<dyn Scene>> {
        // Add movement to the text
        if rl.is_key_down(KEY_LEFT) {
            self.text_position.x -= 1;
        }
        if rl.is_key_down(KEY_RIGHT) {
            self.text_position.x += 1;
        }
        if rl.is_key_down(KEY_UP) {
            self.text_position.y -= 1;
        }
        if rl.is_key_down(KEY_DOWN) {
            self.text_position.y += 1;
        }

        if rl.is_key_released(KEY_ENTER) {
            Some(Box::new(MainMenuScene::new()))
        } else {
            None
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_text(
            "Hello, world!",
            self.text_position.x,
            self.text_position.y,
            20,
            Color::BLACK,
        );
    }
}
