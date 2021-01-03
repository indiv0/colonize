use bevy::{
    reflect::{Reflect, ReflectComponent},
    render::renderer::RenderResources,
};

/// A point y_level
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Reflect, RenderResources)]
#[reflect(Component)]
pub struct YLevel {
    pub value: i32,
}

impl Default for YLevel {
    fn default() -> Self {
        YLevel { value: 0 }
    }
}
