extern crate raylib;

// Use external
use raylib::prelude::*;

// Import modules
mod scene;
mod scenes;

// Use local
use scene::Scene;
use scenes::main_scene::MainScene;

fn main() {
    let (mut rl, thread) = raylib::init().size(1280, 720).title("Hello, World").build();

    // Set the game to 60 FPS
    rl.set_target_fps(60);

    // Prevent to quit the game pressing escape
    rl.set_exit_key(None);

    let main_scene = MainScene::new();
    let mut current_scene: Box<dyn Scene> = Box::new(main_scene);

    // Main game loop
    while !rl.window_should_close() {
        /* -------------------- Update Scene -------------------- */
        let new_scene = current_scene.update(&mut rl);

        /* ------------------------ Draw ------------------------ */
        let mut d = rl.begin_drawing(&thread);

        // Always draw the background as white by default
        d.clear_background(Color::WHITE);

        // Draw the current scene
        current_scene.draw(&mut d);

        /* --------------------- Init Scene --------------------- */
        if let Some(scene) = new_scene {
            current_scene = scene;
        }
    }
}
