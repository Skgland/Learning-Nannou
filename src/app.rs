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

pub struct App {
    pub gui: GUI,
    state: Option<GameState>,
    keys_down: BTreeSet<Key>,
}

impl App {
    pub fn new(gui: GUI) -> Self {
        App { gui, state: None, keys_down: BTreeSet::new() }
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
                GUIVisibility::HUD | GUIVisibility::GameOnly => {
                    // Clear the screen.
                    clear(GREEN, gl);
                }
                _ => {
                    clear(BLUE, gl)
                }
            }

            self.render_game(args, c, gl);

            conrod_piston::draw::primitives(ui.draw(),
                                            c,
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
        use graphics::*;

        if let Some(state) = &self.state {
            let GameState { x_offset, y_offset, rotation, .. } = state;

            const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

            let square = rectangle::square(0.0, 0.0, 50.0);

            let rotation = rotation;
            let (x, y) = (args.width / 2.0,
                          args.height / 2.0);

            let transform = c.transform.trans(x + *x_offset, y + *y_offset).rot_rad(*rotation).trans(-25.0, -25.0);


            //println!("X-off {}, Y-off {}",*x_offset,*y_offset);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        }
    }

    pub fn input(&mut self, event: Input, window: &mut PistonWindow<GlutinWindow>) -> () {
        use piston::input::{Input, Button, Key::*, ButtonArgs};
        use crate::gui::MenuType;

        match (&self.gui.active_menu, event) {
            //Always pass Resize Events to Conrod


            (GUIVisibility::HUD, event @ Input::Resize(..)) |
            (GUIVisibility::GameOnly, event @ Input::Resize(..)) => {
                if let Some(cr_event) = conrod_piston::event::convert(Event::Input(event), self.gui.ui.win_w, self.gui.ui.win_h) {
                    self.gui.ui.handle_event(cr_event);
                }
            }
            (_, Input::Button(ButtonArgs { button: Button::Keyboard(F11), state: ButtonState::Release, .. })) => {
                if self.gui.fullscreen {
                    window.window.window.set_fullscreen(None);
                    self.gui.fullscreen = false;
                } else {
                    let monitor = window.window.window.get_primary_monitor();
                    window.window.window.set_fullscreen(Some(monitor));
                    self.gui.fullscreen = true;
                }
            }
            //This should move to Game Logic once separated
            (GUIVisibility::HUD, Input::Button(ButtonArgs { button: Button::Keyboard(Escape), state: ButtonState::Release, .. })) => {
                self.gui.active_menu = GUIVisibility::OverlayMenu(MenuType::Pause)
            }
            //This should move to Game Logic once separated
            (GUIVisibility::GameOnly, Input::Button(ButtonArgs { button: Button::Keyboard(Escape), state: ButtonState::Release, .. })) => {
                self.gui.active_menu = GUIVisibility::HUD
            }
            //This should move to Game Logic once separated
            (_, Input::Button(ButtonArgs { button: Button::Keyboard(F1), state: ButtonState::Release, .. })) => {
                self.gui.active_menu = GUIVisibility::GameOnly
            }
            //In game Key Processing, may want to do this via Conrod as it tracks this in a maybe nicer way
            (GUIVisibility::GameOnly, Input::Button(ButtonArgs { button: Button::Keyboard(key), state, .. })) |
            (GUIVisibility::HUD, Input::Button(ButtonArgs { button: Button::Keyboard(key), state, .. })) => {
                match state {
                    ButtonState::Press => self.keys_down.insert(key),
                    ButtonState::Release => self.keys_down.remove(&key),
                };
                //println!("{:?}", key);
            }
            //Escape should always bring one back
            (GUIVisibility::MenuOnly(menu), Input::Button(ButtonArgs { button: Button::Keyboard(Escape), state: ButtonState::Release, .. })) |
            (GUIVisibility::OverlayMenu(menu), Input::Button(ButtonArgs { button: Button::Keyboard(Escape), state: ButtonState::Release, .. })) => {
                if let Some(menu) = menu.back() {
                    self.gui.active_menu = menu;
                } else {
                    window.set_should_close(true);
                }
            }
            //while in Menu pass all remaining events to conrod
            (GUIVisibility::OverlayMenu(..), event) |
            (GUIVisibility::MenuOnly(..), event) => {
                if let Some(cr_event) = conrod_piston::event::convert(Event::Input(event), self.gui.ui.win_w, self.gui.ui.win_h) {
                    self.gui.ui.handle_event(cr_event)
                }
            }
            (_, _) => (),
        };
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        use piston::input::Key::*;
        use GUIVisibility::*;

        let ui = &mut self.gui.ui.set_widgets();

        //necessary so that when we stop drawing anything in F1 mode, Resize events will still be processed
        widget::canvas::Canvas::new().border_rgba(0.0, 0.0, 0.0, 0.0).rgba(0.0, 0.0, 0.0, 0.0).set(self.gui.ids.canvas, ui);

        if let Some(state) = &mut self.state {
            // Rotate 2 radians per second.
            state.rotation += 8.0 * args.dt;


            match self.gui.active_menu {
                //update game state while in game
                HUD | GameOnly => {
                    for key in &self.keys_down {
                        match key {
                            Up => state.y_offset -= 0.5,
                            Down => state.y_offset += 0.5,
                            Left => state.x_offset -= 0.5,
                            Right => state.x_offset += 0.5,
                            _ => {}
                        }
                    }
                }
                MenuOnly(..) | OverlayMenu(..) => {}
            }
        }


        match &self.gui.active_menu {
            GameOnly => (),
            HUD => widget::Text::new("HUD").font_size(30).mid_top_of(self.gui.ids.canvas).set(self.gui.ids.title, ui),
            MenuOnly(menu)|
            OverlayMenu(menu) => {
                menu.update(ui,&self.gui.ids);
            }
        }
    }
}
