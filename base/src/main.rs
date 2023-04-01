//
//Initial Setting
//

// Change this to OpenGL::V2_1 if not working.

use learning_conrod_core::error::MainError;
use nannou::prelude::*;

mod gui;

use gui::{AppState};
use nannou_egui::Egui;


impl AppState{
    fn setup(app: &App) -> Self {
        app.set_fullscreen_on_shortcut(true);
        app.set_exit_on_escape(false);

        let window_id = app.new_window()
            .view(|app, model: &Self, frame| model.view(app, &frame))
            .raw_event(|app, model: &mut Self, event| model.raw_window_event(app, event))
            .build().unwrap();

        let egui = Egui::from_window(&app.window(window_id).unwrap());

        AppState {
            main_window: window_id,
            selection: None,
            egui
        }
    }
}

fn main() -> Result<(), MainError> {
    env_logger::Builder::default()
        .filter_level(log::LevelFilter::Warn)
        .filter_module("learning_conrod", log::LevelFilter::Debug)
        .filter_module("learning_conrod_core", log::LevelFilter::Debug)
        .filter_module("learning_conrod_base", log::LevelFilter::Debug)
        .filter_module("learning_conrod_game", log::LevelFilter::Debug)
        .filter_module("learning_conrod_editor", log::LevelFilter::Debug)
        .parse_default_env()
        .init();


    nannou::app(AppState::setup).update(|app, model, update| model.update(app, update)).run();

    Ok(())
}
