mod app;

pub mod game;
pub mod gui;

use gui::*;

use game::TileTextureIndex;


use log::{error, trace};

pub use app::{GameApp, UpdateAction};
use learning_conrod_core::{ gui::load_textures};
use nannou::prelude::*;


fn startup(app: &App) {
    let path = app.assets_path().unwrap().join("levels").join("test.level.ron");

    trace!("Writing test level to disc!");
    if let Err(e) = game::level::saving::save_level(&path, &game::test_level::test_level()) {
        error!("{}", e);
    }
}

pub fn create_game_app(app: &App) -> Result<GameApp, String> {
    startup(app);

    let texture_map = load_textures::<TileTextureIndex>(app);

    let init_menu = MenuState::open_level_selection();

    Ok(GameApp::new( texture_map, init_menu))
}
