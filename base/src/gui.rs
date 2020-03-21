use super::*;
use conrod_core::{widget, Borderable, Colorable, Labelable, Positionable, Widget};
use learning_conrod_core::gui::{cache_queued_glyphs, create_ui};
use piston::window::Window;

pub enum App {
    Game(learning_conrod_game::App),
    Editor(()),
    Selection(SelectionGUI),
}

impl App {
    fn perform_action(&mut self, action: AppResult, window: &mut PistonWindow) {
        match action {
            AppResult::KeepCurrent => {}
            AppResult::SwitchToGame => {
                //TODO handle create_app Err better
                *self = Game(create_app(window).expect("TODO handle this better"))
            }
            AppResult::SwitchToEditor => *self = Editor(unimplemented!()),
            AppResult::SwitchToSelection => {
                //TODO handle create_gui Err better
                *self = Selection(create_gui(window).expect("TODO handle this better"))
            }
            AppResult::Exit => {
                window.set_should_close(true);
            }
        }
    }
}

impl Application for App {
    type RR = ();
    type IR = ();
    type UR = ();

    fn render<'font>(
        &self,
        render_context: &mut RenderContext<'font, GlGraphics>,
        render_args: &RenderArgs,
    ) -> Self::RR {
        match self {
            Self::Game(app) => {
                app.render(render_context, render_args);
            }
            Self::Editor(_app) => unimplemented!(),
            Self::Selection(app) => {
                app.render(render_context, render_args);
            }
        };
    }

    fn input(
        &mut self,
        event: Input,
        event_loop: &mut Events,
        window: &mut PistonWindow,
    ) -> Self::IR {
        match self {
            Self::Game(app) => {
                app.input(event, event_loop, window);
            }
            Self::Editor(_app) => unimplemented!(),
            Self::Selection(app) => {
                let result = app.input(event, event_loop, window);
                self.perform_action(result, window);
            }
        };
    }

    fn update(&mut self, update_args: UpdateArgs, window: &mut PistonWindow) -> Self::UR {
        match self {
            Self::Game(app) => {
                match app.update(update_args, window) {
                    learning_conrod_game::UpdateAction::Nothing => {}
                    //TODO handle create_app Err better
                    learning_conrod_game::UpdateAction::Close => {
                        *self = Selection(create_gui(window).unwrap())
                    }
                }
            }
            Self::Editor(_app) => unimplemented!(),
            Self::Selection(app) => {
                let result = app.update(update_args, window);
                self.perform_action(result, window);
            }
        }
    }
}

widget_ids! {
    pub struct Ids {
        main_canvas,
        menu_title,
        editor_button,
        game_button,
        quit_button,
    }
}

pub struct SelectionGUI {
    gui: GUI<Ids>,
}

pub enum AppResult {
    KeepCurrent,
    SwitchToGame,
    SwitchToEditor,
    SwitchToSelection,
    Exit,
}

pub fn create_gui(window: &PistonWindow) -> Result<SelectionGUI, String> {
    let mut ui = create_ui(window);

    // Create our `conrod_core::image::Map` which describes each of our widget->image mappings.
    // In our case we have no image, however the macro may be used to list multiple.
    let image_map: Map<opengl_graphics::Texture> = conrod_core::image::Map::new();

    // Instantiate the generated list of widget identifiers.
    let generator = ui.widget_id_generator();
    let ids = Ids::new(generator);

    Ok(SelectionGUI {
        gui: GUI {
            ui,
            ids,
            image_ids: vec![],
            image_map,
            fullscreen: false,
        },
    })
}

impl Application for SelectionGUI {
    type RR = ();
    type IR = AppResult;
    type UR = AppResult;

    fn render(
        &self,
        render_context: &mut RenderContext<GlGraphics>,
        _render_args: &RenderArgs,
    ) -> Self::RR {
        //TODO menu to select between game and editor should be presented here!

        let RenderContext {
            gl,
            glyph_cache,
            text_texture_cache,
            text_vertex_data,
            ..
        } = render_context;

        let cache_queued_glyphs = cache_queued_glyphs(text_vertex_data);

        gl.draw(_render_args.viewport(), |c, gl| {
            self.gui
                .draw(text_texture_cache, glyph_cache, cache_queued_glyphs, c, gl);
        });
    }

    fn input(
        &mut self,
        event: Input,
        _event_loop: &mut Events,
        window: &mut PistonWindow,
    ) -> Self::IR {
        if let Some(cr_event) = conrod_piston::event::convert(
            Event::Input(event.clone()),
            self.gui.ui.win_w,
            self.gui.ui.win_h,
        ) {
            self.gui.ui.handle_event(cr_event);
        }

        match &event {
            Input::Button(ButtonArgs {
                button: Button::Keyboard(Key::G),
                ..
            }) => Self::IR::SwitchToGame,
            Input::Button(ButtonArgs {
                button: Button::Keyboard(Key::F11),
                ..
            }) => {
                let monitor = window.window.window.get_current_monitor();

                window.window.window.set_fullscreen(Some(monitor));

                Self::IR::KeepCurrent
            }
            Input::Button(ButtonArgs {
                button: Button::Keyboard(Key::E),
                ..
            }) => Self::IR::SwitchToEditor,
            Input::Button(ButtonArgs {
                button: Button::Keyboard(Key::Escape),
                ..
            }) => Self::IR::Exit,
            _ => Self::IR::KeepCurrent,
        }
    }

    fn update(&mut self, _update_args: UpdateArgs, _window: &mut PistonWindow) -> Self::UR {
        let ui = &mut self.gui.ui.set_widgets();

        widget::canvas::Canvas::new()
            .border_rgba(0.0, 0.0, 0.0, 0.0)
            .rgba(1.0, 1.0, 1.0, 1.0)
            .set(self.gui.ids.main_canvas, ui);

        widget::Text::new("Main Menu")
            .font_size(30)
            .mid_top_of(self.gui.ids.main_canvas)
            .set(self.gui.ids.menu_title, ui);

        let game = widget::Button::new()
            .label("Game")
            .down_from(self.gui.ids.menu_title, 10.0)
            .set(self.gui.ids.game_button, ui);

        let editor = widget::Button::new()
            .label("Editor")
            .down_from(self.gui.ids.game_button, 10.0)
            .enabled(false)
            .set(self.gui.ids.editor_button, ui);
        let quit = widget::Button::new()
            .label("Quit")
            .down_from(self.gui.ids.editor_button, 10.0)
            .set(self.gui.ids.quit_button, ui);

        if quit.was_clicked() {
            AppResult::Exit
        } else if game.was_clicked() {
            AppResult::SwitchToGame
        } else if editor.was_clicked() {
            AppResult::SwitchToEditor
        } else {
            AppResult::KeepCurrent
        }
    }
}
