extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate ron;

use piston::window::*;
use piston::event_loop::*;
use piston::input::*;
use opengl_graphics::Texture;
use conrod_core::Ui;


use conrod_core::image::Map;

mod app;

use app::*;

pub mod game;
pub mod gui;

use gui::*;
use rusttype::gpu_cache::Cache;

use game::TileTextureIndex;
use std::collections::btree_map::BTreeMap;


use std::path::PathBuf;
use graphics::Graphics;

extern crate find_folder;

use piston_window::{PistonWindow, TextureSettings, OpenGL};
use glutin_window::GlutinWindow;
use learning_conrod_base::error::MainError;
use learning_conrod_base::RenderContext;
use learning_conrod_base::Application;


//
//Initial Setting
//

// Change this to OpenGL::V2_1 if not working.

const OPEN_GL_VERSION: OpenGL = OpenGL::V3_2;
const INIT_WIDTH: u32 = 200;
const INIT_HEIGHT: u32 = 200;

pub fn  startup() {
    println!("Writing test level to disc!");
    if let Err(e) = game::level::saving::save_level(get_asset_path().join("levels").join("test.level.ron").as_path(), &game::test_level::test_level()) {
        eprintln!("{}", e);
    }
}

pub fn  run(window: &mut PistonWindow<GlutinWindow>, context: &mut RenderContext<opengl_graphics::GlGraphics>,event_loop: &mut Events) -> Result<(), MainError> {

    println!("Construction game app!");
    // Create a new game and run it.
    let mut app = create_app()?;

    while let Some(e) = event_loop.next(window) {
        e.render(|r| app.render(context, r));
        if let Some(UpdateAction::Close) = e.update(|u| app.update(u, window)){
            break;
        }

        if let Event::Input(i) = e {
            app.input(i, event_loop ,window)
        }
    }

    Ok(())
}

struct TextCache<'font> {
    text_vertex_data: Vec<u8>,
    glyph_cache: Cache<'font>,
    text_texture_cache: Texture,
}

fn create_text_cache(_: &()) -> TextCache {
    // Create a texture to use for efficiently caching text on the GPU.
    let text_vertex_data: Vec<u8> = Vec::new();
    let (glyph_cache, text_texture_cache) = {
        const SCALE_TOLERANCE: f32 = 0.1;
        const POSITION_TOLERANCE: f32 = 0.1;
        let cache = conrod_core::text::GlyphCache::builder()
            .dimensions(INIT_WIDTH, INIT_HEIGHT)
            .scale_tolerance(SCALE_TOLERANCE)
            .position_tolerance(POSITION_TOLERANCE)
            .build();
        let buffer_len = INIT_WIDTH as usize * INIT_HEIGHT as usize;
        let init = vec![128; buffer_len];
        let settings = TextureSettings::new();
        let texture = opengl_graphics::Texture::from_memory_alpha(&init, INIT_WIDTH, INIT_HEIGHT, &settings).unwrap();
        (cache, texture)
    };
    TextCache { text_vertex_data, glyph_cache, text_texture_cache }
}

fn create_window() -> PistonWindow<GlutinWindow> {
    // Create an Glutin window.
    WindowSettings::new(
        "spinning-square",
        [INIT_WIDTH, INIT_HEIGHT],
    ).opengl(OPEN_GL_VERSION)
     .vsync(true)
     .fullscreen(false)
     .build()
     .unwrap()
}

fn get_asset_path() -> PathBuf {
    find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap()
}

fn create_ui() -> Ui {

    //construct Ui
    let mut ui = conrod_core::UiBuilder::new([f64::from(INIT_WIDTH), f64::from(INIT_HEIGHT)])
        .build();


    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = get_asset_path();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();
    ui
}

type TextureMap<G> = std::collections::btree_map::BTreeMap<TileTextureIndex, <G as Graphics>::Texture>;

fn load_textures(texture_map: &mut TextureMap<opengl_graphics::GlGraphics>) {
    use derive_macros_helpers::Enumerable;

    for tile_index in TileTextureIndex::enumerate_all() {
        let file_name = tile_index.file_name();
        load_texture_into_map(texture_map, tile_index, &file_name);
    }
}

fn load_texture_into_map(texture_map: &mut TextureMap<opengl_graphics::GlGraphics>, key: TileTextureIndex, name: &str) {
    let assets = get_asset_path();
    let path = assets.join("textures").join(format!("{}.png", name));
    let settings = TextureSettings::new();
    if let Ok(texture) = Texture::from_path(&path, &settings) {
        texture_map.insert(key, texture);
    } else {
        eprintln!("Failed loading Texture with Index: {:?} , at: {}.png", &key, name);
    }
}

fn create_app() -> Result<App, String> {

    let mut ui = create_ui();

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

    let mut init_menu = MenuState::MainMenu;

    init_menu.open_level_selection();

    Ok(App::new(
        GUI {
            ui,
            ids,
            image_ids: vec![],
            image_map,
            fullscreen: false,
        }, texture_map, init_menu,
    ))
}
