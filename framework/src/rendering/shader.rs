//! Shader and uniforms definition tools.

use glium::Program;
use glium::backend::Facade;

use rendering::mesh::VertexType;

pub trait ShaderType {
    type Vertex: VertexType;

    /// Vertex shader GLSL source code.
    fn vertex() -> &'static str;

    /// Fragment shader GLSL source code.
    fn fragment() -> &'static str;

    /// Compile the shaders and return a shaders object.
    fn program<F>(facade: &F) -> Program
        where F: Facade,
    {
        Program::from_source(facade, Self::vertex(), Self::fragment(), None).unwrap()
    }
}
