#[derive(Debug)]
pub struct SceneState {
    pub current_scene: i16,
}

impl SceneState {
    pub fn new() -> Self {
        Self { current_scene: -1 }
    }
}

impl Default for SceneState {
    fn default() -> Self {
        Self::new()
    }
}
