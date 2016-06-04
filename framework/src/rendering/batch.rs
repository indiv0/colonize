use cgmath::{conv, Matrix4};
use glium::Surface;
use glium::backend::Facade;

use color::Color;
use rendering::{sprite, Mesh, Renderable, Renderer, ShaderType, Sprite};
use rendering::mesh::VertexType;
use texture::TextureRef;

pub struct Batch<V>
    where V: VertexType,
{
    pub color_multiply: Color,
    pub texture: TextureRef,
    mesh: Mesh<V>,
}

trait ToBatch<V>
    where V: VertexType,
{
    fn batch(&self) -> Batch<V>;
}

impl<'a> ToBatch<sprite::Vertex> for [&'a Sprite] {
    fn batch(&self) -> Batch<sprite::Vertex> {
        let first = self[0];

        // Generate the combined mesh for the `Sprite`s.
        let mut mesh = Mesh::new();
        for sprite in self {
            mesh.push(
                sprite.mesh.vertices.clone(),
                sprite.mesh.indices.clone(),
            );
        }

        Batch {
            color_multiply: first.color_multiply.clone(),
            texture: first.texture.clone(),
            mesh: mesh,
        }
    }
}

impl<F, S, T, V> Renderable<F, S, T> for Batch<V>
    where F: Facade,
          S: Surface,
          T: ShaderType,
          V: VertexType,
{
    fn draw(&self, renderer: &Renderer<F, T>, surface: &mut S, parent: &Matrix4<f32>) {
        let uniforms = uniform! {
            color_multiply: self.color_multiply.as_array(),
            matrix: conv::array4x4(*parent),
            tex: self.texture.clone(),
        };
        renderer.draw(surface, &self.mesh, &uniforms);
    }
}

impl<'a, F, S> Renderable<F, S, sprite::Shader> for Vec<&'a Sprite>
    where F: Facade,
          S: Surface,
{
    fn draw(&self, renderer: &Renderer<F, sprite::Shader>, surface: &mut S, parent: &Matrix4<f32>) {
        let sprites = &self[..];
        let batch = sprites.batch();
        batch.draw(renderer, surface, parent);
    }
}
