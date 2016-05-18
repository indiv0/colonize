use std::rc::Rc;

use fps_counter;
use glium::{Frame, Surface};
use glium::backend::Facade;
use glium_graphics::GliumWindow;
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
use rgframework::backend::graphics::RenderContext;
use rgframework::manager::Manager;
use rgframework::rendering::{sprite, Renderer, TextRenderer};
use rgframework::texture::Texture;
use time;

use config::Config;
use localization::Localization;
use scene::{MenuScene, SceneContext};

pub struct Game<E, F, S, W>
    where E: GenericEvent,
          F: Facade,
          S: Surface,
{
    fps_counter: fps_counter::FPSCounter,
    scene_manager: SceneManager<E, S>,
    events: WindowEvents,
    window: W,
    context: Rc<SceneContext<F>>,
}

impl<E, F, S, W> Game<E, F, S, W>
    where E: GenericEvent,
          F: Facade + 'static,
          S: Surface,
          W: Window,
{
    pub fn new(config: Config, localization: Rc<Localization>, facade: F, window: W, sprite_renderer: Renderer<F, sprite::Shader>, textures: Manager<F, Texture>, text_renderer: TextRenderer) -> Self {
        let scene_context = Rc::new(SceneContext::new(
                config,
                localization,
                facade,
                sprite_renderer,
                textures,
                text_renderer,
            )
        );

        let mut scene_manager = SceneManager::new();
        scene_manager.push_scene(MenuScene::new(scene_context.clone()).to_box());

        let events = window.events().ups(scene_context.clone().config.ups).max_fps(scene_context.clone().config.max_fps);

        Self::new_internal(scene_context, events, scene_manager, window)
    }

    fn new_internal(context: Rc<SceneContext<F>>, events: WindowEvents, scene_manager: SceneManager<E, S>, window: W) -> Self {
        Game {
            events: events,
            fps_counter: fps_counter::FPSCounter::new(),
            scene_manager: scene_manager,
            window: window,
            context: context,
        }
    }
}

impl<F> Game<Event, F, Frame, GliumWindow>
    where F: Facade,
{
    pub fn run(&mut self, context: &mut RenderContext) {
        while let Some(e) = self.events.next(&mut self.window) {
            use piston::input::Event;

            match e {
                Event::Render(_) => {
                    let mut target = self.window.draw();
                    let start_time = time::precise_time_ns();
                    if let Some(mut scene) = self.scene_manager.pop_scene() {
                        scene.render(&mut target, context);
                        self.scene_manager.push_scene(scene);
                    }
                    let end_time = time::precise_time_ns();
                    target.finish().ok(); // TODO: handle this result better.
                    let frame_end_time = time::precise_time_ns();

                    let fps = self.fps_counter.tick();
                    let fps_info = format!(
                        "{} - {}: {:.2}{unit_millisecond}+{:.2}{unit_millisecond} @ {} {unit_fps}",
                        self.context.localization.colonize_window_title,
                        self.context.localization.debug_render_info,
                        (end_time - start_time) as f64 / 1e6,
                        (frame_end_time - end_time) as f64 / 1e6,
                        fps,
                        unit_millisecond=self.context.localization.util_unit_millisecond,
                        unit_fps=self.context.localization.util_unit_fps,
                    );
                    self.window.set_title(fps_info);
                },
                _ => {
                    self.scene_manager.handle_event(&e);
                }
            }
        }
    }
}
