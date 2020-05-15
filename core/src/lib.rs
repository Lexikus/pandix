mod engine;
mod scene;
mod scene_management;
mod system;
mod tick;

pub use crate::engine::Engine;
pub mod resource {
    pub use crate::scene_management::SceneManagement as Scene;
    pub use crate::tick::Tick;
    pub use context::input::Input;
}
