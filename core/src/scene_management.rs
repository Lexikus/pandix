#[derive(Debug)]
pub struct SceneManagement {
    current_scene: i16,
}

impl SceneManagement {
    pub(crate) fn new() -> Self {
        Self { current_scene: -1 }
    }
}

impl SceneManagement {
    pub fn change(&mut self, scene: u8) {
        self.current_scene = scene as i16;
    }

    pub fn current(&self) -> i16 {
        self.current_scene
    }
}
