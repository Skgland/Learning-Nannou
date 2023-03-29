//
//Initial Setting
//

// Change this to OpenGL::V2_1 if not working.

use learning_conrod_core::error::MainError;
use learning_conrod_core::gui::{Application};
use nannou::prelude::*;


const INIT_WIDTH: u32 = 200;
const INIT_HEIGHT: u32 = 200;

mod gui;

use gui::{AppState, create_gui, AppSelection};


impl AppState{
    fn setup(app: &App) -> Self {
        app.set_fullscreen_on_shortcut(true);

        let window_id = app.new_window()
            .view(|app, model: &Self, frame| model.view(app, &frame))
            .raw_event(|app, model: &mut Self, event| model.raw_window_event(app, event))
            .build().unwrap();

        AppState {
            main_window: window_id,
            selection: AppSelection::Selection(create_gui(app, window_id).unwrap()),
        }
    }
}

fn main() -> Result<(), MainError> {
    env_logger::Builder::default()
        .filter_level(log::LevelFilter::Warn)
        .filter_module("learning_conrod", log::LevelFilter::Trace)
        .parse_default_env()
        .init();


    nannou::app(AppState::setup).update(|app, model, update| model.update(app, update)).run();

    Ok(())
}
