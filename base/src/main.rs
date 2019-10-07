//
//Initial Setting
//

// Change this to OpenGL::V2_1 if not working.

use std::path::PathBuf;
use conrod_core::{
    Ui,
    image::Map,
    text::rt::gpu_cache::Cache,
    widget_ids
};
use learning_conrod_base::{GUI, RenderContext, Application};
use opengl_graphics::{GlGraphics, Texture, OpenGL};
use piston::window::{WindowSettings, Window};
use piston::input::{Event, RenderEvent, UpdateEvent, RenderArgs, UpdateArgs, Input, ButtonArgs, Key, Button};
use piston::event_loop::{Events, EventSettings};
use glutin_window::GlutinWindow;
use learning_conrod_base::error::MainError;
use piston_window::{PistonWindow, TextureSettings};

const OPEN_GL_VERSION: OpenGL = OpenGL::V3_2;
const INIT_WIDTH: u32 = 200;
const INIT_HEIGHT: u32 = 200;

fn main() -> Result<(), MainError> {
    let mut window = create_window();

    let ui = create_ui();

    println!("Construction base gui!");

    //Create selection menu editor vs. game
    let mut gui = create_gui(ui)?;


    println!("Creating render Context!");
    let mut context = create_render_context();


    println!("Creating event loop iterator");
    let mut event_loop = Events::new(EventSettings::new());

    while let Some(e) = event_loop.next(&mut window) {
        e.render(|r| gui.render(&mut context, r));
        e.update(|u| gui.update(u, &mut window));
        if let Event::Input(i) = e {
            gui.input(i.clone(), &mut event_loop, &mut window);
            match &i {
                Input::Button(ButtonArgs { button: Button::Keyboard(Key::G), .. }) => {
                    learning_conrod_game::run(&mut window, &mut context,&mut event_loop);
                }
                Input::Button(ButtonArgs { button: Button::Keyboard(Key::Escape), .. }) => {
                    window.window.set_should_close(true);
                }
                _ => {}
            }
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
        "Learning Conrod",
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

widget_ids! {
    pub struct Ids {
        main_canvas,
        menu_title,
        editor_button,
        game_button,
        quit_button,
    }
}

struct BaseGUI{
    gui: GUI<Ids>
}

impl Application for BaseGUI{

    type RR = ();
    type IR = ();
    type UR = ();

    fn render(&self, render_context: &mut RenderContext<GlGraphics>, render_args: &RenderArgs) -> Self::RR {
    }

    fn input(&mut self, event: Input, event_loop: &mut Events, window: &mut PistonWindow<GlutinWindow>) -> Self::IR {

    }

    fn update(&mut self, update_args: &UpdateArgs, window: &mut PistonWindow<GlutinWindow>) -> Self::UR {
    }
}

fn create_gui(mut ui: Ui) -> Result<BaseGUI, String> {

    // Create our `conrod_core::image::Map` which describes each of our widget->image mappings.
    // In our case we have no image, however the macro may be used to list multiple.
    let image_map: Map<opengl_graphics::Texture> = conrod_core::image::Map::new();

    // Instantiate the generated list of widget identifiers.
    let generator = ui.widget_id_generator();
    let ids = Ids::new(generator);


    Ok(BaseGUI{
        gui: GUI {
            ui,
            ids,
            image_ids: vec![],
            image_map,
            fullscreen: false,
        }
    })
}

fn create_render_context<'font>() -> RenderContext<'font, opengl_graphics::GlGraphics> {
    let TextCache { text_vertex_data, glyph_cache, text_texture_cache } = create_text_cache(&());
    let gl = GlGraphics::new(OPEN_GL_VERSION);
    RenderContext {
        text_texture_cache,
        glyph_cache,
        text_vertex_data,
        gl,
    }
}
