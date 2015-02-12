use std::collections::HashMap;
use std::num::{ self, NumCast };

use backend::{ Renderer, RendererTrait };
use gfx_voxel::array::Array;
use noise::{ open_simplex2, Seed };
use utility::{ Bounds, Point };

use terrain::{ Tile, TileType };
use world::CHUNK_SIZE;
use world::chunk::Chunk;

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
            open_simplex2(&self.seed, &[cast::<_, f32>(x as i32 + pos_x * CHUNK_SIZE as i32), cast::<_, f32>(z as i32 + pos_z * CHUNK_SIZE as i32)])
        });

        Chunk::generate(height_map)
    }

    fn get_tile(&self, p: Point, height: usize) -> Tile {
        let chunk_x = p.x / CHUNK_SIZE as i32;
        let chunk_y = p.y / CHUNK_SIZE as i32;
        let tx = ((p.x % CHUNK_SIZE as i32 + CHUNK_SIZE as i32) % CHUNK_SIZE as i32) as usize;
        let ty = ((p.y % CHUNK_SIZE as i32 + CHUNK_SIZE as i32) % CHUNK_SIZE as i32) as usize;

        match self.chunks.get(&(chunk_x, chunk_y)) {
            Some(chunk) => chunk.tiles[height][tx][ty],
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

                renderer.render_obj(pos, display_char);
            }
        }
    }
}

pub fn get_glyph(tile_type: TileType) -> char {
    match tile_type {
        TileType::Air => ' ',
        TileType::Wall => 177u8 as char,
        TileType::OutOfBounds => '?',
    }
}
