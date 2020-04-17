mod mat2;
mod mat2d;
mod mat3;
mod mat4;
mod quat;
mod vec2;
mod vec3;
mod vec4;

pub use mat2::Mat2;
pub use mat2d::Mat2d;
pub use mat3::Mat3;
pub use mat4::Mat4;
pub use quat::Quat;
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4::Vec4;

pub use std::f32::consts::PI;
pub use std::f32::EPSILON;

pub fn health() {
    println!("math reachable");
}
