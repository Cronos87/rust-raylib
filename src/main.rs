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

    // Create the first scene of the game
    let main_scene = MainScene::new();
    main_scene.init(&mut rl);

    // Define the current scene as the first scene
    let mut current_scene: Box<dyn Scene> = Box::new(main_scene);

    // Store the next scene
    let mut next_scene: Option<Box<dyn Scene>> = None;

    // Main game loop
    while !rl.window_should_close() {
        /* --------------------- Init Scene --------------------- */
        if let Some(scene) = next_scene {
            current_scene = scene;

            // Init the scene
            current_scene.init(&mut rl);

            // Reset the next scene
            next_scene = None
        }

        /* -------------------- Update Scene -------------------- */
        let new_scene = current_scene.update(&mut rl);

        /* ---------------------- New Scene --------------------- */
        // If a new scene has been returns,
        // define the next scene with the new one
        if let Some(scene) = new_scene {
            // Replace scene
            current_scene = scene;

            // Init the new scene
            current_scene.init(&mut rl);
        }

        /* ------------------------ Draw ------------------------ */
        let mut d = rl.begin_drawing(&thread);

        // Always draw the background as white by default
        d.clear_background(Color::WHITE);

        // Draw the current scene
        current_scene.draw(&mut d);
    }
}
