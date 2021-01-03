use bevy::{pbr::PbrBundle, prelude::RenderPipelines, render::pipeline::RenderPipeline};

use crate::render_graph::FORWARD_PIPELINE_HANDLE;
pub use bevy::pbr::LightBundle;

pub fn pbr_bundle() -> PbrBundle {
    PbrBundle {
        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
            FORWARD_PIPELINE_HANDLE.typed(),
        )]),
        ..PbrBundle::default()
    }
}
