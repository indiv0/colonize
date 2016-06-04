use std::rc::Rc;

use cgmath::{conv, Matrix4, Vector4};
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

pub struct Sprite<'f, F>
    where F: Facade + 'f,
{
    pub color_multiply: Color,
    pub transform: Transform,
    pub mesh: Mesh<Vertex>,
    pub texture: TextureRef,
    width: f32,
    height: f32,
    rectangle: Rectangle,
    facade: &'f F,
}

impl<'f, F> Sprite<'f, F>
    where F: Facade,
{
    pub fn new(
        facade: &'f F,
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
        Sprite::with_rect(facade, texture, rectangle, width, height)
    }

    pub fn with_rect(
        facade: &'f F,
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
            mesh: Self::build_mesh(facade, width, height, &rectangle),
            rectangle: rectangle,
            facade: facade,
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;

        self.mesh = Self::build_mesh(
            self.facade,
            self.width,
            self.height,
            &self.rectangle,
        );
    }

    /// Change the texture atlas rectangle.
    pub fn rectangle(&mut self, rectangle: Rectangle) {
        self.rectangle = rectangle;

        self.mesh = Self::build_mesh(
            self.facade,
            self.width,
            self.height,
            &self.rectangle,
        );
    }

    pub fn vertices(&self) -> Vec<Vertex> {
        let x = self.rectangle.x;
        let y = self.rectangle.y;
        let w = self.rectangle.width;
        let h = self.rectangle.height;

        macro_rules! vertex {
            ([$a:expr, $b:expr] [$c:expr, $d:expr]) => (
                Vertex {
                    position: (self.transform.matrix()
                               * Vector4::new($a, $b, 0.0, 1.0))
                        .truncate()
                        .truncate()
                        .into(),
                    tex_coord: [$c as f32, $d as f32], // TODO: consider making the sprite tex_coord type i32
                }
            )
        }

        vec![
            vertex!([0.0, self.height] [x, y]),
            vertex!([0.0, 0.0] [x, y + h]),
            vertex!([self.width, 0.0] [x + w, y + h]),
            vertex!([self.width, self.height] [x + w, y]),
        ]
    }

    fn build_mesh(
        facade: &F,
        width: f32,
        height: f32,
        rectangle: &Rectangle,
    ) -> Mesh<Vertex>
        where F: Facade,
    {
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

        Mesh::with_indices(facade, &vertices, &[0, 1, 2, 2, 3, 0])
    }
}

impl<'f, F> ToMesh<Vertex> for Sprite<'f, F>
    where F: Facade,
{
    fn mesh(self) -> Mesh<Vertex> {
        self.mesh
    }
}

impl<'f, F, S> Renderable<F, S, Shader> for Sprite<'f, F>
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
