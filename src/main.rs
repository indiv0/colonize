extern crate event;
extern crate gfx_voxel;
extern crate input;
extern crate noise;
extern crate quack;
extern crate tcod;
extern crate tcod_window;
extern crate window;

use std::cell::RefCell;
use std::rand::Rng;

use event::{ Event, Events, MaxFps, Ups };
use quack::Set;
use tcod_window::TcodWindow;
use tcod::{ BackgroundFlag, Console, KeyCode };
use tcod::Key::Special;
use window::WindowSettings;

// Local imports.
use scene::Scene;
use gamestate::GameState;
use menuscene::MenuScene;

// Modules.
mod chunk;
mod gamescene;
mod gamestate;
mod map;
mod menuscene;
mod scene;

/*fn render(con: &mut Console, x: i32, y: i32, dogX: i32, dogY: i32) {
    con.clear();
    con.put_char(x, y, '@', BackgroundFlag::Set);
    con.put_char(dogX, dogY, 'd', BackgroundFlag::Set);
    Console::flush();
}*/

fn main() {
    let window = TcodWindow::new(
        WindowSettings {
            title: "TcodWindow".to_string(),
            size: [80, 50],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );
    let ref window = RefCell::new(window);
    let mut event_iter = Events::new(window).set(Ups(180)).set(MaxFps(60));

    let mut gamestate = GameState::new();
    let mut current_scene = MenuScene::new();

    for e in event_iter {
        use input::Button::Keyboard;
        use input::Input::{ Move, Press };
        use input::keyboard::Key;
        use input::Motion::MouseCursor;

        match e {
            Event::Render(_) => {},
            Event::Update(_) => {},
            Event::Input(Press(Keyboard(key))) => {
                println!("Pressed: {:?}", key);
            },
            Event::Input(Move(MouseCursor(x, y))) => {
                println!("Moved mouse: {} {}", x, y);
            },
            Event::Input(Press(input::Button::Mouse(button))) => {
                println!("Mouse: {:?}", button);
            },
            _ => {},
        }
    }

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
}
