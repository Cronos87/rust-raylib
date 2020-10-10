extern crate raylib;

// Use external
use raylib::prelude::*;

// Import modules
mod scene;
mod scenes;

// Use local
use scenes::main_scene::MainScene;

fn main() {
    let (mut rl, thread) = raylib::init().size(1280, 720).title("Hello, World").build();

    // Set the game to 60 FPS
    rl.set_target_fps(60);

    let mut scene = MainScene::new();

    // Main game loop
    while !rl.window_should_close() {
        // Update the current scene
        scene.update(&mut rl);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        // Draw the current scene
        scene.draw(&mut d);
    }
}
