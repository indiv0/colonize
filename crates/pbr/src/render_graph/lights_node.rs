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
use crate::light::{AmbientLight, Light, LightRaw};
use bevy::{
    core::{AsBytes, Byteable},
    ecs::{Commands, IntoSystem, Local, Query, Res, ResMut, Resources, System, World},
    prelude::GlobalTransform,
    render::{
        render_graph::{CommandQueue, Node, ResourceSlots, SystemNode},
        renderer::{
            BufferId, BufferInfo, BufferUsage, RenderContext, RenderResourceBinding,
            RenderResourceBindings, RenderResourceContext,
        },
    },
};

use super::uniform;

/// A Render Graph [Node] that write light data from the ECS to GPU buffers
#[derive(Debug, Default)]
pub struct LightsNode {
    command_queue: CommandQueue,
    max_lights: usize,
}

impl LightsNode {
    pub fn new(max_lights: usize) -> Self {
        LightsNode {
            max_lights,
            command_queue: CommandQueue::default(),
        }
    }
}

impl Node for LightsNode {
    fn update(
        &mut self,
        _world: &World,
        _resources: &Resources,
        render_context: &mut dyn RenderContext,
        _input: &ResourceSlots,
        _output: &mut ResourceSlots,
    ) {
        self.command_queue.execute(render_context);
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct LightCount {
    pub num_lights: [u32; 4],
}

unsafe impl Byteable for LightCount {}

impl SystemNode for LightsNode {
    fn get_system(&self, commands: &mut Commands) -> Box<dyn System<In = (), Out = ()>> {
        let system = lights_node_system.system();
        commands.insert_local_resource(
            system.id(),
            LightsNodeSystemState {
                command_queue: self.command_queue.clone(),
                max_lights: self.max_lights,
                light_buffer: None,
                staging_buffer: None,
            },
        );
        Box::new(system)
    }
}

/// Local "lights node system" state
#[derive(Debug, Default)]
pub struct LightsNodeSystemState {
    light_buffer: Option<BufferId>,
    staging_buffer: Option<BufferId>,
    command_queue: CommandQueue,
    max_lights: usize,
}

pub fn lights_node_system(
    mut state: Local<LightsNodeSystemState>,
    render_resource_context: Res<Box<dyn RenderResourceContext>>,
    ambient_light_resource: Res<AmbientLight>,
    // TODO: this write on RenderResourceBindings will prevent this system from running in parallel with other systems that do the same
    mut render_resource_bindings: ResMut<RenderResourceBindings>,
    query: Query<(&Light, &GlobalTransform)>,
) {
    let state = &mut state;
    let render_resource_context = &**render_resource_context;

    let ambient_light: [f32; 4] = ambient_light_resource.color.into();
    let ambient_light_size = std::mem::size_of::<[f32; 4]>();
    let light_count = query.iter().count();
    let size = std::mem::size_of::<LightRaw>();
    let light_count_size = ambient_light_size + std::mem::size_of::<LightCount>();
    let light_array_size = size * light_count;
    let light_array_max_size = size * state.max_lights;
    let current_light_uniform_size = light_count_size + light_array_size;
    let max_light_uniform_size = light_count_size + light_array_max_size;

    if let Some(staging_buffer) = state.staging_buffer {
        if light_count == 0 {
            return;
        }

        render_resource_context.map_buffer(staging_buffer);
    } else {
        let buffer = render_resource_context.create_buffer(BufferInfo {
            size: max_light_uniform_size,
            buffer_usage: BufferUsage::UNIFORM | BufferUsage::COPY_SRC | BufferUsage::COPY_DST,
            ..Default::default()
        });
        render_resource_bindings.set(
            uniform::LIGHTS,
            RenderResourceBinding::Buffer {
                buffer,
                range: 0..max_light_uniform_size as u64,
                dynamic_index: None,
            },
        );
        state.light_buffer = Some(buffer);

        let staging_buffer = render_resource_context.create_buffer(BufferInfo {
            size: max_light_uniform_size,
            buffer_usage: BufferUsage::COPY_SRC | BufferUsage::MAP_WRITE,
            mapped_at_creation: true,
        });
        state.staging_buffer = Some(staging_buffer);
    }

    let staging_buffer = state.staging_buffer.unwrap();
    render_resource_context.write_mapped_buffer(
        staging_buffer,
        0..current_light_uniform_size as u64,
        &mut |data, _renderer| {
            // ambient light
            data[0..ambient_light_size].copy_from_slice(ambient_light.as_bytes());

            // light count
            data[ambient_light_size..light_count_size]
                .copy_from_slice([light_count as u32, 0, 0, 0].as_bytes());

            // light array
            for ((light, global_transform), slot) in query
                .iter()
                .zip(data[light_count_size..current_light_uniform_size].chunks_exact_mut(size))
            {
                slot.copy_from_slice(LightRaw::from(light, global_transform).as_bytes());
            }
        },
    );
    render_resource_context.unmap_buffer(staging_buffer);
    let light_buffer = state.light_buffer.unwrap();
    state.command_queue.copy_buffer_to_buffer(
        staging_buffer,
        0,
        light_buffer,
        0,
        max_light_uniform_size as u64,
    );
}
