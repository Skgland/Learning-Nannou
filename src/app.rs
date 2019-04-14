#![allow(dead_code,unused_variables)]

use std::collections::btree_set::BTreeSet;

use conrod_core::input::RenderArgs;
use conrod_core::position::Positionable;
use conrod_core::color::Colorable;
use conrod_core::Borderable;
use conrod_core::widget;
use conrod_core::widget::Widget;
use opengl_graphics::GlGraphics;
use piston::input::*;
pub use piston::window::*;
use piston_window::texture::UpdateTexture;

use crate::game::GameState;
use crate::gui::*;
use piston_window::PistonWindow;
use glutin_window::GlutinWindow;
use graphics::Context;
use crate::gui::GUIVisibility::HUD;
use crate::gui::GUIVisibility::GameOnly;
use crate::gui::GUIVisibility::OverlayMenu;
use std::collections::btree_map::BTreeMap;
use crate::game::TileTextureIndex;
use opengl_graphics::Texture;
use crate::game::LevelTemplate;

pub struct App {
    pub gui: GUI,
    pub texture_map: BTreeMap<TileTextureIndex,Texture>,
    pub level_list: Vec<LevelTemplate>,
    keys_down: BTreeSet<Key>,
}

pub enum Action {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Action {
    pub fn perform(&self, state: &mut GameState) {
        match self {
            Action::UP => state.position.y -= 0.5,
            Action::DOWN => state.position.y += 0.5,
            Action::LEFT => state.position.x -= 0.5,
            Action::RIGHT => state.position.x += 0.5,
        }
    }
}

impl App {
    pub fn new(gui: GUI, texture_map: BTreeMap<TileTextureIndex,Texture>, level_list:Vec<LevelTemplate>) -> Self {
        App { gui, keys_down: BTreeSet::new(), texture_map , level_list}
    }

    pub fn render(&self, context: &mut RenderContext, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];



        // Specify how to get the drawable texture from the image. In this case, the image
        // *is* the texture.
        fn texture_from_image<T>(img: &T) -> &T { img }

        let RenderContext { gl, glyph_cache, text_texture_cache, text_vertex_data, .. } = context;

        let App { gui: GUI { ui, image_map, active_menu, .. }, .. } = self;


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
            match active_menu {
                GUIVisibility::HUD(_) | GUIVisibility::GameOnly(_) => {
                    // Clear the screen.
                    clear(super::game::color::D_RED, gl);
                }
                _ => {
                    clear(BLUE, gl)
                }
            }

            self.render_game(args, c, gl);


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


    fn render_game(&self, args: &RenderArgs, c: Context, gl: &mut GlGraphics) {
        if
        let HUD(state)
        |GameOnly(state)| OverlayMenu(_, state) = &self.gui.active_menu {
            let GameState {  level_state, .. } = &state;

            let (x, y) = (args.width / 2.0,
                          args.height / 2.0);

            let c = c.trans(x,y);


            for (coord, tile) in &level_state.tile_map {
                tile.draw_tile(c, gl, &self.texture_map, coord, &state);
            }

            // Draw a box rotating around the middle of the screen.

            state.draw_player(c, gl, &self.texture_map);

        }

        use graphics::*;
    }

    pub fn input(&mut self, event: Input, window: &mut PistonWindow<GlutinWindow>) -> () {
        use piston::input::{Input, Button, ButtonArgs};

        if let Some(cr_event) = conrod_piston::event::convert(Event::Input(event.clone()), self.gui.ui.win_w, self.gui.ui.win_h) {
            self.gui.ui.handle_event(cr_event);
        }

        match (&self.gui.active_menu, event) {
            (GUIVisibility::GameOnly(_), Input::Button(ButtonArgs { button: Button::Keyboard(key), state, .. })) |
            (GUIVisibility::HUD(_), Input::Button(ButtonArgs { button: Button::Keyboard(key), state, .. })) => {
                match state {
                    ButtonState::Press => self.keys_down.insert(key),
                    ButtonState::Release => self.keys_down.remove(&key),
                };
                //println!("{:?}", key);
            }
            (_, _) => (),
        };
    }


    pub fn toggle_fullscreen(window: &mut PistonWindow<GlutinWindow>, current: &mut bool) {
        if *current {
            window.window.window.set_fullscreen(None);
            *current = false;
        } else {
            let monitor = window.window.window.get_primary_monitor();
            window.window.window.set_fullscreen(Some(monitor));
            *current = true;
        }
    }

    pub fn update(&mut self, args: &UpdateArgs, window: &mut PistonWindow<GlutinWindow>) {
        use GUIVisibility::*;


        let ui = &mut self.gui.ui.set_widgets();

        {
            use conrod_core::event::{Event, Ui, Release, Button};
            for event in ui.global_input().events() {
                if let Event::Ui(event) = event {
                    match event {
                        Ui::Release(_, Release { button: Button::Keyboard(Key::F11), .. }) => { Self::toggle_fullscreen(window, &mut self.gui.fullscreen) }
                        Ui::Release(_, Release { button: Button::Keyboard(Key::Escape), .. }) => {
                            self.gui.active_menu.handle_esc(window);
                        }
                        Ui::Release(_, Release { button: Button::Keyboard(Key::F1), .. }) => {
                            if
                            let HUD(state) | OverlayMenu(_, state) = &mut self.gui.active_menu {
                                self.gui.active_menu = GameOnly(state.clone());
                            }
                        }
                        _ => ()
                    }
                }
            }
        }

        //necessary so that when we stop drawing anything in F1 mode, Resize events will still be processed
        widget::canvas::Canvas::new().border_rgba(0.0, 0.0, 0.0, 0.0).rgba(0.0, 0.0, 0.0, 0.0).set(self.gui.ids.main_canvas, ui);

        // Rotate 2 radians per second.

        let mut key_map: BTreeMap<Key, Action> = BTreeMap::new();

        key_map.insert(Key::W, Action::UP);
        key_map.insert(Key::A, Action::LEFT);
        key_map.insert(Key::S, Action::DOWN);
        key_map.insert(Key::D, Action::RIGHT);


        match &mut self.gui.active_menu {
            //update game state while in game
            HUD(state) | GameOnly(state) => {
                state.rotation += 8.0 * args.dt;
                let keys_down = &self.keys_down;
                key_map.iter().filter(|(&k, _)| keys_down.contains(&k)).for_each(|(_, action)| action.perform(state));
            }
            MenuOnly(..) | OverlayMenu(..) => {}
        }


        match &self.gui.active_menu {
            GameOnly(_) => (),
            HUD(_) => widget::Text::new("HUD").font_size(30).mid_top_of(self.gui.ids.main_canvas).set(self.gui.ids.menu_title, ui),
            MenuOnly(menu) |
            OverlayMenu(menu, _) => {
                if let Some(menu) = menu.update(ui, &mut self.gui.ids) {
                    self.gui.active_menu = menu;
                }
            }
        }
    }
}
