use std::collections::HashMap;
use std::rc::Rc;

use cgmath::{EuclideanSpace, Point2, Point3, Vector3};
use graphics;
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
use rgframework::backend::{Backend, Graphics};
use rgframework::backend::graphics::{Context};
use rgframework::draw::Draw;
use utility::Bounds;
use world;
use world::{Direction, World};

use action::Action;
use camera;
use camera::{Camera, CameraAction};
use config::Config;
use localization::Localization;
use scene::MenuScene;
use textures::TextureType;

const CAMERA_INITIAL_POSITION: Point3<i32> = Point3 { x: 0, y: 15, z: 1};
const CAMERA_MOVEMENT_SPEED: Vector3<i32> = Vector3 { x: 1, y: 1, z: 1 };
const CURSOR_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const CURSOR_SIZE: f64 = 16.0;
const TILE_SIZE: f64 = 16.0;

pub struct GameScene<B>
    where B: Backend,
{
    config: Rc<Config>,
    localization: Rc<Localization>,
    key_bindings: BindingsHashMap<Key, Action>,
    mouse_pos: Point2<f64>,
    world: World,
    bounds: Bounds<i32>,
    camera: Camera,
    cursor: Cursor,
    textures: Rc<HashMap<TextureType, B::Texture>>,
}

impl<B> GameScene<B>
    where B: Backend,
{
    pub fn new(config: Rc<Config>, localization: Rc<Localization>, textures: Rc<HashMap<TextureType, B::Texture>>) -> Self {
        Self::new_internal(
            config.clone(),
            localization.clone(),
            config.game_scene_key_bindings.unwrap_bindings(),
            textures,
        )
    }

    fn new_internal(config: Rc<Config>, localization: Rc<Localization>, key_bindings: BindingsHashMap<Key, Action>, textures: Rc<HashMap<TextureType, B::Texture>>) -> Self {
        // TODO: refactor these magic numbers.
        let bounds = Bounds::new(0, 0, 54, 49);
        let cursor = Cursor::new(
            bounds.width() as f64 / 2.0,
            bounds.height() as f64 / 2.0,
        );

        GameScene {
            key_bindings: key_bindings,
            mouse_pos: Point2::origin(),
            localization: localization,
            world: World::new(None, config.initial_world_size),
            config: config,
            bounds: bounds,
            camera: Camera::new(CAMERA_MOVEMENT_SPEED, CAMERA_INITIAL_POSITION),
            cursor: cursor,
            textures: textures,
        }
    }
}

impl<B, E, G> Scene<B, E, G> for GameScene<B>
    where B: Backend + 'static,
          E: GenericEvent,
          G: Graphics<Texture=B::Texture>,
{
    fn to_box(self) -> BoxedScene<B, E, G> {
        Box::new(self)
    }

    fn render(&mut self, context: &Context, graphics: &mut G, glyph_cache: &mut B::CharacterCache) {
        use graphics::{clear, color, Transformed};
        use graphics::text::Text;

        clear(color::WHITE, graphics);

        let camera_pos = self.camera.get_position();

        let start_x = camera_pos.x - self.bounds.width() / 2;
        let start_z = camera_pos.z - self.bounds.height() / 2;

        for x in 0..self.bounds.width() {
            for z in 0..self.bounds.height() {
                let screen_pos = Point2::new(x, z);
                let pos = Point3::new(x + start_x, camera_pos.y, z + start_z);
                let cell_drawable = CellDrawable::new(pos, screen_pos, &self.world, self.textures.clone());
                Draw::<B, G>::draw(&cell_drawable, context, graphics, glyph_cache);
            }
        }

        Draw::<B, G>::draw(&self.cursor, context, graphics, glyph_cache);

        Text::new(self.config.font_size).draw(
            &self.localization.gamescene_welcome_text,
            glyph_cache,
            &context.draw_state,
            context.transform.trans(10.0, 100.0),
            graphics);

        Text::new(self.config.font_size).draw(
            format!("{}: {:?}", self.localization.gamescene_debug_cursor, self.mouse_pos).as_ref(),
            glyph_cache,
            &context.draw_state,
            context.transform.trans(10.0, 150.0),
            graphics);

        Text::new(self.config.font_size).draw(
            format!("{}: {:?}", self.localization.gamescene_debug_camera, self.camera.get_position()).as_ref(),
            glyph_cache,
            &context.draw_state,
            context.transform.trans(10.0, 200.0),
            graphics);

        Text::new(self.config.font_size).draw(
            format!("{}: {:?}", self.localization.gamescene_debug_chunk, world::abs_pos_to_chunk_pos(self.camera.get_position())).as_ref(),
            glyph_cache,
            &context.draw_state,
            context.transform.trans(10.0, 250.0),
            graphics);
    }

    fn handle_event(&mut self, e: &E) -> Option<SceneCommand<B, E, G>> {
        let mut maybe_scene = None;

        e.mouse_cursor(|x, y| {
            self.mouse_pos = Point2::new(x, y);
        });

        e.press(|button_type| {
            if let Keyboard(key) = button_type {
                match key {
                    Key::Backspace => maybe_scene = Some(SceneCommand::SetScene(MenuScene::new(self.config.clone(), self.localization.clone(), self.textures.clone()).to_box())),
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

impl<B> BindingMap<Key> for GameScene<B>
    where B: Backend,
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
    x: f64,
    y: f64,
}

impl Cursor {
    pub fn new(x: f64, y: f64) -> Self {
        Cursor {
            x: x,
            y: y,
        }
    }
}

impl<B, G> Draw<B, G> for Cursor
    where B: Backend,
          G: Graphics<Texture=B::Texture>,
{
    fn draw(&self, context: &Context, graphics: &mut G, _glyph_cache: &mut B::CharacterCache) {
        use graphics::Rectangle;

        Rectangle::new(CURSOR_COLOR).draw(
            [self.x, self.y, CURSOR_SIZE, CURSOR_SIZE],
            &context.draw_state,
            context.transform,
            graphics);
    }
}

/// Drawable representation of a single cell.
pub struct CellDrawable<'a, B>
    where B: Backend,
{
    pub pos: Point3<i32>,
    pub screen_pos: Point2<i32>,
    pub world: &'a World,
    textures: Rc<HashMap<TextureType, B::Texture>>,
}

impl<'a, B, G> Draw<B, G> for CellDrawable<'a, B>
    where B: Backend,
          G: Graphics<Texture=B::Texture>,
{
    fn draw(&self, context: &Context, graphics: &mut G, _glyph_cache: &mut B::CharacterCache) {
        self.draw_cell::<G>(context, graphics);
    }
}

impl<'a, B> CellDrawable<'a, B>
    where B: Backend,
{
    pub fn new(pos: Point3<i32>, screen_pos: Point2<i32>, world: &'a World, textures: Rc<HashMap<TextureType, B::Texture>>) -> Self {
        CellDrawable {
            pos: pos,
            screen_pos: screen_pos,
            world: world,
            textures: textures,
        }
    }

    fn draw_cell<G>(&self, context: &Context, graphics: &mut G)
        where G: Graphics<Texture=B::Texture>,
    {
        self.draw_terrain::<G>(context, graphics);
    }

    fn draw_terrain<G>(&self, context: &Context, graphics: &mut G)
        where G: Graphics<Texture=B::Texture>,
    {
        use graphics::Image;

        let tile = self.world.area.get_tile(&self.pos);

        // If the tile is see-through, we want to render the tile_type
        // underneath it, instead.
        let texture = if tile.tile_type.is_solid() {
            self.textures.get(&TextureType::TileTexture(tile.tile_type))
        } else {
            let tile = self.world.area.get_tile(&(self.pos + Direction::Down.to_vector()));
            self.textures.get(&TextureType::TileTexture(tile.tile_type))
        };

        // Don't draw invisible tiles.
        let texture = match texture {
            Some(texture) => texture,
            None => return,
        };

        let texture_x = self.screen_pos.x as f64 * TILE_SIZE;
        let texture_y = self.screen_pos.y as f64 * TILE_SIZE;
        Image::new()
            .rect(graphics::rectangle::square(texture_x, texture_y, TILE_SIZE))
            .draw(texture, &context.draw_state, context.transform, graphics);
    }
}
