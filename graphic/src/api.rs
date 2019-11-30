#![allow(dead_code)]

extern crate gl;

pub fn enable_depth_test() {
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }
}

pub fn disable_depth_test() {
    unsafe {
        gl::Disable(gl::DEPTH_TEST);
    }
}

pub fn clear() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}

pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::ClearColor(r, g, b, a);
    }
}

// TODO(alex):
// this can fail. we should error handle this somehow.
pub fn load_gpu_function_pointers<F: FnMut(&'static str) -> *const std::ffi::c_void>(
    mut loadfn: F,
) {
    gl::load_with(|s| loadfn(s));
}
