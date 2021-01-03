use bevy::{
    core::Byteable,
    prelude::GlobalTransform,
    render::camera::{CameraProjection, PerspectiveProjection},
};

pub use bevy::pbr::{AmbientLight, Light};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct LightRaw {
    pub proj: [[f32; 4]; 4],
    pub pos: [f32; 4],
    pub color: [f32; 4],
}

unsafe impl Byteable for LightRaw {}

impl LightRaw {
    pub fn from(light: &Light, global_transform: &GlobalTransform) -> LightRaw {
        let perspective = PerspectiveProjection {
            fov: light.fov,
            aspect_ratio: 1.0,
            near: light.depth.start,
            far: light.depth.end,
        };

        let proj = perspective.get_projection_matrix() * global_transform.compute_matrix();
        let (x, y, z) = global_transform.translation.into();
        LightRaw {
            proj: proj.to_cols_array_2d(),
            pos: [x, y, z, 1.0],
            color: light.color.into(),
        }
    }
}
