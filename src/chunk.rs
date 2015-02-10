use backend::{ Renderer, RendererTrait };
use gfx_voxel::array::Array;
use utility::Point;

pub const SIZE: usize = 16;

#[derive(Copy)]
pub struct BlockState {
    pub value: u16
}

pub const EMPTY_BLOCK: BlockState = BlockState { value: 0 };

pub struct Chunk {
    pub blocks: [[[BlockState; SIZE]; SIZE]; SIZE]
}

pub const EMPTY_CHUNK: &'static Chunk = &Chunk {
    blocks: [[[EMPTY_BLOCK; SIZE]; SIZE]; SIZE]
};

fn array_16x16x16<T, F>(mut f: F) -> [[[T; SIZE]; SIZE]; SIZE]
    where F: FnMut(usize, usize, usize) -> T
{
    Array::from_fn(|y| -> [[T; SIZE]; SIZE]
        Array::from_fn(|z| -> [T; 16]
            Array::from_fn(|x| f(x, y, z))
        )
    )
}

impl Chunk {
    pub fn generate(height_map: [[f32; SIZE]; SIZE]) -> Chunk {
        Chunk {
            blocks: array_16x16x16(|x, y, z| {
                let height = (height_map[x][z] * SIZE as f32) as usize;
                BlockState {
                    value: match height {
                            h if h < y => 0,
                            _ => 1,
                        }
                }
            }),
        }
    }

    pub fn render(&self, renderer: &mut Renderer, x_offset: i32, z_offset: i32, height: usize) {
        for z in (0..SIZE) {
            for x in (0..SIZE) {
                let display_char = match self.blocks[height][z][x].value {
                    0 => ' ',
                    1 => 177u8 as char,
                    _ => '?',
                };
                renderer.render_obj(Point { x: x as i32 + x_offset, y: z as i32 + z_offset }, display_char);
            }
        }
    }
}
