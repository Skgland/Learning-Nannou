use conrod_core::image::Map;

mod app;

pub mod game;
pub mod gui;

use gui::*;

use game::TileTextureIndex;

use piston_window::{OpenGL, PistonWindow};

use log::{error, trace};

pub use app::{GameApp, UpdateAction};
use learning_conrod_core::{get_asset_path, gui::create_ui, gui::load_textures, gui::GUI};

//
//Initial Setting

// Change this to OpenGL::V2_1 if not working.

#[allow(dead_code)]
const OPEN_GL_VERSION: OpenGL = OpenGL::V3_2;

fn startup() {
    let path = get_asset_path().join("levels").join("test.level.ron");

    trace!("Writing test level to disc!");
    if let Err(e) = game::level::saving::save_level(&path, &game::test_level::test_level()) {
        error!("{}", e);
    }
}

pub fn create_game_app(window: &PistonWindow) -> Result<(GameApp, GUI<GameIds>), String> {
    startup();

    let mut ui = create_ui(window);

    // Load the rust logo from file to a piston_window texture.
    //let test_texture = load_texture("test.png");

    // Create our `conrod_core::image::Map` which describes each of our widget->image mappings.
    // In our case we have no image, however the macro may be used to list multiple.
    let image_map: Map<opengl_graphics::Texture> = conrod_core::image::Map::new();
    //let test_texture = image_map.insert(test_texture);

    let texture_map = load_textures::<TileTextureIndex>();

    // Instantiate the generated list of widget identifiers.
    let generator = ui.widget_id_generator();
    let ids = GameIds::new(generator);

    let init_menu = MenuState::open_level_selection();

    Ok((
        GameApp::new(texture_map, init_menu),
        GUI {
            ui,
            ids,
            image_ids: vec![],
            image_map,
            fullscreen: false,
        },
    ))
}
