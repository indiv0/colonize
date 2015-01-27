pub struct Grid {
    grid: Vec<Vec<Vec<Block>>>,
}

pub struct Block {
    pub loc: Location,
}

pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            grid: vec![vec![vec![]]]
        }
    }
}
