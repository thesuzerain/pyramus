// TODO: Is this still needed now that we have Affine2?
#[derive(Debug)]
pub struct RelativeTransform {
    pub position: (f32, f32),
    pub scale: (f32, f32),
    pub rotation: f32, // In degrees
}

impl Default for RelativeTransform {
    fn default() -> Self {
        RelativeTransform {
            position: (0.0, 0.0),
            scale: (1.0, 1.0),
            rotation: 0.0,
        }
    }
}

// TODO: From?
impl RelativeTransform {
    pub fn to_glam_affine(&self) -> glam::Affine2 {
        let (x, y) = self.position;
        let (sx, sy) = self.scale;
        let r = self.rotation.to_radians();
        glam::Affine2::from_scale_angle_translation(
            glam::Vec2::new(sx, sy),
            r,
            glam::Vec2::new(x, y),
        )
    }
}
