use crate::math::Vec3;

#[derive(Clone, Copy)] 
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Transform {
    pub fn py_new(position: Option<Vec3>, rotation: Option<Vec3>, scale: Option<Vec3>) -> Self {
        Self {
            position: position.unwrap_or_else(Vec3::default),
            rotation: rotation.unwrap_or_else(Vec3::default),
            scale: scale.unwrap_or_else(|| Vec3::new(1.0, 1.0, 1.0)), // Default scale to 1.0
        }
    }
}

// Keep your standard Rust impl for internal use
impl Transform {
    pub fn new(position: Vec3, rotation: Vec3, scale: Vec3) -> Self {
        Self { position, rotation, scale }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self { position: Vec3::default(), rotation: Vec3::default(), scale: Vec3::default() }
    }
}