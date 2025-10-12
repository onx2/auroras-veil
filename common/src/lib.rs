use glam::Vec2;

pub mod calculate_step;
pub mod chunk;

pub fn distance_squared(a: [f32; 2], b: [f32; 2]) -> f32 {
    let a = Vec2::from_array(a);
    let b = Vec2::from_array(b);
    a.distance_squared(b)
}
