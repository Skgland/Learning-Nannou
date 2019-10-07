#![allow(dead_code, unused_variables)]


use derive_macros::*;

use crate::{gui::*,  game::GameState, TextureMap};
use conrod_core::{
    Borderable,
    color::Colorable,
    widget,
    widget::Widget,
};

use opengl_graphics::GlGraphics;
use piston_window::{texture::UpdateTexture, PistonWindow};
use piston::input::{Input, UpdateArgs, RenderArgs, Event};
use glutin_window::GlutinWindow;
use conrod_core::input::Key;
use learning_conrod_base::{RenderContext, Application};
use piston::event_loop::{Events};

pub struct App {
    pub gui: GUI,
    pub texture_map: TextureMap<opengl_graphics::GlGraphics>,
    pub current_menu: MenuState,
}

#[derive(Bounded)]
pub enum Action {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Action {
    pub fn perform(&self, state: &mut GameState) {
        if let GameState::GameState { position, .. } = state {
            match self {
                Action::UP => position.y -= 0.5 / 64.0,
                Action::DOWN => position.y += 0.5 / 64.0,
                Action::LEFT => position.x -= 0.5 / 64.0,
                Action::RIGHT => position.x += 0.5 / 64.0,
            }
        }
    }
}

type G = opengl_graphics::GlGraphics;


impl Application for App {
    type RR = ();
    type IR = ();
    type UR = UpdateAction;

    fn render(&self, context: &mut RenderContext<G>, args: &RenderArgs) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];



        // Specify how to get the drawable texture from the image. In this case, the image
        // *is* the texture.
        fn texture_from_image<T>(img: &T) -> &T { img }

        let RenderContext { gl, glyph_cache, text_texture_cache, text_vertex_data, .. } = context;

        let App { gui: GUI { ui, image_map, .. }, .. } = self;


        // A function used for caching glyphs to the texture cache.
        let cache_queued_glyphs = |_graphics: &mut GlGraphics, cache: &mut opengl_graphics::Texture, rect: conrod_core::text::rt::Rect<u32>, data: &[u8]| {
            let offset = [rect.min.x, rect.min.y];
            let size = [rect.width(), rect.height()];
            let format = piston_window::texture::Format::Rgba8;
            text_vertex_data.clear();
            text_vertex_data.extend(data.iter().flat_map(|&b| vec![255, 255, 255, b]));
            UpdateTexture::update(cache, &mut (), format, &text_vertex_data[..], offset, size)
                .expect("failed to update texture")
        };


        gl.draw(args.viewport(), |c, gl| {
            self.current_menu.draw_raw(args, c, gl, &self.texture_map);

            let view = c.store_view();

            conrod_piston::draw::primitives(ui.draw(),
                                            view,
                                            gl,
                                            text_texture_cache,
                                            glyph_cache,
                                            image_map,
                                            cache_queued_glyphs,
                                            texture_from_image,
            );
        });
    }

    fn input(&mut self, event: Input, event_loop: &mut Events, window: &mut PistonWindow<GlutinWindow>) {
        if let Some(cr_event) = conrod_piston::event::convert(Event::Input(event.clone()), self.gui.ui.win_w, self.gui.ui.win_h) {
            self.gui.ui.handle_event(cr_event);
        }

        self.current_menu.handle_input(event);

    }

    fn update(&mut self, args: &UpdateArgs, window: &mut PistonWindow<GlutinWindow>) -> UpdateAction {
        let ui = &mut self.gui.ui.set_widgets();

        use conrod_core::event::{Event, Ui, Release, Button};
        for event in ui.global_input().events() {
            if let Event::Ui(event) = event {
                match event {
                    Ui::Release(_, Release { button: Button::Keyboard(Key::F11), .. }) => { Self::toggle_fullscreen(window, &mut self.gui.fullscreen) }
                    Ui::Release(_, Release { button: Button::Keyboard(Key::Escape), .. }) => {
                        if let UpdateAction::Close = self.current_menu.handle_esc(window) {
                            return UpdateAction::Close
                        }
                    }
                    Ui::Release(_, Release { button: Button::Keyboard(Key::F1), .. }) => {
                        /*if let HUD(state) | OverlayMenu(_, state) = &mut self.gui.active_menu {
                            self.gui.active_menu = GameOnly(state.clone());
                        }*/
                    }
                    _ => ()
                }
            }
        }

        //necessary so that when we stop drawing anything in F1 mode, Resize events will still be processed
        widget::canvas::Canvas::new().border_rgba(0.0, 0.0, 0.0, 0.0).rgba(0.0, 0.0, 0.0, 0.0).set(self.gui.ids.main_canvas, ui);

        self.current_menu.update(ui, &mut self.gui.ids, args);

        UpdateAction::Nothing
    }
}

impl App {

    pub fn new(gui: GUI, texture_map: TextureMap<G>, init_menu:MenuState) -> Self {
        App { gui, texture_map, current_menu:init_menu}
    }

    pub fn toggle_fullscreen(window: &mut PistonWindow<GlutinWindow>, current: &mut bool) {
        //TODO how to do this again
        if *current {
            //window.window.window.set_fullscreen(None)
            *current = false;
        } else {
            //window.window.window.set_fullscreen(Some(monitor));
            *current = true;
        }
    }




}

pub enum UpdateAction {
    Nothing,
    Close
}
