pub struct Scene {
    name: String
}

impl Scene {
    // Create a new instance of a scene
    pub fn new(name: String) -> Scene {
        Scene { name: name }
    }
}
