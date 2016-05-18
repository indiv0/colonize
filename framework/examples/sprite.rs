extern crate cgmath;
extern crate glium;
extern crate image;
extern crate rgframework;

use std::rc::Rc;

use rgframework::color;
use rgframework::color::Color;

const CLEAR_COLOR: Color = color::WHITE;
const TEXTURE_COLOR: Color = color::RED;

fn main() {
    use glium::DisplayBuild;

    // Build the display.
    let display = glium::glutin::WindowBuilder::new()
        .build_glium()
        .unwrap();

    // Shader definitions
    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        in vec2 tex_coord;

        out vec2 v_tex_coord;

        uniform mat4 matrix;

        void main() {
            v_tex_coord = tex_coord;
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;
    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coord;

        out vec4 out_color;

        uniform sampler2D tex;
        uniform vec4 color;

        void main() {
            out_color = color * texture(tex, v_tex_coord);
        }
    "#;

    // Create the renderer.
    let renderer = rgframework::rendering::Renderer::new(
        display.clone(),
        vertex_shader_src,
        fragment_shader_src,
    );

    // Load the image.
    use std::io::Cursor;
    let image = image::load(
        Cursor::new(&include_bytes!("../tests/fixture/opengl.png")[..]),
        image::PNG,
    ).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(
        image.into_raw(),
        image_dimensions,
    );

    // Load the texture.
    let texture = rgframework::texture::Texture::new(&display, image);
    let texture = Rc::new(texture);

    // Create the sprite.
    let clip_size = cgmath::Vector2::new(
        image_dimensions.0,
        image_dimensions.1,
    ).cast();
    let size = cgmath::Vector2::new(1, 1);
    let sprite = rgframework::rendering::Sprite::new(
        TEXTURE_COLOR,
        clip_size,
        size,
        texture,
    );

    // Create the model matrix.
    let model = cgmath::Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, -0.5, 1.0,
    );

    loop {
        use glium::Surface;
        use rgframework::rendering::Renderable;

        let mut target = display.draw();

        target.clear_color(
            CLEAR_COLOR.r,
            CLEAR_COLOR.g,
            CLEAR_COLOR.b,
            CLEAR_COLOR.a,
        );
        sprite.draw(&renderer, &mut target, &model);
        target.finish().unwrap();

        for e in display.poll_events() {
            match e {
                glium::glutin::Event::Closed => return,
                _ => {},
            }
        }
    }
}
