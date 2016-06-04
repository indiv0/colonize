use std::rc::Rc;

use cgmath::{conv, Matrix4};
use glium::backend::Facade;
use glium::Surface;

use color;
use color::Color;
use rectangle::Rectangle;
use rendering::mesh::{Mesh, ToMesh};
use rendering::renderer::{Renderable, Renderer};
use rendering::shader::ShaderType;
use texture::{Texture, TextureRef};
use transform::Transform;

/// 2D Sprite vertex type.
#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coord: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coord);

/// Sprite shader.
pub struct Shader;

impl ShaderType for Shader {
    type Vertex = Vertex;

    fn vertex() -> &'static str {
        include_str!("sprite.vert")
    }

    fn fragment() -> &'static str {
        include_str!("sprite.frag")
    }
}

pub struct Sprite {
    pub color_multiply: Color,
    pub transform: Transform,
    pub mesh: Mesh<Vertex>,
    pub texture: TextureRef,
    width: f32,
    height: f32,
    rectangle: Rectangle,
}

impl Sprite {
    pub fn new(
        texture: Rc<Texture>,
        width: u32,
        height: u32,
    ) -> Self {
        let rectangle = Rectangle::new(
            0,
            0,
            texture.width as i32,
            texture.height as i32,
        );
        Sprite::with_rect(texture, rectangle, width, height)
    }

    pub fn with_rect(
        texture: Rc<Texture>,
        rectangle: Rectangle,
        width: u32,
        height: u32,
    ) -> Self {
        let (width, height) = (width as f32, height as f32);
        Sprite {
            color_multiply: color::WHITE,
            transform: Transform::new(),
            texture: TextureRef(texture),
            width: width,
            height: height,
            mesh: Self::build_mesh(width, height, &rectangle),
            rectangle: rectangle,
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;

        self.mesh = Self::build_mesh(self.width, self.height, &self.rectangle);
    }

    /// Change the texture atlas rectangle.
    pub fn rectangle(&mut self, rectangle: Rectangle) {
        self.rectangle = rectangle;

        self.mesh = Self::build_mesh(self.width, self.height, &self.rectangle);
    }

    fn build_mesh(
        width: f32,
        height: f32,
        rectangle: &Rectangle,
    ) -> Mesh<Vertex> {
        let x = rectangle.x;
        let y = rectangle.y;
        let w = rectangle.width;
        let h = rectangle.height;

        macro_rules! vertex {
            ([$a:expr, $b:expr] [$c:expr, $d:expr]) => (
                Vertex {
                    position: [$a, $b],
                    tex_coord: [$c as f32, $d as f32], // TODO: consider making the sprite tex_coord type i32
                }
            )
        }

        let vertices = vec![
            vertex!([0.0, height] [x, y]),
            vertex!([0.0, 0.0] [x, y + h]),
            vertex!([width, 0.0] [x + w, y + h]),
            vertex!([width, height] [x + w, y]),
        ];
        let mut mesh = Mesh::new();
        mesh.push_faces(
            vertices,
            vec![[0, 1, 2], [0, 2, 3]],
        );
        mesh
    }
}

impl ToMesh<Vertex> for Sprite {
    fn mesh(self) -> Mesh<Vertex> {
        self.mesh
    }
}

impl<F, S> Renderable<F, S, Shader> for Sprite
    where F: Facade,
          S: Surface,
{
    fn draw(&self, renderer: &Renderer<F, Shader>, surface: &mut S, parent: &Matrix4<f32>) {
        let uniforms = uniform! {
            color_multiply: self.color_multiply.as_array(),
            matrix: conv::array4x4(parent * self.transform.matrix()),
            tex: self.texture.clone(),
        };
        renderer.draw(surface, &self.mesh, &uniforms);
    }
}
