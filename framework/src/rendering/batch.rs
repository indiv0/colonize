use cgmath::{conv, Matrix4};
use glium::Surface;
use glium::backend::Facade;

use color::Color;
use rendering::{sprite, Mesh, Renderable, Renderer, ShaderType, Sprite};
use rendering::mesh::{self, Either, IndexBuffer, VertexBuffer, VertexType};
use texture::TextureRef;

pub struct Batch<V>
    where V: VertexType,
{
    pub color_multiply: Color,
    pub texture: TextureRef,
    mesh: Mesh<V>,
}

trait ToBatch<F, V>
    where F: Facade,
          V: VertexType,
{
    fn batch(&self, facade: &F) -> Batch<V>;
}

trait Batchable {
    fn is_batchable(&self, other: &Self) -> bool;
}

impl<'f, F> Batchable for Sprite<'f, F>
    where F: Facade,
{
    fn is_batchable(&self, other: &Self) -> bool {
        *self.texture == *other.texture &&
            self.color_multiply == other.color_multiply
    }
}

impl<'a, 'f, F> ToBatch<F, sprite::Vertex> for [&'a Sprite<'f, F>]
    where F: Facade,
{
    fn batch(&self, facade: &F) -> Batch<sprite::Vertex> {
        const INDICES_PER_SPRITE: usize = 6;
        const VERTICES_PER_SPRITE: usize = 4;

        let len = self.len();

        // The list of sprites to batch should never be empty.
        if len == 0 {
            panic!()
        }

        // Get the first `Sprite` in the list.
        let first = self[0];

        let mut vertex_buffer = VertexBuffer::empty_dynamic(
            facade,
            len * VERTICES_PER_SPRITE,
        ).unwrap();
        let mut index_buffer = Vec::with_capacity(
            len * INDICES_PER_SPRITE,
        );

        for (i, chunk) in vertex_buffer.map().chunks_mut(4).enumerate() {
            let sprite = self[i];
            // The sprite must be able to batch with the first `Sprite`.
            assert!(first.is_batchable(sprite));
            //let ref mut vertices = sprite.mesh.vertices;

            // Add the vertices from the `Sprite`'s mesh to the batch vertex
            // buffer.
            let vertices = sprite.vertices();
            for i in 0..VERTICES_PER_SPRITE {
                chunk[i] = vertices[i];
            }
            /*
            for i in 0..VERTICES_PER_SPRITE {
                chunk[i] = vertices[i];
            }
            */

            // Add the indices to the index buffer with the proper offset.
            let num = i as u16;
            let offset = num * VERTICES_PER_SPRITE as u16;
            index_buffer.push(offset + 0);
            index_buffer.push(offset + 1);
            index_buffer.push(offset + 2);
            index_buffer.push(offset + 1);
            index_buffer.push(offset + 3);
            index_buffer.push(offset + 2);
        }

        let index_buffer = IndexBuffer::new(
            facade,
            mesh::INDEX_TYPE,
            &index_buffer[..]
        ).unwrap();

        Batch {
            color_multiply: first.color_multiply.clone(),
            texture: first.texture.clone(),
            mesh: Mesh {
                vertices: vertex_buffer,
                indices: Either::Left(index_buffer),
            },
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

impl<'a, 'f, F, S> Renderable<F, S, sprite::Shader> for Vec<&'a Sprite<'f, F>>
    where F: Facade,
          S: Surface,
{
    fn draw(&self, renderer: &Renderer<F, sprite::Shader>, surface: &mut S, parent: &Matrix4<f32>) {
        let sprites = &self[..];
        let batch = sprites.batch(&renderer.facade);
        batch.draw(renderer, surface, parent);
    }
}
