extern crate gfx_voxel;
extern crate tcod;

use std::rand::Rng;
use tcod::{BackgroundFlag, Console, KeyCode};
use tcod::Key::Special;

// Local imports.
//use scene::Scene;

// Modules.
/*d chunk;
mod gamescene;
mod gamestate;
mod map;
mod menuscene;
mod scene;*/

fn render(con: &mut Console, x: i32, y: i32, dogX: i32, dogY: i32) {
    con.clear();
    con.put_char(x, y, '@', BackgroundFlag::Set);
    con.put_char(dogX, dogY, 'd', BackgroundFlag::Set);
    Console::flush();
}

fn main() {
    let conX = 80i32;
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
    }
}
