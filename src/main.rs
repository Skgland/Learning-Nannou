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
use conrod_core::widget_ids;
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

extern crate find_folder;

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        canvas,
        title,
    }
}

//
//Initial Setting
//

// Change this to OpenGL::V2_1 if not working.

const OPEN_GL_VERSION: OpenGL = OpenGL::V3_2;
const INIT_WIDTH: u32 = 200;
const INIT_HEIGHT: u32 = 200;

fn main() {
    let mut window = createWindow();

    let mut ui = createUi();

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();


    let TextCache { text_vertex_data, glyph_cache, text_texture_cache } = createTextCache(&());


    // Create our `conrod_core::image::Map` which describes each of our widget->image mappings.
    // In our case we have no image, however the macro may be used to list multiple.
    let image_map: Map<opengl_graphics::Texture> = conrod_core::image::Map::new();
    // let rust_logo = image_map.insert(rust_logo);

    // Instantiate the generated list of widget identifiers.
    let ids = Ids::new(ui.widget_id_generator());

    // Create a new game and run it.
    let mut app = App::new(
        GlGraphics::new(OPEN_GL_VERSION),
        GUI {
            ui,
            text_vertex_data,
            ids,
            glyph_cache,
            image_map,
            text_texture_cache,
            visibility: GUIVisibility::HUD,
        },
        GameState::new(),
    );

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        e.render(|r| app.render(r));

        if let Event::Input(i) = e {
            app.input(i,&mut window);
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

fn createTextCache<'font>(_: &()) -> TextCache {
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

fn createWindow() -> PistonWindow<glutin_window::GlutinWindow> {
    // Create an Glutin window.
    WindowSettings::new(
        "spinning-square",
        [INIT_WIDTH, INIT_HEIGHT],
    ).opengl(OPEN_GL_VERSION)
     .vsync(true)
     .build()
     .unwrap()
}

fn createUi() -> Ui {

    //construct Ui
    conrod_core::UiBuilder::new([INIT_WIDTH as f64, INIT_HEIGHT as f64])
        .build()
}
