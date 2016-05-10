use std::collections::HashMap;
use std::rc::Rc;

use fps_counter;
use opengl_graphics::GlGraphics;
use piston::event_loop::{
    EventLoop,
    Events,
    WindowEvents,
};
use piston::input::{Event, GenericEvent};
use piston::window::{
    AdvancedWindow,
    Window,
};
use rgframework::{Scene, SceneManager};
use rgframework::backend::{Backend, Graphics};
use time;

use backend::GlBackend;
use config::Config;
use localization::Localization;
use scene::MenuScene;
use textures::TextureType;

pub struct Game<B, E, G, W>
    where B: Backend,
          E: GenericEvent,
          G: Graphics<Texture=B::Texture>,
          W: AdvancedWindow + Window,
{
    localization: Rc<Localization>,
    fps_counter: fps_counter::FPSCounter,
    scene_manager: SceneManager<B, E, G>,
    events: WindowEvents,
    window: W,
}

impl<B, E, G, W> Game<B, E, G, W>
    where B: Backend + 'static,
          E: GenericEvent,
          G: Graphics<Texture=B::Texture>,
          W: AdvancedWindow + Window,
{
    pub fn new(config: Config, localization: Localization, window: W, textures: HashMap<TextureType, B::Texture>) -> Self {
        let config = Rc::new(config);
        let localization = Rc::new(localization);
        let textures = Rc::new(textures);

        let mut scene_manager = SceneManager::new();
        scene_manager.push_scene(MenuScene::new(config.clone(), localization.clone(), textures.clone()).to_box());

        let events = window.events().ups(config.ups).max_fps(config.max_fps);

        Self::new_internal(events, localization, scene_manager, window)
    }

    fn new_internal(events: WindowEvents, localization: Rc<Localization>, scene_manager: SceneManager<B, E, G>, window: W) -> Self {
        Game {
            events: events,
            fps_counter: fps_counter::FPSCounter::new(),
            scene_manager: scene_manager,
            window: window,
            localization: localization,
        }
    }
}

impl<W> Game<GlBackend, Event<W::Event>, GlGraphics, W>
    where W: AdvancedWindow + Window,
          W::Event: GenericEvent,
{
    pub fn run(&mut self, gl: &mut GlGraphics, glyph_cache: &mut <GlBackend as Backend>::CharacterCache) {
        while let Some(e) = self.events.next(&mut self.window) {
            use piston::input::Event;

            match e {
                Event::Render(args) => {
                    let start_time = time::precise_time_ns();
                    if let Some(mut scene) = self.scene_manager.pop_scene() {
                        gl.draw(args.viewport(), |c, gl| scene.render(&c, gl, glyph_cache));
                        self.scene_manager.push_scene(scene);
                    }
                    let end_time = time::precise_time_ns();

                    let fps = self.fps_counter.tick();
                    let title = format!(
                        "{}: {:.2}{unit_millisecond} @ {} {unit_fps}",
                        self.localization.colonize_window_title,
                        (end_time - start_time) as f64 / 1e6,
                        fps,
                        unit_millisecond=self.localization.util_unit_millisecond,
                        unit_fps=self.localization.util_unit_fps,
                    );
                    self.window.set_title(title);
                },
                _ => {
                    self.scene_manager.handle_event(&e);
                }
            }
        }
    }
}
