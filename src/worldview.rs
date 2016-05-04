use cgmath::{Point2, Point3};

use backend::{ TcodRenderer, Renderer };
use utility::Bounds;
use world::{Direction, World};

use camera::Camera;

// TODO: replace `TcodRenderer` with `Renderer`
pub fn draw_world(world: &World, renderer: &mut TcodRenderer, bounds: Bounds<i32>, camera: &Camera) {
    let camera_pos = camera.get_position();

    let start_x = camera_pos.x - bounds.width() / 2;
    let start_z = camera_pos.z - bounds.height() / 2;

    // TODO: implement proper occlusion culling.
    for x in 0..bounds.width() {
        for z in 0..bounds.height() {
            let tile_pos = Point3::new(x + start_x, camera_pos.y, z + start_z);

            let tile = world.area.get_tile(tile_pos);

            // If the tile is see-through, we want to render the tile_type
            // underneath it, instead.
            let display_char = if tile.tile_type.is_solid() {
                tile.tile_type.get_glyph()
            } else {
                let tile = world.area.get_tile(tile_pos + Direction::Down.to_vector());
                tile.tile_type.get_lower_glyph()
            };

            let pos = Point2::new(x, z);

            renderer.render_obj(pos, display_char);
        }
    }

    draw_cursor(renderer, bounds);
}

pub fn draw_cursor(renderer: &mut TcodRenderer, bounds: Bounds<i32>) {
    let x = bounds.width() / 2;
    let y = bounds.height() / 2;

    renderer.render_obj(Point2::new(x, y), 'C');
}
