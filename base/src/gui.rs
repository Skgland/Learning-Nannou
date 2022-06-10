use learning_conrod_core::gui::{cache_queued_glyphs, create_ui, Application, RenderContext, GUI};
//use piston::window::Window;
use crate::gui::App::{Editor, Game, Selection};
use crate::gui::AppResult::{KeepCurrent, SwitchToSelection};
use conrod_core::{
    image::Map, input::Button, input::Key, widget, widget_ids, Borderable, Colorable, Labelable,
    Positionable, Widget,
};
use learning_conrod_game::{create_game_app, gui::GameIds, GameApp};
use opengl_graphics::GlGraphics;

use learning_conrod_editor::{create_editor_app, EditorApp, EditorIds};
use log::info;
use piston_window::{
    ButtonArgs, ButtonState, Context, Event, Events, Input, PistonWindow, RenderArgs, UpdateArgs,
    Window,
};

pub enum App {
    Game(GameApp, GUI<GameIds>),
    Editor(EditorApp, GUI<EditorIds>),
    Selection(SelectionGUI, GUI<Ids>),
}

impl App {
    fn perform_action(&mut self, action: AppResult, window: &mut PistonWindow) {
        let fullscreen = match self {
            Game(_, gui) => gui.fullscreen,
            Editor(_, gui) => gui.fullscreen,
            Selection(_, gui) => gui.fullscreen,
        };

        match action {
            AppResult::KeepCurrent => {}
            AppResult::SwitchToGame => {
                //TODO handle create_app Err better
                let (app, mut gui) = create_game_app(window).expect("TODO handle this better");
                gui.set_fullscreen(window, fullscreen);
                *self = Game(app, gui)
            }
            AppResult::SwitchToEditor => {
                //TODO handle create_app Err better
                let (app, mut gui) = create_editor_app(window).expect("TODO handle this better");
                gui.set_fullscreen(window, fullscreen);
                *self = Editor(app, gui)
            }
            AppResult::SwitchToSelection => {
                //TODO handle create_gui Err better
                let (app, mut gui) = create_gui(window).expect("TODO handle this better");
                gui.fullscreen = fullscreen;
                *self = Selection(app, gui)
            }
            AppResult::Exit => {
                window.set_should_close(true);
            }
        }
    }
}

impl Application<'_> for App {
    type RR = ();
    type IR = ();
    type UR = ();
    type GUI = ();
    type RP = ();
    type UP = ();

    fn render<'font>(
        &self,
        _gui: &Self::GUI,
        _rp: &Self::RP,
        gl: &mut GlGraphics,
        context: Context,
        render_context: &mut RenderContext<'font>,
        render_args: &RenderArgs,
    ) -> Self::RR {
        match self {
            Self::Game(game_app, gui) => {
                game_app.render(gui, &(), gl, context, render_context, render_args);
            }
            Self::Editor(editor_app, gui) => {
                editor_app.render(gui, &(), gl, context, render_context, render_args)
            }
            Self::Selection(selection_app, gui) => {
                selection_app.render(gui, &(), gl, context, render_context, render_args);
            }
        };
    }

    fn input(
        &mut self,
        _gui: &mut Self::GUI,
        event: Input,
        event_loop: &mut Events,
        window: &mut PistonWindow,
    ) -> Self::IR {
        match self {
            Self::Game(app, gui) => {
                app.input(gui, event, event_loop, window);
            }
            Self::Editor(app, gui) => app.input(gui, event, event_loop, window),
            Self::Selection(app, gui) => {
                let result = app.input(gui, event, event_loop, window);
                self.perform_action(result, window);
            }
        };
    }

    fn update(
        &mut self,
        _gui: &mut Self::GUI,
        _up: &mut Self::UP,
        update_args: UpdateArgs,
        window: &mut PistonWindow,
    ) -> Self::UR {
        let action = match self {
            Self::Game(game_app, gui) => match game_app.update(gui, &mut (), update_args, window) {
                learning_conrod_game::UpdateAction::Nothing => KeepCurrent,
                learning_conrod_game::UpdateAction::Close => SwitchToSelection,
            },
            Self::Editor(editor_app, gui) => {
                editor_app.update(gui, &mut (), update_args, window);
                KeepCurrent
            }
            Self::Selection(selection_app, gui) => {
                selection_app.update(gui, &mut (), update_args, window)
            }
        };
        self.perform_action(action, window);
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

pub struct SelectionGUI {}

pub enum AppResult {
    KeepCurrent,
    SwitchToGame,
    SwitchToEditor,
    SwitchToSelection,
    Exit,
}

pub fn create_gui(window: &PistonWindow) -> Result<(SelectionGUI, GUI<Ids>), String> {
    let mut ui = create_ui(window);

    // Create our `conrod_core::image::Map` which describes each of our widget->image mappings.
    // In our case we have no image, however the macro may be used to list multiple.
    let image_map: Map<opengl_graphics::Texture> = conrod_core::image::Map::new();

    // Instantiate the generated list of widget identifiers.
    let generator = ui.widget_id_generator();
    let ids = Ids::new(generator);

    Ok((
        SelectionGUI {},
        GUI {
            ui,
            ids,
            image_ids: vec![],
            image_map,
            fullscreen: false,
        },
    ))
}

impl Application<'_> for SelectionGUI {
    type RR = ();
    type IR = AppResult;
    type UR = AppResult;
    type GUI = GUI<Ids>;
    type RP = ();
    type UP = ();

    fn render(
        &self,
        gui: &Self::GUI,
        _rp: &Self::RP,
        gl: &mut GlGraphics,
        context: Context,
        render_context: &mut RenderContext,
        _render_args: &RenderArgs,
    ) -> Self::RR {
        //TODO menu to select between game and editor should be presented here!

        let RenderContext {
            glyph_cache,
            text_texture_cache,
            text_vertex_data,
            ..
        } = render_context;

        let cache_queued_glyphs = cache_queued_glyphs(text_vertex_data);

        gui.draw(
            text_texture_cache,
            glyph_cache,
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
        window: &mut PistonWindow,
    ) -> Self::IR {
        if let Some(cr_event) = conrod_piston::event::convert(
            Event::Input(event.clone(), None),
            gui.ui.win_w,
            gui.ui.win_h,
        ) {
            gui.ui.handle_event(cr_event);
        }

        match &event {
            Input::Button(ButtonArgs {
                button: Button::Keyboard(Key::G),
                ..
            }) => Self::IR::SwitchToGame,
            Input::Button(ButtonArgs {
                button: Button::Keyboard(Key::F11),
                state: ButtonState::Release,
                ..
            }) => {
                if gui.fullscreen {
                    info!("F11 pressed: Turning off fullscreen!");
                    window.window.ctx.window().set_fullscreen(None);
                    gui.fullscreen = false;
                } else {
                    info!("F11 pressed: Turning on fullscreen!");
                    window
                        .window
                        .ctx
                        .window()
                        .set_fullscreen(Some(glutin::window::Fullscreen::Borderless(None)));
                    gui.fullscreen = true;
                }

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

    fn update(
        &mut self,
        gui: &mut Self::GUI,
        _up: &mut Self::UP,
        _update_args: UpdateArgs,
        _window: &mut PistonWindow,
    ) -> Self::UR {
        let ui = &mut gui.ui.set_widgets();

        widget::canvas::Canvas::new()
            .border_rgba(0.0, 0.0, 0.0, 0.0)
            .rgba(1.0, 1.0, 1.0, 1.0)
            .set(gui.ids.main_canvas, ui);

        widget::Text::new("Main Menu")
            .font_size(30)
            .mid_top_of(gui.ids.main_canvas)
            .set(gui.ids.menu_title, ui);

        let game = widget::Button::new()
            .label("Game")
            .down_from(gui.ids.menu_title, 10.0)
            .set(gui.ids.game_button, ui);

        let editor = widget::Button::new()
            .label("Editor")
            .down_from(gui.ids.game_button, 10.0)
            .enabled(false)
            .set(gui.ids.editor_button, ui);
        let quit = widget::Button::new()
            .label("Quit")
            .down_from(gui.ids.editor_button, 10.0)
            .set(gui.ids.quit_button, ui);

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
