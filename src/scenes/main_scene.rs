use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;

use crate::scene::Scene;

struct Position {
    x: i32,
    y: i32,
}

pub struct MainScene {
    scene: Scene,
    text_position: Position,
}

impl MainScene {
    pub fn new() -> MainScene {
        MainScene {
            scene: Scene::new(String::from("Main Scene")),
            text_position: Position { x: 20, y: 20 },
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
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
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_text(
            "Hello, world!",
            self.text_position.x,
            self.text_position.y,
            20,
            Color::BLACK,
        );
    }
}