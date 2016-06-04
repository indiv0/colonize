use std::marker::PhantomData;

use cgmath::Matrix4;
use glium::{Blend, DrawParameters, IndexBuffer, Program, Surface, VertexBuffer};
use glium::backend::Facade;
use glium::index::PrimitiveType;
use glium::uniforms::Uniforms;

use rendering::mesh::{Mesh, VertexType};
use rendering::shader::ShaderType;

const INDEX_TYPE: PrimitiveType = PrimitiveType::TrianglesList;

pub struct Renderer<F, T>
    where F: Facade,
          T: ShaderType,
{
    pub facade: F,
    program: Program,
    params: DrawParameters<'static>,
    _shader_marker: PhantomData<T>,
}

impl<F, T> Renderer<F, T>
    where F: Facade,
          T: ShaderType,
{
    pub fn new(facade: F) -> Self {
        let program = T::program(&facade);

        Renderer {
            facade: facade,
            program: program,
            params: Self::build_params(),
            _shader_marker: PhantomData,
        }
    }

    pub fn draw<S, U, V>(&self, surface: &mut S, mesh: &Mesh<V>, uniforms: &U)
        where S: Surface,
              U: Uniforms,
              V: VertexType,
    {
        let vertices = VertexBuffer::new(&self.facade, &mesh.vertices).unwrap();
        let indices = IndexBuffer::new(&self.facade, INDEX_TYPE, &mesh.indices).unwrap();

        surface.draw(
            &vertices,
            &indices,
            &self.program,
            uniforms,
            &self.params,
        ).unwrap();
    }

    pub fn render<R, S>(&self, surface: &mut S, renderable: &R, parent: &Matrix4<f32>)
        where R: Renderable<F, S, T>,
              S: Surface,
    {
        renderable.draw(self, surface, parent)
    }

    fn build_params() -> DrawParameters<'static> {
        DrawParameters {
            blend: Blend::alpha_blending(),
            ..Default::default()
        }
    }
}

pub trait Renderable<F, S, T>
    where F: Facade,
          S: Surface,
          T: ShaderType,
{
    fn draw(&self, renderer: &Renderer<F, T>, target: &mut S, parent: &Matrix4<f32>);
}
