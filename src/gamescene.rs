use event::Event;
use event::Event::Render;

use gamestate::GameState;
//use map::Map;
use scene::{Scene, BoxedScene};

pub struct GameScene;
    //map: Map,
//}

impl GameScene {
    pub fn new() -> BoxedScene {
        Box::new(GameScene
            //map: Map::new(),
        )
    }
}

impl Scene for GameScene {
    fn handle_event(&mut self, e: &Event, state: &mut GameState) -> Option<BoxedScene> {
        match e {
            &Render(args) => {
                /*let (uic, gl) = state.get_drawables();
                gl.draw([0, 0, args.width as i32, args.height as i32], |ctx, gl| {
                    self.map.render(gl, &ctx);

                    // Draw a background color.
                    uic.background().color(self.bg_color).draw(gl);

                    uic.label("SP")
                        .position(10.0, 30.0)
                        .size(48u32)
                        .color(self.bg_color.plain_contrast())
                        .draw(gl);
                });*/

        /*let conX = 80i32;
        let conY = 50i32;
        let mut con = Console::init_root(conX, conY, "Colonize", false);
        let mut exit = false;
        let mut charX = 40i32;
        let mut charY = 25i32;
        let mut dogX = 10i32;
        let mut dogY = 10i32;
        // render
        render(&mut con, charX, charY, dogX, dogY);
        while !(Console::window_closed() || exit) {
            let keypress = Console::wait_for_keypress(true);

            match keypress.key {
                Special(KeyCode::Escape) => exit = true,
                Special(KeyCode::Up) => {
                    if charY >= 1 {
                        charY -= 1;
                    }
                },
                Special(KeyCode::Down) => {
                    if charY <= (conY - 1) {
                        charY += 1;
                    }
                },
                Special(KeyCode::Left) => {
                    if charX >= 1 {
                        charX -= 1;
                    }
                },
                Special(KeyCode::Right) => {
                    if charX <= (conX - 1) {
                        charX += 1;
                    }
                },
                _ => {}
            }

            let offset_x = std::rand::thread_rng().gen_range(0, 3i32) - 1;
            if (dogX + offset_x) > 0 && (dogX + offset_x) < (conX - 1) {
                dogX += offset_x;
            }

            let offset_y = std::rand::thread_rng().gen_range(0, 3i32) - 1;
            if (dogY + offset_y) > 0 && (dogY + offset_y) < (conY - 1) {
                dogY += offset_y;
            }

            render(&mut con, charX, charY, dogX, dogY);
        }*/

                None
            },
            _ => None
        }
    }
}
