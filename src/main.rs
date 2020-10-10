extern crate raylib;

use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

struct Position {
    x: i32,
    y: i32
}

struct Scene {
    name: String
}

impl Scene {
    // Create a new instance of a scene
    fn new(name: String) -> Scene {
        Scene { name: name }
    }
}

struct MainScene {
    scene: Scene,
    text_position: Position
}

impl MainScene {
    fn new(text_position: Position) -> MainScene {
        MainScene { scene: Scene::new(String::from("Main Scene")), text_position: text_position }
    }

    fn update(&mut self, rl: &mut RaylibHandle) {
        // Add movement to the text
        if rl.is_key_down(KEY_LEFT) { self.text_position.x -= 1; }
        if rl.is_key_down(KEY_RIGHT) { self.text_position.x += 1; }
        if rl.is_key_down(KEY_UP) { self.text_position.y -= 1; }
        if rl.is_key_down(KEY_DOWN) { self.text_position.y += 1; }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_text("Hello, world!", self.text_position.x, self.text_position.y, 20, Color::BLACK);
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Hello, World")
        .build();

    // Set the game to 60 FPS
    rl.set_target_fps(60);

    let mut scene = MainScene::new(Position { x: 10, y: 10 });

    // Main game loop
    while !rl.window_should_close() {
        scene.update(&mut rl);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        scene.draw(&mut d);
    }
}
