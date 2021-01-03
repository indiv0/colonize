// MIT License
//
// Copyright (c) 2020 Carter Anderson
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
pub mod render_graph;

mod entity;
mod light;
mod material;
mod y_level;

use bevy::{
    ecs::IntoSystem,
    prelude::{stage, AddAsset, AppBuilder, Assets, Color, Handle, Plugin},
    reflect::RegisterTypeBuilder,
    render::{render_graph::RenderGraph, shader},
};
pub use entity::*;
pub use light::*;
pub use material::*;
pub use y_level::*;

pub mod prelude {
    pub use crate::{entity::*, light::Light, material::StandardMaterial};
}

use material::StandardMaterial;
use render_graph::add_pbr_graph;

/// NOTE: this isn't PBR yet. consider this name "aspirational" :)
#[derive(Default)]
pub struct PbrPlugin;

impl Plugin for PbrPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<StandardMaterial>()
            .register_type::<Light>()
            .add_system_to_stage(
                stage::POST_UPDATE,
                shader::asset_shader_defs_system::<StandardMaterial>.system(),
            )
            .init_resource::<AmbientLight>();
        let resources = app.resources();
        let mut render_graph = resources.get_mut::<RenderGraph>().unwrap();
        add_pbr_graph(&mut render_graph, resources);

        // add default StandardMaterial
        let mut materials = app
            .resources()
            .get_mut::<Assets<StandardMaterial>>()
            .unwrap();
        materials.set_untracked(
            Handle::<StandardMaterial>::default(),
            StandardMaterial {
                albedo: Color::PINK,
                shaded: false,
                albedo_texture: None,
            },
        );
    }
}
