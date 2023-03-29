
use learning_conrod_core::gui::Application;
use nannou::prelude::*;
use nannou_egui::{Egui, egui::{self, Key}};
//use piston::window::Window;
use crate::gui::AppResult::{KeepCurrent, SwitchToSelection};
use learning_conrod_game::{create_game_app, GameApp};
use learning_conrod_editor::{create_editor_app, EditorApp};
use log::info;



pub struct AppState {
    pub(crate) main_window: WindowId,
    pub(crate) selection: AppSelection,
}

pub(crate) enum AppSelection {
    Game(GameApp),
    Editor(EditorApp),
    Selection(SelectionGUI),
}

impl AppState {
    fn perform_action(&mut self, app: &App, action: AppResult) {

        match action {
            AppResult::KeepCurrent => {}
            AppResult::SwitchToGame => {
                //TODO handle create_app Err better
                let app = create_game_app(app, self.main_window).expect("TODO handle this better");
                self.selection = AppSelection::Game(app)
            }
            AppResult::SwitchToEditor => {
                //TODO handle create_app Err better
                let app = create_editor_app(app, self.main_window).expect("TODO handle this better");
                self.selection = AppSelection::Editor(app)
            }
            AppResult::SwitchToSelection => {
                //TODO handle create_gui Err better
                let app = create_gui(app, self.main_window).expect("TODO handle this better");
                self.selection = AppSelection::Selection(app);
            }
            AppResult::Exit => {
                // FIXME how do I end the program gracefully?
            }
        }
    }

    pub fn raw_window_event(&mut self, app: &App, event: &nannou::winit::event::WindowEvent) {

        use AppSelection::*;

        match &mut self.selection {
            Game(game) => {
                game.raw_window_event( app, event);
            }
            Editor(editor) => editor.raw_window_event(app, event),
            Selection(selection) => {
                selection.raw_window_event(app, event);
            }
        };

    }

    pub fn update(&mut self, app: &App, update: Update) {
        use AppSelection::*;
        let action = match &mut self.selection {
            Game(game_app) => match game_app.update(app,update, self.main_window) {
                learning_conrod_game::UpdateAction::Nothing => KeepCurrent,
                learning_conrod_game::UpdateAction::Close => SwitchToSelection,
            },
            Editor(editor_app) => {
                editor_app.update(app, update, self.main_window);
                KeepCurrent
            }
            Selection(selection_app) => {
                selection_app.update(app, update, self.main_window)
            }
        };
        self.perform_action(app, action);
    }

    pub(crate) fn view(&self, app: &App, frame: &nannou::Frame)  {
        use AppSelection::*;
        match &self.selection {
            Game(game_app   ) => game_app.view(app, frame),
            Editor(editor_app) => editor_app.view(app, frame),
            Selection(selection_app) => selection_app.view(app, frame),
        }
    }
}


pub struct SelectionGUI {
    egui: Egui
}

pub enum AppResult {
    KeepCurrent,
    SwitchToGame,
    SwitchToEditor,
    SwitchToSelection,
    Exit,
}

pub fn create_gui(app: &App, main_window: WindowId) -> Result<SelectionGUI, String> {


    let window = app.window(main_window).unwrap();
    let egui = Egui::from_window(&window);

    Ok(SelectionGUI {egui})
}

impl Application<'_> for SelectionGUI {
    type ViewResult = ();
    type RawEventResult = ();
    type UpdateResult = AppResult;

    fn view(
            &self,
            app: &App,
            frame: &Frame,
        ) -> Self::ViewResult {

        //TODO menu to select between game and editor should be presented here!
        self.egui.draw_to_frame(frame);
    }

    fn raw_window_event(&mut self, app: &App, event: &nannou::winit::event::WindowEvent) -> Self::RawEventResult {
        self.egui.handle_raw_event(event);
    }

    fn update(
        &mut self,
        app:&App,
        update: Update,
        main_window: WindowId
    ) -> Self::UpdateResult {

        let ctx = self.egui.begin_frame();

        if ctx.input().key_pressed(Key::G) {
            return AppResult::SwitchToGame
        } else if ctx.input().key_pressed(Key::E) {
            return AppResult::SwitchToEditor
        } else if ctx.input().key_pressed(Key::Escape) {
            // TODO Quit
        }

        egui::Window::new("Learning Conrod").show(&ctx, |ui| {
             /*
            widget::canvas::Canvas::new()
                .border_rgba(0.0, 0.0, 0.0, 0.0)
                .rgba(1.0, 1.0, 1.0, 1.0)
                .set(gui.ids.main_canvas, ui);*/

            ui.label("Main Menu");
            if ui.button("Game").clicked() {
                AppResult::SwitchToGame
            } else  if ui.button("Editor").clicked() {
                AppResult::SwitchToEditor
            } else if ui.button("Quit").clicked() {
                AppResult::Exit
            } else {
                AppResult::KeepCurrent}

        }).and_then(|inner| inner.inner).unwrap_or(AppResult::KeepCurrent)


    }
}
