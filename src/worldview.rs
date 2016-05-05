extern crate tcod;

use cgmath::{Point2, Point3};
use self::tcod::Color;

use backend::{Renderer, TcodRenderer};
use utility::Bounds;
use world::{Direction, TileType, World};

use camera::Camera;
use drawable::Drawable;

// TODO: replace `TcodRenderer` with `Renderer`
pub fn draw_world(world: &World, renderer: &mut TcodRenderer, bounds: Bounds<i32>, camera: &Camera) {
    let camera_pos = camera.get_position();

    let start_x = camera_pos.x - bounds.width() / 2;
    let start_z = camera_pos.z - bounds.height() / 2;

    // TODO: implement proper occlusion culling.
    for x in 0..bounds.width() {
        for z in 0..bounds.height() {
            let screen_pos = Point2::new(x, z);
            let pos = Point3::new(x + start_x, camera_pos.y, z + start_z);
            let cell_drawable = CellDrawable::new(pos, world);
            cell_drawable.draw(renderer, screen_pos);
        }
    }

    draw_cursor(renderer, bounds);
}

pub fn draw_cursor(renderer: &mut TcodRenderer, bounds: Bounds<i32>) {
    let x = bounds.width() / 2;
    let y = bounds.height() / 2;

    renderer.render_obj(Point2::new(x, y), '@', None);
}

/// Drawable representation of a single cell.
pub struct CellDrawable<'a> {
    pub pos: Point3<i32>,
    // TODO: find a way to avoid borrowing the world here.
    pub world: &'a World,
}

impl<'a> Drawable for CellDrawable<'a> {
    fn draw(&self, renderer: &mut TcodRenderer, offset: Point2<i32>) {
        self.draw_cell(renderer, offset);
    }
}

impl<'a> CellDrawable<'a> {
    pub fn new(pos: Point3<i32>, world: &'a World) -> Self {
        CellDrawable {
            pos: pos,
            world: world,
        }
    }

    fn draw_cell(&self, renderer: &mut TcodRenderer, offset: Point2<i32>) {
        self.draw_terrain(renderer, offset);
    }

    fn draw_terrain(&self, renderer: &mut TcodRenderer, offset: Point2<i32>) {
        let tile = self.world.area.get_tile(&self.pos);

        // If the tile is see-through, we want to render the tile_type
        // underneath it, instead.
        let (display_char, color) = if tile.tile_type.is_solid() {
            (
                get_glyph(&tile.tile_type),
                get_color(&tile.tile_type),
            )
        } else {
            let tile = self.world.area.get_tile(&(self.pos + Direction::Down.to_vector()));
            (
                get_lower_glyph(&tile.tile_type),
                get_color(&tile.tile_type),
            )
        };

        renderer.render_obj(offset, display_char, color);
    }
}

fn get_color(tile_type: &TileType) -> Option<Color> {
    use self::tcod::colors::*;

    match *tile_type {
        TileType::Air | TileType::OutOfBounds => None,
        TileType::Grass => Some(GREEN),
        TileType::Sand => Some(YELLOW),
        TileType::Soil => Some(DARKER_SEPIA),
        TileType::Wall => Some(DARK_GREY),
        TileType::Water => Some(BLUE),
    }
}

fn get_glyph(tile_type: &TileType) -> char {
    match *tile_type {
        TileType::Air => ' ',
        TileType::Grass | TileType::Sand | TileType::Soil => 178u8 as char,
        TileType::OutOfBounds => '?',
        TileType::Wall => 177u8 as char,
        TileType::Water => '=',
    }
}

/// Returns the glyph for a tile which is a level lower than the rendered
/// level.
fn get_lower_glyph(tile_type: &TileType) -> char {
    match *tile_type {
        TileType::Air => ' ',
        TileType::Grass | TileType::Sand | TileType::Soil => ',',
        TileType::OutOfBounds => '?',
        TileType::Wall => '.',
        TileType::Water => '~',
    }
}
