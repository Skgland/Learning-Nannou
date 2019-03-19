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

extern crate find_folder;

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        canvas,
        title,
    }
}



fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    struct Size { width: u32, height: u32 };
    let init_size = Size { width: 200, height: 200 };

    // Create an Glutin window.
    let mut window: PistonWindow<glutin_window::GlutinWindow> = WindowSettings::new(
        "spinning-square",
        [init_size.width, init_size.height],
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .unwrap();

    //construct Ui
    let mut ui = conrod_core::UiBuilder::new([init_size.width as f64, init_size.height as f64])
        .build();

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();


    // Create a texture to use for efficiently caching text on the GPU.
    let text_vertex_data: Vec<u8> = Vec::new();
    let (glyph_cache, text_texture_cache) = {
        const SCALE_TOLERANCE: f32 = 0.1;
        const POSITION_TOLERANCE: f32 = 0.1;
        let cache = conrod_core::text::GlyphCache::builder()
            .dimensions(init_size.width, init_size.height)
            .scale_tolerance(SCALE_TOLERANCE)
            .position_tolerance(POSITION_TOLERANCE)
            .build();
        let buffer_len = init_size.width as usize * init_size.height as usize;
        let init = vec![128; buffer_len];
        let settings = TextureSettings::new();
        let texture = opengl_graphics::Texture::from_memory_alpha(&init, init_size.width, init_size.height, &settings).unwrap();
        (cache, texture)
    };

    // Create our `conrod_core::image::Map` which describes each of our widget->image mappings.
    // In our case we have no image, however the macro may be used to list multiple.
    let image_map: Map<opengl_graphics::Texture> = conrod_core::image::Map::new();
    // let rust_logo = image_map.insert(rust_logo);

    // Instantiate the generated list of widget identifiers.
    let ids = Ids::new(ui.widget_id_generator());

    // Create a new game and run it.
    let mut app = App::new(
        GlGraphics::new(opengl),
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
            app.input(i);
        } else {
            e.update(|u| app.update(u));

        }
    }
}
