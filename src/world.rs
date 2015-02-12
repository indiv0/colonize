use std::collections::HashMap;
use std::num::{ self, NumCast };

use backend::{ Renderer, RendererTrait };
use gfx_voxel::array::Array;
use noise::{ open_simplex2, Seed };
use utility::{ Bounds, Point };

use chunk::{ self, Chunk, Tile, TileType };

static SEED: u32 = 0;

pub struct World {
    chunks: HashMap<(i32, i32), Chunk>,
    seed: Seed,
}

fn array_16x16<T, F>(mut f: F) -> [[T; 16]; 16]
    where F: FnMut(usize, usize) -> T
{
    Array::from_fn(|z| -> [T; 16]
        Array::from_fn(|x| f(x, z))
    )
}

fn cast<T: NumCast, R: NumCast>(val: T) -> R {
    num::cast(val).unwrap()
}

impl World {
    pub fn new() -> World {
        World {
            chunks: HashMap::new(),
            seed: Seed::new(SEED),
        }
    }

    pub fn add_chunk(&mut self, x: i32, z: i32, c: Chunk) {
        self.chunks.insert((x, z), c);
    }

    pub fn generate_chunk(&self, pos_x: i32, pos_z: i32) -> Chunk {
        let height_map = array_16x16(|x, z| {
            open_simplex2(&self.seed, &[cast::<_, f32>(x as i32 + pos_x * chunk::SIZE as i32), cast::<_, f32>(z as i32 + pos_z * chunk::SIZE as i32)])
        });

        Chunk::generate(height_map)
    }

    fn get_tile(&self, p: Point, height: usize) -> Tile {
        let chunk_x = p.x / chunk::SIZE as i32;
        let chunk_y = p.y / chunk::SIZE as i32;
        let tx = ((p.x % chunk::SIZE as i32 + chunk::SIZE as i32) % chunk::SIZE) as usize;
        let ty = ((p.y % chunk::SIZE as i32 + chunk::SIZE as i32) % chunk::SIZE) as usize;

        match self.chunks.get(&(chunk_x, chunk_y)) {
            Some(chunk) => chunk.tiles[tx][ty][height],
            None => Tile::new(TileType::OutOfBounds)
        }
    }

    pub fn render(&self, renderer: &mut Renderer, bounds: Bounds, camera_pos: Point, height: usize) {
        let start_x = camera_pos.x - bounds.width() / 2;
        let start_y = camera_pos.y - bounds.height() / 2;

        for x in (0 .. bounds.width()) {
            for y in (0 .. bounds.height()) {
                let tx = x + start_x;
                let ty = y + start_y;
                let tile = self.get_tile(Point { x: tx, y: ty }, height);
                let pos = Point { x: x, y: y };
                let display_char = get_glyph(tile.tile_type);

                renderer.render_obj(Point { x: x, y: y }, display_char);
                /*let x_offset = x * chunk::SIZE as i32 + camera_pos.x;
                let z_offset = y * chunk::SIZE as i32 + camera_pos.y;

                if bounds.contains(Point { x: x_offset, y: z_offset }) {
                    chunk.render(renderer, x_offset, z_offset, height);
                }*/
            }
        }

        /*for (&(x, y), chunk) in self.chunks.iter() {
        }*/
    }
}

pub fn get_glyph(tile_type: TileType) -> char {
    match tile_type {
        TileType::Air => ' ',
        TileType::Wall => 177u8 as char,
        TileType::OutOfBounds => '?',
    }
}
