extern crate gl;

use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;

pub enum ShaderError {
    FailedOpeningFile,
    FailedReadingFile,
    FailedCompilingShader(String),
}

pub enum ShaderType {
    VertexShader,
    FragmentShader,
}

pub struct Shader {
    id: u32,
}

impl Shader {
    pub fn new(path: &str, shader_type: ShaderType) -> Result<Shader, ShaderError> {
        let mut shader_file = File::open(path).map_err(|_| ShaderError::FailedOpeningFile)?;
        let mut shader_buffer = String::new();
        shader_file
            .read_to_string(&mut shader_buffer)
            .map_err(|_| ShaderError::FailedReadingFile)?;
        let shader =
            CString::new(shader_buffer.as_bytes()).map_err(|_| ShaderError::FailedReadingFile)?;

        let id: u32 = unsafe {
            let id = match shader_type {
                ShaderType::VertexShader => gl::CreateShader(gl::VERTEX_SHADER),
                ShaderType::FragmentShader => gl::CreateShader(gl::FRAGMENT_SHADER),
            };

            gl::ShaderSource(id, 1, &shader.as_ptr(), ptr::null());

            id
        };

        let success = unsafe {
            let mut success = 0;
            gl::CompileShader(id);
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            success
        };

        if success == 0 {
            let error_message = unsafe {
                let mut len = 0;
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);

                let mut buffer = Vec::with_capacity(len as usize);
                let buffer_ptr = buffer.as_mut_ptr() as *mut gl::types::GLchar;

                buffer.set_len(len as usize);
                gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), buffer_ptr);

                match String::from_utf8(buffer) {
                    Ok(log) => log,
                    Err(vec) => {
                        return Err(ShaderError::FailedCompilingShader(format!(
                            "Could not convert compilation log from buffer: {}",
                            vec
                        )))
                    }
                }
            };

            return Err(ShaderError::FailedCompilingShader(error_message));
        }

        Ok(Shader { id })
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn delete(&self) {
        unsafe { gl::DeleteShader(self.id) };
    }
}
