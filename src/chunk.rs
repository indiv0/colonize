use gfx_voxel::array::Array;
use graphics::Context;
use opengl_graphics::Gl;

static SIZE: usize = 16;

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
/*
fn array_16x16x16<T, F>(mut f: F) -> [[[f32; SIZE]; SIZE]; SIZE]
    where F: FnMut(usize, usize, usize) -> T
{
    Array::from_fn(|y| -> [[T; SIZE]; SIZE]
        Array::from_fn(|z| -> [T; SIZE]
            Array::from_fn(|x| -> f(x, y, z))
        )
    )
}*/

impl Chunk {
    pub fn generate<H>(height_map: [[f32; SIZE]; SIZE]) -> Chunk {
        Chunk {
            blocks: array_16x16x16(|x, y, z| {
                let height = height_map[x][z] * SIZE;
                BlockState {
                    value: match height {
                            h if h < y => 0,
                            _ => 1,
                        }
                }
            }),
        }
    }

    pub fn render(&self, gl: &mut Gl, ctx: &Context) {
        for column in self.blocks.iter() {
            for row in column.iter() {
                for block in row.iter() {
                    block.render(gl, ctx);
                }
            }
        }
    }
}
