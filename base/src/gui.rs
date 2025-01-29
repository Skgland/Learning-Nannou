use learning_conrod_core::gui::Application;
use nannou::prelude::*;
use nannou_egui::{
    egui::{self, Key},
    Egui,
};
//use piston::window::Window;
use crate::gui::AppResult::{KeepCurrent, SwitchToSelection};
use learning_conrod_editor::{create_editor_app, EditorApp};
use learning_conrod_game::{create_game_app, GameApp};

pub struct AppState {
    pub(crate) main_window: WindowId,
    pub(crate) egui: Egui,
    pub(crate) selection: Option<AppSelection>,
}

pub(crate) enum AppSelection {
    Game(GameApp),
    Editor(EditorApp),
}

impl AppState {
    fn perform_action(
        &mut self,
        app: &App,
        action: AppResult,
        _update: Update,
        _main_window: WindowId,
    ) {
        match action {
            AppResult::KeepCurrent => {}
            AppResult::SwitchToGame => {
                //TODO handle create_app Err better
                let game = create_game_app(app).expect("TODO handle this better");
                self.selection = Some(AppSelection::Game(game));
            }
            AppResult::SwitchToEditor => {
                //TODO handle create_app Err better
                let editor = create_editor_app(app).expect("TODO handle this better");
                self.selection = Some(AppSelection::Editor(editor))
            }
            AppResult::SwitchToSelection => {
                self.selection = None;
            }
            AppResult::Exit => app.quit(),
        }
    }

    pub fn raw_window_event(&mut self, _app: &App, event: &nannou::winit::event::WindowEvent) {
        self.egui.handle_raw_event(event);
    }

    pub fn update(&mut self, app: &App, update: Update) {
        use AppSelection::*;
        let action = match &mut self.selection {
            Some(Game(game_app)) => {
                match game_app.update(app, update, &mut self.egui, self.main_window) {
                    learning_conrod_game::UpdateAction::Nothing => KeepCurrent,
                    learning_conrod_game::UpdateAction::Close => SwitchToSelection,
                }
            }
            Some(Editor(editor_app)) => {
                editor_app.update(app, update, &mut self.egui, self.main_window);
                KeepCurrent
            }
            None => {
                let ctx = self.egui.begin_frame();

                if ctx.input(|state| state.key_pressed(Key::G)) {
                    AppResult::SwitchToGame
                } else if ctx.input(|state| state.key_pressed(Key::E)) {
                    AppResult::SwitchToEditor
                } else if ctx.input(|state| state.key_pressed(Key::Escape)) {
                    AppResult::Exit
                } else {
                    egui::Window::new("Learning Conrod")
                        .show(&ctx, |ui| {
                            /*
                            widget::canvas::Canvas::new()
                                .border_rgba(0.0, 0.0, 0.0, 0.0)
                                .rgba(1.0, 1.0, 1.0, 1.0)
                                .set(gui.ids.main_canvas, ui);*/

                            ui.label("Main Menu");
                            if ui.button("Game").clicked() {
                                AppResult::SwitchToGame
                            } else if ui.button("Editor").clicked() {
                                AppResult::SwitchToEditor
                            } else if ui.button("Quit").clicked() {
                                AppResult::Exit
                            } else {
                                AppResult::KeepCurrent
                            }
                        })
                        .and_then(|inner| inner.inner)
                        .unwrap_or(AppResult::KeepCurrent)
                }
            }
        };
        self.perform_action(app, action, update, self.main_window);
    }

    pub(crate) fn view(&self, app: &App, frame: &nannou::Frame) {
        use AppSelection::*;
        match &self.selection {
            Some(Game(game_app)) => game_app.view(app, frame, &self.egui),
            Some(Editor(editor_app)) => editor_app.view(app, frame, &self.egui),
            None => {
                self.egui.draw_to_frame(frame).unwrap();
            }
        }
    }
}

pub enum AppResult {
    KeepCurrent,
    SwitchToGame,
    SwitchToEditor,
    SwitchToSelection,
    Exit,
}
