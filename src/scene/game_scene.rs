use std::rc::Rc;

use cgmath::{EuclideanSpace, Matrix4, Point2, Point3, Vector2};
use glium::Surface;
use glium::backend::Facade;
use piston::input::keyboard::Key;
use piston::input::{GenericEvent, MouseCursorEvent, PressEvent};
use piston::input::Button::Keyboard;
use rgframework::{
    BindingsHashMap,
    BindingMap,
    BindingStore,
    BoxedScene,
    Command,
    Scene,
    SceneCommand,
    UnwrapBindings,
};
use rgframework::backend::graphics::RenderContext;
use rgframework::color;
use rgframework::color::Color;
use rgframework::rectangle::Rectangle;
use rgframework::rendering::{sprite, Renderable, Renderer, Sprite};
use rgframework::texture::Texture;
use utility::Bounds;
use world;
use world::World;

use action::Action;
use camera;
use camera::{Camera, CameraAction};
use scene::{MenuScene, SceneContext};

// Colors
const BACKGROUND_COLOR: Color = color::WHITE;
const TEXT_COLOR: Color = color::BLACK;

// Cursor
const CURSOR_COLOR: Color = color::RED;
const CURSOR_SIZE: f64 = 16.0;

// Tile
const DISPLAY_SIZE: u32 = 32;
const TILE_SIZE: u32 = 32;
const TILE_COLOR: Color = color::WHITE;
const MARGIN: u32 = 0;

pub struct GameScene<F>
    where F: Facade,
{
    key_bindings: BindingsHashMap<Key, Action>,
    mouse_pos: Point2<f64>,
    world: World,
    bounds: Bounds<i32>,
    camera: Camera<F>,
    cursor: Cursor,
    context: Rc<SceneContext<F>>,
    texture: Rc<Texture>,
}

impl<F> GameScene<F>
    where F: Facade,
{
    pub fn new(context: Rc<SceneContext<F>>) -> Self {
        Self::new_internal(
            context.clone(),
            context.config.game_scene_key_bindings.unwrap_bindings(),
        )
    }

    fn new_internal(context: Rc<SceneContext<F>>, key_bindings: BindingsHashMap<Key, Action>) -> Self {
        let texture = context.textures.load("game_scene/block.png");

        // TODO: refactor these magic numbers.
        let bounds = Bounds::new(0, 0, 54, 49);
        let cursor = Cursor::new(
            bounds.width() as f32 / 2.0,
            bounds.height() as f32 / 2.0,
            texture.clone(),
        );

        GameScene {
            key_bindings: key_bindings,
            mouse_pos: Point2::origin(),
            world: World::new(None, context.config.initial_world_size),
            bounds: bounds,
            camera: Camera::new(context.facade.clone()),
            cursor: cursor,
            texture: texture,
            context: context,
        }
    }
}

impl<E, F, S> Scene<E, S> for GameScene<F>
    where E: GenericEvent,
          F: Facade + 'static,
          S: Surface,
{
    fn to_box(self) -> BoxedScene<E, S> {
        Box::new(self)
    }

    fn render(&mut self, surface: &mut S, context: &RenderContext) {
        surface.clear_color(BACKGROUND_COLOR.r, BACKGROUND_COLOR.g, BACKGROUND_COLOR.b, BACKGROUND_COLOR.a);

        let camera_pos = self.camera.get_position();

        let start_x = camera_pos.x - self.bounds.width() / 2;
        let start_z = camera_pos.z - self.bounds.height() / 2;

        let matrix = Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        for x in 0..self.bounds.width() {
            for z in 0..self.bounds.height() {
                let screen_pos = Point2::new(x, z);
                let pos = Point3::new(x + start_x, camera_pos.y, z + start_z);

                let cell_drawable = CellDrawable::new(pos, screen_pos, &self.world, self.texture.clone());
                cell_drawable.draw(&self.context.sprite_renderer, surface, &self.camera.matrix());
            }
        }

        self.cursor.draw(&self.context.sprite_renderer, surface, &matrix);

        for &(ref text_str, pos) in &[
            (&self.context.localization.gamescene_welcome_text, Vector2::new(10.0, 100.0)),
            (&format!(
                    "{}: {:?}",
                    self.context.localization.gamescene_debug_cursor,
                    self.mouse_pos,
                ), Vector2::new(10.0, 150.0)),
            (&format!(
                    "{}: {:?}",
                    self.context.localization.gamescene_debug_camera,
                    self.camera.get_position(),
                ), Vector2::new(10.0, 200.0)),
            (&format!(
                    "{}: {:?}",
                    self.context.localization.gamescene_debug_chunk,
                    world::abs_pos_to_chunk_pos(self.camera.get_position()),
                ), Vector2::new(10.0, 250.0)),
        ] {
            self.context.text_renderer.borrow_mut().draw(&context.context, surface, text_str, pos, TEXT_COLOR);
        }
    }

    fn handle_event(&mut self, e: &E) -> Option<SceneCommand<E, S>> {
        let mut maybe_scene = None;

        e.mouse_cursor(|x, y| {
            self.mouse_pos = Point2::new(x, y);
        });

        e.press(|button_type| {
            if let Keyboard(key) = button_type {
                match key {
                    Key::Backspace => maybe_scene = Some(SceneCommand::SetScene(MenuScene::new(self.context.clone()).to_box())),
                    _ => {
                        let command = self.get_command_from_binding(&key);
                        if let Some(mut command) = command {
                            command();
                        }
                    }
                }
            }
        });

        maybe_scene
    }
}

impl<F> BindingMap<Key> for GameScene<F>
    where F: Facade,
{
    fn get_command_from_binding(&mut self, binding: &Key) -> Option<Command> {
        match self.key_bindings.get_action_from_binding(binding) {
            Some(action) => {
                match *action {
                    Action::Camera(ref action) => {
                        match *action {
                            CameraAction::Move(ref direction) => Some(camera::new_move_camera_command(direction, &mut self.camera)),
                        }
                    },
                }
            },
            _ => None,
        }
    }
}

struct Cursor {
    x: f32,
    y: f32,
    pub texture: Rc<Texture>,
}

impl Cursor {
    pub fn new(x: f32, y: f32, texture: Rc<Texture>) -> Self {
        Cursor {
            x: x,
            y: y,
            texture: texture,
        }
    }
}

impl<F, S> Renderable<F, S, sprite::Shader> for Cursor
    where F: Facade,
          S: Surface,
{
    fn draw(&self, renderer: &Renderer<F, sprite::Shader>, surface: &mut S, parent: &Matrix4<f32>) {
        use cgmath::ElementWise;

        use tile::{GetOffset, TextureType};

        let offset = TextureType::Cursor.offset().mul_element_wise(TILE_SIZE);
        let size: Vector2<i32> = (Vector2::new(DISPLAY_SIZE, DISPLAY_SIZE).add_element_wise(MARGIN)).cast();

        let (i, j) = (-self.x as i32, -self.y as i32);
        let (a, b) = (size.x / 2, size.y / 4);
        let x = Vector2::new(a, b) * i;
        let y = Vector2::new(a, -b) * -j;
        let position = (x + y).cast();

        let rectangle = Rectangle::new(
            offset.x as i32,
            offset.y as i32,
            TILE_SIZE as i32,
            TILE_SIZE as i32,
        );
        let mut sprite = Sprite::with_rect(
            self.texture.clone(),
            rectangle,
            TILE_SIZE,
            TILE_SIZE,
        );
        sprite.transform = sprite.transform.position(position.x, position.y, 0.0);
        /*
            .position((x + y).extend(0).cast()) // TODO: get rid of this extend
            .offset(offset.mul_element_wise(TILE_SIZE));
        */

        sprite.draw(renderer, surface, parent);
    }
}

/// Drawable representation of a single cell.
pub struct CellDrawable<'a> {
    pub pos: Point3<i32>,
    pub screen_pos: Point2<i32>,
    pub world: &'a World,
    pub texture: Rc<Texture>,
}

impl<'a, F, S> Renderable<F, S, sprite::Shader> for CellDrawable<'a>
    where F: Facade,
          S: Surface,
{
    fn draw(&self, renderer: &Renderer<F, sprite::Shader>, surface: &mut S, parent: &Matrix4<f32>) {
        self.draw_cell(renderer, surface, parent);
    }
}

impl<'a> CellDrawable<'a> {
    pub fn new(pos: Point3<i32>, screen_pos: Point2<i32>, world: &'a World, texture: Rc<Texture>) -> Self {
        CellDrawable {
            pos: pos,
            screen_pos: screen_pos,
            world: world,
            texture: texture,
        }
    }

    fn draw_cell<F, S>(&self, renderer: &Renderer<F, sprite::Shader>, surface: &mut S, parent: &Matrix4<f32>)
        where F: Facade,
              S: Surface,
    {
        self.draw_terrain(renderer, surface, parent);
    }

    fn draw_terrain<F, S>(&self, renderer: &Renderer<F, sprite::Shader>, surface: &mut S, parent: &Matrix4<f32>)
        where F: Facade,
              S: Surface,
    {
        use cgmath::ElementWise;
        use tile::GetOffset;

        let tile = self.world.area.get_tile(&self.pos);

        // If the tile is see-through, we want to render the tile_type
        // underneath it, instead.
        /*
        if !tile.tile_type.is_solid() {
            tile = self.world.area.get_tile(&(self.pos + Direction::Down.to_vector()));
        }
        */

        // If the tile is see-through, don't render it.
        if !tile.tile_type.is_solid() {
            return;
        }

        // Get the texture offset.
        let offset = tile.tile_type.offset().mul_element_wise(TILE_SIZE);

        let size: Vector2<i32> = (Vector2::new(DISPLAY_SIZE, DISPLAY_SIZE).add_element_wise(MARGIN)).cast();

        let (i, j) = (-self.screen_pos.x, -self.screen_pos.y);
        let (a, b) = (size.x / 2, size.y / 4);
        let x = Vector2::new(a, b) * i;
        let y = Vector2::new(a, -b) * -j;
        let position = (x + y).cast();

        let rectangle = Rectangle::new(
            offset.x as i32,
            offset.y as i32,
            TILE_SIZE as i32,
            TILE_SIZE as i32,
        );
        let mut sprite = Sprite::with_rect(
            self.texture.clone(),
            rectangle,
            TILE_SIZE,
            TILE_SIZE,
        );
        sprite.transform = sprite.transform.position(position.x, position.y, 0.0);
        /*
            .position((x + y).extend(0).cast()) // TODO: get rid of this extend
            .offset(offset.mul_element_wise(TILE_SIZE));
        */

        sprite.draw(renderer, surface, parent);
    }
}
