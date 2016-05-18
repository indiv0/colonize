use cgmath::{Basis3, Matrix4, Quaternion, Vector3, VectorSpace};

pub struct Transform {
    pub scale: f32,
    pub position: Vector3<f32>,
    /// A vector setting the offset of the center of the model.
    pub anchor: Vector3<f32>,
    pub rotation: Quaternion<f32>,
}

impl Transform {
    pub fn new() -> Transform {
        Self::default()
    }

    pub fn position(self, x: f32, y: f32, z: f32) -> Transform {
        Transform { position: Vector3::new(x, y, z), ..self }
    }

    pub fn scale(self, scale: f32) -> Transform {
        Transform { scale: scale, ..self }
    }

    /// Set the object center offset.
    pub fn anchor(self, x: f32, y: f32, z: f32) -> Transform {
        Transform { anchor: Vector3::new(x, y, z), ..self }
    }

    /// Apply transform to a point.
    #[inline]
    pub fn compute(&self, x: f32, y: f32, z: f32) -> Vector3<f32> {
        self.rotation * &((Vector3::new(x, y, z) - self.anchor) * self.scale) + self.position
    }

    /// Build transform matrix.
    #[inline]
    pub fn matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.position) *
            Matrix4::from(*Basis3::from(self.rotation).as_ref()) *
            Matrix4::from_scale(self.scale) *
            Matrix4::from_translation(-self.anchor)
    }
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            scale: 1.0,
            position: Vector3::zero(),
            rotation: Quaternion::zero(),
            anchor: Vector3::zero(),
        }
    }
}
