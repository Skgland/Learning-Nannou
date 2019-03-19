use conrod_core::input::RenderArgs;
use opengl_graphics::GlGraphics;
use crate::gui::GUI;
use crate::gui::GUIVisibility;
use conrod_core::widget;
use conrod_core::position::Positionable;
use piston_window::texture::UpdateTexture;
use conrod_core::widget::Widget;
use piston::input::*;

use crate::game::GameState;
use std::collections::btree_set::BTreeSet;
pub use piston::window::*;

pub struct App<'font> {
    // OpenGL drawing backend.
    gl: GlGraphics,
    pub gui: GUI<'font>,
    state: GameState,
    keys_down: BTreeSet<Key>,
}

impl<'font> App<'font> {
    pub fn new(gl: GlGraphics, gui: GUI<'font>, state: GameState) -> Self {
        App { gl, gui, state, keys_down: BTreeSet::new() }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.state.rotation;
        let (x, y) = (args.width / 2.0,
                      args.height / 2.0);


        // Specify how to get the drawable texture from the image. In this case, the image
        // *is* the texture.
        fn texture_from_image<T>(img: &T) -> &T { img }

        let App {
            gui: GUI { ui, text_vertex_data, text_texture_cache, glyph_cache, image_map, .. },
            state: GameState {
                x_offset,
                y_offset,
                ..
            },
            ..
        } = self;

        let gl = &mut self.gl;

        // A function used for caching glyphs to the texture cache.
        let cache_queued_glyphs = |_graphics: &mut GlGraphics,
                                   cache: &mut opengl_graphics::Texture,
                                   rect: conrod_core::text::rt::Rect<u32>,
                                   data: &[u8]|
            {
                let offset = [rect.min.x, rect.min.y];
                let size = [rect.width(), rect.height()];
                let format = piston_window::texture::Format::Rgba8;
                text_vertex_data.clear();
                text_vertex_data.extend(data.iter().flat_map(|&b| vec![255, 255, 255, b]));
                UpdateTexture::update(cache, &mut (), format, &text_vertex_data[..], offset, size)
                    .expect("failed to update texture")
            };


        gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(x + *x_offset, y + *y_offset)
                             .rot_rad(rotation)
                             .trans(-25.0, -25.0);

            //println!("X-off {}, Y-off {}",*x_offset,*y_offset);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);

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

    pub fn input<W: Window>(&mut self, event: Input, window: &mut W) {
        use piston::input::{Input, Button, Key::*, ButtonArgs};

        match self.gui.visibility {
            GUIVisibility::HIDDEN | GUIVisibility::HUD => {
                match event {
                    Input::Button(ButtonArgs { button: Button::Keyboard(key), state, .. }) => {
                        match (key, state) {
                            (Escape, ButtonState::Release) => match self.gui.visibility {
                                GUIVisibility::HUD => self.gui.visibility = GUIVisibility::MENU,
                                GUIVisibility::HIDDEN => self.gui.visibility = GUIVisibility::HUD,
                                _ => (),
                            },
                            (F1, ButtonState::Release) => {
                                self.gui.visibility = GUIVisibility::HIDDEN;
                            }
                            _ => (),
                        }
                        match state {
                            ButtonState::Press => self.keys_down.insert(key),
                            ButtonState::Release => self.keys_down.remove(&key),
                        };

                        //println!("{:?}", key);

                    }
                    Input::Resize(..) => {
                        if let Some(cr_event) = conrod_piston::event::convert(Event::Input(event), self.gui.ui.win_w, self.gui.ui.win_h) {
                            self.gui.ui.handle_event(cr_event)
                        }
                    }
                    _other => {}
                }
            }
            GUIVisibility::MENU | GUIVisibility::FULL => {
                match event {
                    Input::Button(ButtonArgs { button: Button::Keyboard(Escape), state: ButtonState::Release, .. }) => {
                        match self.gui.visibility {
                            GUIVisibility::MENU => self.gui.visibility = GUIVisibility::FULL,
                            GUIVisibility::FULL => {
                                self.gui.visibility = GUIVisibility::FULL;
                                window.set_should_close(true);
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                }

                if let Some(cr_event) = conrod_piston::event::convert(Event::Input(event), self.gui.ui.win_w, self.gui.ui.win_h) {
                    self.gui.ui.handle_event(cr_event)
                }
            }
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        use piston::input::{Key::*};

// Rotate 2 radians per second.
        self.state.rotation += 8.0 * args.dt;
        let ui = &mut self.gui.ui.set_widgets();

        for key in &self.keys_down {
            match key {
                Up => self.state.y_offset -= 0.5,
                Down => self.state.y_offset += 0.5,
                Left => self.state.x_offset -= 0.5,
                Right => self.state.x_offset += 0.5,
                _ => {}
            }
        }


//widget::Canvas::new().pad(30.0).scroll_kids_vertically().rgba(0.0,0.0,0.0,0.0).set(self.ids.canvas, ui);
        if self.gui.visibility != GUIVisibility::HIDDEN {
            widget::Text::new(format!("{}", self.gui.visibility).as_str()).font_size(30).mid_top().set(self.gui.ids.title, ui);
        }
    }
}
