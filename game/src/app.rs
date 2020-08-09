use derive_macros::*;

use crate::game::TileTextureIndex;
use crate::{game::GameState, gui::*};
use conrod_core::{color::Colorable, input::Key, widget, widget::Widget, Borderable};
use learning_conrod_core::gui::{cache_queued_glyphs, Application, RenderContext, TextureMap, GUI};
use opengl_graphics::GlGraphics;
use piston_window::{Context, Event, Events, Input, PistonWindow, RenderArgs, UpdateArgs};

pub struct GameApp {
    pub(crate) texture_map: TextureMap<GlGraphics, TileTextureIndex>,
    pub(crate) current_menu: MenuState,
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

impl Application<'_> for GameApp {
    type RR = ();
    type IR = ();
    type UR = UpdateAction;
    type GUI = GUI<GameIds>;
    type RP = ();
    type UP = ();

    fn render(
        &self,
        gui: &Self::GUI,
        _rp: &Self::RP,
        gl: &mut GlGraphics,
        context: Context,
        render_context: &mut RenderContext,
        render_args: &RenderArgs,
    ) {
        #[allow(dead_code)]
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        #[allow(dead_code)]
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        //self.texture_map;
        self.current_menu.render(
            &(),
            &self.texture_map,
            gl,
            context,
            render_context,
            render_args,
        );

        // A function used for caching glyphs to the texture cache.
        let cache_queued_glyphs = cache_queued_glyphs(&mut render_context.text_vertex_data);

        gui.draw(
            &mut render_context.text_texture_cache,
            &mut render_context.glyph_cache,
            cache_queued_glyphs,
            context,
            gl,
        );
    }

    fn input(
        &mut self,
        gui: &mut Self::GUI,
        event: Input,
        _event_loop: &mut Events,
        _window: &mut PistonWindow,
    ) {
        if let Some(cr_event) =
            conrod_piston::event::convert(Event::Input(event.clone()), gui.ui.win_w, gui.ui.win_h)
        {
            gui.ui.handle_event(cr_event);
        }

        self.current_menu.handle_input(event);
    }

    fn update(
        &mut self,
        gui: &mut Self::GUI,
        _up: &mut Self::UP,
        update_args: UpdateArgs,
        window: &mut PistonWindow,
    ) -> UpdateAction {
        let mut ui = gui.ui.set_widgets();

        let mut toggle_fullscreen = false;

        use conrod_core::event::{Button, Event, Release, Ui};
        for event in ui.global_input().events() {
            if let Event::Ui(event) = event {
                match event {
                    Ui::Release(
                        _,
                        Release {
                            button: Button::Keyboard(Key::F11),
                            ..
                        },
                    ) => toggle_fullscreen = true,
                    Ui::Release(
                        _,
                        Release {
                            button: Button::Keyboard(Key::Escape),
                            ..
                        },
                    ) => {
                        if let UpdateAction::Close = self.current_menu.handle_esc(window) {
                            return UpdateAction::Close;
                        }
                    }
                    Ui::Release(
                        _,
                        Release {
                            button: Button::Keyboard(Key::F1),
                            ..
                        },
                    ) => {
                        /*if let HUD(state) | OverlayMenu(_, state) = &mut self.gui.active_menu {
                            self.gui.active_menu = GameOnly(state.clone());
                        }*/
                    }
                    _ => (),
                }
            }
        }

        //necessary so that when we stop drawing anything in F1 mode, Resize events will still be processed
        widget::canvas::Canvas::new()
            .border_rgba(0.0, 0.0, 0.0, 0.0)
            .rgba(0.0, 0.0, 0.0, 0.0)
            .set(gui.ids.main_canvas, &mut ui);

        self.current_menu
            .update(&mut (), &mut (ui, &mut gui.ids), update_args, window);

        if toggle_fullscreen {
            gui.toggle_fullscreen(window);
        }

        UpdateAction::Nothing
    }
}

impl GameApp {
    pub fn new(
        texture_map: TextureMap<GlGraphics, TileTextureIndex>,
        init_menu: MenuState,
    ) -> Self {
        GameApp {
            texture_map,
            current_menu: init_menu,
        }
    }
}

pub enum UpdateAction {
    Nothing,
    Close,
}
