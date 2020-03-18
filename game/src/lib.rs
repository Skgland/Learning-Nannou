use opengl_graphics::Texture;

use conrod_core::image::Map;

mod app;

pub mod game;
pub mod gui;

use gui::*;

use game::TileTextureIndex;
use std::collections::btree_map::BTreeMap;

use graphics::Graphics;

use piston_window::{OpenGL, PistonWindow, TextureSettings};

use log::{error, trace};

pub use app::{App, UpdateAction};
use learning_conrod_core::{create_ui, get_asset_path, GUI};

//
//Initial Setting
//

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

type TextureMap<G> =
    std::collections::btree_map::BTreeMap<TileTextureIndex, <G as Graphics>::Texture>;

fn load_textures(texture_map: &mut TextureMap<opengl_graphics::GlGraphics>) {
    use derive_macros_helpers::Enumerable;

    for tile_index in TileTextureIndex::enumerate_all() {
        let file_name = tile_index.file_name();
        load_texture_into_map(texture_map, tile_index, &file_name);
    }
}

fn load_texture_into_map(
    texture_map: &mut TextureMap<opengl_graphics::GlGraphics>,
    key: TileTextureIndex,
    name: &str,
) {
    let assets = get_asset_path();
    let path = assets.join("textures").join(format!("{}.png", name));
    let settings = TextureSettings::new();
    if let Ok(texture) = Texture::from_path(&path, &settings) {
        texture_map.insert(key, texture);
    } else {
        error!(
            "Failed loading Texture with Index: {:?} , at: {:?}",
            &key, path
        );
    }
}

pub fn create_app(window: &PistonWindow) -> Result<App, String> {
    startup();

    let mut ui = create_ui(window);

    // Load the rust logo from file to a piston_window texture.
    //let test_texture = load_texture("test.png");

    // Create our `conrod_core::image::Map` which describes each of our widget->image mappings.
    // In our case we have no image, however the macro may be used to list multiple.
    let image_map: Map<opengl_graphics::Texture> = conrod_core::image::Map::new();
    //let test_texture = image_map.insert(test_texture);

    let mut texture_map = BTreeMap::new();

    load_textures(&mut texture_map);

    // Instantiate the generated list of widget identifiers.
    let generator = ui.widget_id_generator();
    let ids = Ids::new(generator);

    let init_menu = MenuState::open_level_selection();

    Ok(App::new(
        GUI {
            ui,
            ids,
            image_ids: vec![],
            image_map,
            fullscreen: false,
        },
        texture_map,
        init_menu,
    ))
}
