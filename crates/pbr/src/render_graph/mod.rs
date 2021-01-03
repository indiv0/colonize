mod forward_pipeline;
mod lights_node;
mod y_level_node;

use bevy::{
    ecs::Resources,
    prelude::{AssetServer, Assets, GlobalTransform},
    render::{
        pipeline::PipelineDescriptor,
        render_graph::{base, AssetRenderResourcesNode, RenderGraph, RenderResourcesNode},
    },
};
pub use forward_pipeline::*;
pub use lights_node::*;
pub use y_level_node::*;

use crate::prelude::StandardMaterial;

/// the names of pbr graph nodes
pub mod node {
    pub const TRANSFORM: &str = "transform";
    pub const STANDARD_MATERIAL: &str = "standard_material";
    pub const LIGHTS: &str = "lights";
    pub const Y_LEVEL: &str = "y_level";
}

/// the names of pbr uniforms
pub mod uniform {
    pub const LIGHTS: &str = "Lights";
    pub const Y_LEVEL: &str = "YLevel";
}

pub(crate) fn add_pbr_graph(graph: &mut RenderGraph, resources: &Resources) {
    graph.add_system_node(
        node::TRANSFORM,
        RenderResourcesNode::<GlobalTransform>::new(true),
    );
    graph.add_system_node(
        node::STANDARD_MATERIAL,
        AssetRenderResourcesNode::<StandardMaterial>::new(true),
    );
    graph.add_system_node(node::LIGHTS, LightsNode::new(10));
    graph.add_system_node(node::Y_LEVEL, YLevelNode::new());
    let mut pipelines = resources.get_mut::<Assets<PipelineDescriptor>>().unwrap();
    let mut asset_server = resources.get_mut::<AssetServer>().unwrap();
    pipelines.set_untracked(
        FORWARD_PIPELINE_HANDLE,
        build_forward_pipeline(&mut asset_server),
    );

    // TODO: replace these with "autowire" groups
    graph
        .add_node_edge(node::STANDARD_MATERIAL, base::node::MAIN_PASS)
        .unwrap();
    graph
        .add_node_edge(node::TRANSFORM, base::node::MAIN_PASS)
        .unwrap();
    graph
        .add_node_edge(node::LIGHTS, base::node::MAIN_PASS)
        .unwrap();
    graph
        .add_node_edge(node::Y_LEVEL, base::node::MAIN_PASS)
        .unwrap();
}
