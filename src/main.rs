extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL};
//use conrod_core::color::Colorable;
use piston_window::TextureSettings;
use piston_window::PistonWindow;
use conrod_core::image::Map;

mod gui;
mod app;
mod game;

use app::*;
use gui::*;
use crate::game::GameState;
use conrod_core::Ui;
use rusttype::gpu_cache::Cache;
use opengl_graphics::Texture;
use glutin_window::GlutinWindow;

extern crate find_folder;

//
//Initial Setting
//

// Change this to OpenGL::V2_1 if not working.

const OPEN_GL_VERSION: OpenGL = OpenGL::V3_2;
const INIT_WIDTH: u32 = 200;
const INIT_HEIGHT: u32 = 200;

fn main() {
    let mut window = create_window();

    let ui = create_ui();


    // Create a new game and run it.
    let mut app = create_app(ui);

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        e.render(|r| app.render(r));

        if let Event::Input(i) = e {
             app.input(i, &mut window);
        } else {
            e.update(|u| app.update(u));
        }
    }
}

struct TextCache<'font> {
    text_vertex_data: Vec<u8>,
    glyph_cache: Cache<'font>,
    text_texture_cache: Texture,
}

fn create_text_cache<'font>(_: &()) -> TextCache {
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

fn create_ui() -> Ui {

    //construct Ui
    let mut ui = conrod_core::UiBuilder::new([INIT_WIDTH as f64, INIT_HEIGHT as f64])
        .build();


    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();
    ui
}

fn create_app<'font>(mut ui: Ui) -> App<'font> {
    let TextCache { text_vertex_data, glyph_cache, text_texture_cache } = create_text_cache(&());


    // Create our `conrod_core::image::Map` which describes each of our widget->image mappings.
    // In our case we have no image, however the macro may be used to list multiple.
    let image_map: Map<opengl_graphics::Texture> = conrod_core::image::Map::new();
    // let rust_logo = image_map.insert(rust_logo);

    // Instantiate the generated list of widget identifiers.
    let ids = Ids::new(ui.widget_id_generator());

    App::new(
        GlGraphics::new(OPEN_GL_VERSION),
        GUI {
            ui,
            text_vertex_data,
            ids,
            glyph_cache,
            image_map,
            text_texture_cache,
            active_menu: GUIVisibility::HUD,
            fullscreen: false,
        },
        GameState::new(),
    )
}
