extern crate gl;

pub enum TextureError {
    OpeningTextureFailed,
}

pub struct Texture {
    id: u32,
}

#[derive(Copy, Clone)]
pub enum TextureFormat {
    RED = 0x1903,
    RG = 0x8227,
    RGB = 0x1907,
    RGBA = 0x1908,
}

impl Texture {
    pub fn new(raw_pixels: Vec<u8>, format: TextureFormat, width: u32, height: u32) -> Result<Texture, TextureError> {
        let mut id: u32 = 0;

        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as i32,
                width as i32,
                height as i32,
                0,
                format as u32,
                gl::UNSIGNED_BYTE,
                raw_pixels.as_ptr() as *const std::ffi::c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Ok(Texture {
            id,
        })
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn bind_at_position(&self, position: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 as u32 + position);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}
