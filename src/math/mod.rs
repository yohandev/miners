pub use vek::*;

/// Creates a new 2-dimensional vector
#[inline]
pub fn vec2<T>(x: T, y: T) -> Vec2<T>
{
    Vec2 { x, y }
}

/// Creates a new 3-dimensional vector
#[inline]
pub fn vec3<T>(x: T, y: T, z: T) -> Vec3<T>
{
    Vec3 { x, y, z }
}