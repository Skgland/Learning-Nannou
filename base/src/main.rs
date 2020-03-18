//
//Initial Setting
//

// Change this to OpenGL::V2_1 if not working.

use conrod_core::{image::Map, text::rt::gpu_cache::Cache, widget_ids};
use learning_conrod_core::error::MainError;
use learning_conrod_core::{Application, RenderContext, GUI};
use opengl_graphics::{GlGraphics, OpenGL, Texture};
use piston::event_loop::{EventSettings, Events};
use piston::input::{
    Button, ButtonArgs, Event, Input, Key, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};
use piston::window::WindowSettings;
use piston_window::{PistonWindow, TextureSettings};

use crate::App::{Editor, Game, Selection};
use env_logger::Env;
use learning_conrod_game::create_app;
use log::{info, trace};

const OPEN_GL_VERSION: OpenGL = OpenGL::V3_2;
const INIT_WIDTH: u32 = 200;
const INIT_HEIGHT: u32 = 200;

mod gui;

use gui::{create_gui, App};

fn main() -> Result<(), MainError> {
    env_logger::from_env(Env::default().default_filter_or("warn,learning_conrod=trace")).init();

    let mut window = create_window();

    trace!("Creating render Context!");
    let mut context = create_render_context();

    trace!("Creating event loop iterator");
    let mut event_loop = Events::new(EventSettings::new());

    trace!("Construction base gui!");

    let mut app = App::Selection(create_gui(&window)?);

    info!("Press G to start game, E to start editor or ESC to exit!");

    while let Some(e) = event_loop.next(&mut window) {
        e.render(|r| app.render(&mut context, r));
        e.update(|u| app.update(*u, &mut window));
        if let Event::Input(i) = e {
            app.input(i.clone(), &mut event_loop, &mut window);
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
        let texture =
            opengl_graphics::Texture::from_memory_alpha(&init, INIT_WIDTH, INIT_HEIGHT, &settings)
                .unwrap();
        (cache, texture)
    };
    TextCache {
        text_vertex_data,
        glyph_cache,
        text_texture_cache,
    }
}

fn create_window() -> PistonWindow {
    // Create an Glutin window.
    WindowSettings::new("Learning Conrod", [INIT_WIDTH, INIT_HEIGHT])
        .opengl(OPEN_GL_VERSION)
        .vsync(true)
        .fullscreen(false)
        .build()
        .unwrap()
}

fn create_render_context<'font>() -> RenderContext<'font, opengl_graphics::GlGraphics> {
    let TextCache {
        text_vertex_data,
        glyph_cache,
        text_texture_cache,
    } = create_text_cache(&());
    let gl = GlGraphics::new(OPEN_GL_VERSION);
    RenderContext {
        text_texture_cache,
        glyph_cache,
        text_vertex_data,
        gl,
    }
}
