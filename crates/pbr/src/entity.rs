use bevy::{
    ecs::Bundle,
    prelude::{Draw, GlobalTransform, Handle, Mesh, RenderPipelines, Transform, Visible},
    render::{pipeline::RenderPipeline, render_graph::base::MainPass},
};

use crate::{light::Light, material::StandardMaterial, render_graph::FORWARD_PIPELINE_HANDLE};

/// A component bundle for "pbr mesh" entities
#[derive(Bundle)]
pub struct PbrBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub main_pass: MainPass,
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for PbrBundle {
    fn default() -> Self {
        Self {
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                FORWARD_PIPELINE_HANDLE.typed(),
            )]),
            mesh: Default::default(),
            visible: Default::default(),
            material: Default::default(),
            main_pass: Default::default(),
            draw: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}

/// A component bundle for "light" entities
#[derive(Debug, Default, Bundle)]
pub struct LightBundle {
    pub light: Light,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
