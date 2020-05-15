#[derive(Debug)]
pub struct SceneManagement {
    pub current_scene: i16,
}

impl SceneManagement {
    pub fn new() -> Self {
        Self { current_scene: -1 }
    }
}

impl Default for SceneManagement {
    fn default() -> Self {
        Self::new()
    }
}
