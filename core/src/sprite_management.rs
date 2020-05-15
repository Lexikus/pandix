extern crate image;

use std::collections::HashMap;

use image::DynamicImage;

pub(crate) enum Sprite {
    None,
    Image(DynamicImage),
    Texture2d(u32)
}

pub(crate) struct SpriteManagement {
    sprites: HashMap<String, Sprite>,
}

impl SpriteManagement {
    pub(crate) fn new() -> Self {
        Self {
            sprites: HashMap::new(),
        }
    }
}

impl SpriteManagement {
    pub(crate) fn add(&mut self, name: &str, path: &str) {
        match image::open(path) {
            Ok(image) => self.sprites.insert(name.to_owned(), Sprite::Image(image)),
            Err(_) => self.sprites.insert(name.to_owned(), Sprite::None)
        };
    }

    pub(crate) fn sprites_mut(&mut self) -> &mut HashMap<String, Sprite> {
        &mut self.sprites
    }
}
