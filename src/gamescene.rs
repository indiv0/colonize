use conrod::{
    Background,
    Color,
    Colorable,
    Drawable,
    Label,
    Labelable,
    Positionable,
};
use event::Event;
use event::Event::Render;

use gamestate::GameState;
use grid::Grid;
use scene::{Scene, BoxedScene};

pub struct GameScene {
    bg_color: Color,
    grid: Grid,
}

impl GameScene {
    pub fn new() -> BoxedScene {
        Box::new(GameScene {
            bg_color: Color::new(0.2, 0.35, 0.45, 1.0),
            grid: Grid::new(),
        })
    }
}

impl Scene for GameScene {
    fn handle_event(&mut self, e: &Event, state: &mut GameState) -> Option<BoxedScene> {
        match e {
            &Render(args) => {
                let (uic, gl) = state.get_drawables();
                gl.draw([0, 0, args.width as i32, args.height as i32], |_, gl| {
                    // Draw a background color.
                    uic.background().color(self.bg_color).draw(gl);

                    uic.label("SP")
                        .position(10.0, 30.0)
                        .size(48u32)
                        .color(self.bg_color.plain_contrast())
                        .draw(gl);
                });

                None
            },
            _ => None
        }
    }
}
