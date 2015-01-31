extern crate gfx_voxel;
extern crate tcod;

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

fn main() {
    let mut con = Console::init_root(80, 50, "Colonize", false);
    let mut exit = false;
    while !(Console::window_closed() || exit) {
        con.clear();
        con.put_char(40, 25, '@', BackgroundFlag::Set);
        Console::flush();
        let keypress = Console::wait_for_keypress(true);
        match keypress.key {
            Special(KeyCode::Escape) => exit = true,
            _ => {}
        }
    }
}
