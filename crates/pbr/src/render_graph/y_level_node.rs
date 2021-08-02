use crate::YLevel;
use bevy::{
    core::AsBytes,
    ecs::{Commands, IntoSystem, Local, Res, ResMut, Resources, System, World},
    render::{
        render_graph::{CommandQueue, Node, ResourceSlots, SystemNode},
        renderer::{
            BufferId, BufferInfo, BufferUsage, RenderContext, RenderResourceBinding,
            RenderResourceBindings, RenderResourceContext,
        },
    },
};

#[derive(Debug)]
pub struct YLevelNode {
    command_queue: CommandQueue,
}

#[allow(clippy::new_without_default)]
impl YLevelNode {
    pub fn new() -> Self {
        YLevelNode {
            command_queue: Default::default(),
        }
    }
}

impl Node for YLevelNode {
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

impl SystemNode for YLevelNode {
    fn get_system(&self, commands: &mut Commands) -> Box<dyn System<In = (), Out = ()>> {
        let system = y_level_node_system.system();
        commands.insert_local_resource(
            system.id(),
            YLevelNodeState {
                command_queue: self.command_queue.clone(),
                y_level_buffer: None,
                staging_buffer: None,
            },
        );
        Box::new(system)
    }
}

#[derive(Debug, Default)]
pub struct YLevelNodeState {
    command_queue: CommandQueue,
    y_level_buffer: Option<BufferId>,
    staging_buffer: Option<BufferId>,
}

pub(crate) fn y_level_node_system(
    mut state: Local<'_, YLevelNodeState>,
    render_resource_context: Res<'_, Box<dyn RenderResourceContext>>,
    // TODO: this write on RenderResourceBindings will prevent this system from running in parallel with other systems that do the same
    mut render_resource_bindings: ResMut<'_, RenderResourceBindings>,
    y_level: Res<'_, YLevel>,
) {
    let state = &mut state;
    let render_resource_context = &**render_resource_context;

    // Why do we need an [f32; 4] to store a vec3?
    let y_level: [f32; 4] = [y_level.value as f32; 4];
    let y_level_size = std::mem::size_of::<[f32; 4]>();
    let size = y_level_size;

    let staging_buffer = if let Some(staging_buffer) = state.staging_buffer {
        render_resource_context.map_buffer(staging_buffer);
        staging_buffer
    } else {
        let buffer = render_resource_context.create_buffer(BufferInfo {
            size,
            buffer_usage: BufferUsage::COPY_DST | BufferUsage::UNIFORM,
            ..Default::default()
        });
        render_resource_bindings.set(
            super::uniform::Y_LEVEL,
            RenderResourceBinding::Buffer {
                buffer,
                range: 0..size as u64,
                dynamic_index: None,
            },
        );
        state.y_level_buffer = Some(buffer);

        let staging_buffer = render_resource_context.create_buffer(BufferInfo {
            size,
            buffer_usage: BufferUsage::COPY_SRC | BufferUsage::MAP_WRITE,
            mapped_at_creation: true,
        });

        state.staging_buffer = Some(staging_buffer);
        staging_buffer
    };

    render_resource_context.write_mapped_buffer(
        staging_buffer,
        0..size as u64,
        &mut |data, _renderer| {
            data[0..size].copy_from_slice(y_level.as_bytes());
        },
    );
    render_resource_context.unmap_buffer(staging_buffer);

    let y_level_buffer = state.y_level_buffer.unwrap();
    state
        .command_queue
        .copy_buffer_to_buffer(staging_buffer, 0, y_level_buffer, 0, size as u64);
}
