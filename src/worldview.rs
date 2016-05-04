use backend::{ TcodRenderer, Renderer };
use utility::Bounds;
use world::World;

use camera::Camera;

// TODO: replace `TcodRenderer` with `Renderer`
pub fn draw_world(world: &World, renderer: &mut TcodRenderer, bounds: Bounds<i32>, camera: &Camera) {
    let camera_pos = camera.get_position();
    let height = camera.get_height();

    let start_x = camera_pos[0] - bounds.width() / 2;
    let start_z = camera_pos[1] - bounds.height() / 2;

    // TODO: implement proper occlusion culling.
    for x in 0..bounds.width() {
        for z in 0..bounds.height() {
            let tx = x + start_x;
            let tz = z + start_z;

            let tile = world.area.get_tile([tx, tz], *height);

            // If the tile is see-through, we want to render the tile_type
            // underneath it, instead.
            let display_char = if !tile.tile_type.is_solid() && *height > 0 {
                let tile = world.area.get_tile([tx, tz], height - 1);
                tile.tile_type.get_lower_glyph()
            } else {
                tile.tile_type.get_glyph()
            };

            let pos = [x, z];

            renderer.render_obj(pos, display_char);
        }
    }

    draw_cursor(renderer, bounds);
}

pub fn draw_cursor(renderer: &mut TcodRenderer, bounds: Bounds<i32>) {
    let x = bounds.width() / 2;
    let y = bounds.height() / 2;

    renderer.render_obj([x, y], 'C');
}
