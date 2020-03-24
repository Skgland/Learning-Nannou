use conrod_core::image::{Id, Map};
use conrod_core::text::rt::gpu_cache::Cache;
use conrod_core::Ui;
use derive_macros_helpers::{Bounded, Enumerable};
use log::error;
use opengl_graphics::{GlGraphics, Texture};
use piston_window::{
    Context, Events, Graphics, Input, PistonWindow, RenderArgs, TextureSettings, UpdateArgs,
};
use std::fmt::Debug;

pub struct GUI<Ids> {
    pub image_map: Map<opengl_graphics::Texture>,
    pub image_ids: Vec<Id>,
    pub ui: Ui,
    pub ids: Ids,
    pub fullscreen: bool,
}

pub fn cache_queued_glyphs<'a>(
    text_vertex_data: &'a mut Vec<u8>,
) -> impl FnMut(
    &mut GlGraphics,
    &mut opengl_graphics::Texture,
    conrod_core::text::rt::Rect<u32>,
    &[u8],
) + 'a {
    use piston_window::texture::UpdateTexture;
    move |_graphics: &mut GlGraphics,
          cache: &mut opengl_graphics::Texture,
          rect: conrod_core::text::rt::Rect<u32>,
          data: &[u8]| {
        let offset = [rect.min.x, rect.min.y];
        let size = [rect.width(), rect.height()];
        let format = piston_window::texture::Format::Rgba8;
        text_vertex_data.clear();
        text_vertex_data.extend(data.iter().flat_map(|&b| vec![255, 255, 255, b]));
        UpdateTexture::update(cache, &mut (), format, &text_vertex_data[..], offset, size)
            .expect("failed to update texture")
    }
}

impl<Ids> GUI<Ids> {
    pub fn draw(
        &self,
        text_texture_cache: &mut Texture,
        glyph_cache: &mut Cache,
        cache_queued_glyphs: impl FnMut(
            &mut GlGraphics,
            &mut opengl_graphics::Texture,
            conrod_core::text::rt::Rect<u32>,
            &[u8],
        ),
        context: Context,
        gl: &mut GlGraphics,
    ) {
        // Specify how to get the drawable texture from the image. In this case, the image
        // *is* the texture.
        fn texture_from_image<T>(img: &T) -> &T {
            img
        }

        let view = context.store_view();

        conrod_piston::draw::primitives(
            self.ui.draw(),
            view,
            gl,
            text_texture_cache,
            glyph_cache,
            &self.image_map,
            cache_queued_glyphs,
            texture_from_image,
        )
    }

    pub fn set_fullscreen(&mut self, window: &mut PistonWindow, fullscreen: bool) {
        if fullscreen {
            let monitor = window.window.window.get_current_monitor();
            window.window.window.set_fullscreen(Some(monitor));
            self.fullscreen = true;
        } else {
            window.window.window.set_fullscreen(None);
            self.fullscreen = false;
        }
    }

    pub fn toggle_fullscreen(&mut self, window: &mut PistonWindow) {
        self.set_fullscreen(window, !self.fullscreen)
    }
}

pub fn create_ui(window: &PistonWindow) -> Ui {
    use super::get_asset_path;
    use piston_window::Window;

    let size = window.window.draw_size();

    //construct Ui
    let mut ui = conrod_core::UiBuilder::new([size.width, size.height]).build();

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = get_asset_path();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();
    ui
}

pub struct RenderContext<'font, T = opengl_graphics::Texture> {
    pub text_texture_cache: T,
    pub text_vertex_data: Vec<u8>,
    pub glyph_cache: Cache<'font>,
}

pub trait Application<'a> {
    type RR;
    type IR;
    type UR;
    type GUI;
    type RP;
    type UP;

    fn render(
        &self,
        gui: &Self::GUI,
        render_param: &Self::RP,
        gl: &mut GlGraphics,
        context: Context,
        render_context: &mut RenderContext,
        render_args: &RenderArgs,
    ) -> Self::RR;
    fn input(
        &mut self,
        gui: &mut Self::GUI,
        event: Input,
        event_loop: &mut Events,
        window: &mut PistonWindow,
    ) -> Self::IR;

    fn update(
        &mut self,
        gui: &mut Self::GUI,
        up: &mut Self::UP,
        update_args: UpdateArgs,
        window: &mut PistonWindow,
    ) -> Self::UR;
}

pub type TextureMap<G, K> = std::collections::btree_map::BTreeMap<K, <G as Graphics>::Texture>;

pub fn load_textures<K: Ord + Debug + Enumerable + Bounded + ToString>() -> TextureMap<GlGraphics, K>
{
    let mut texture_map = <TextureMap<GlGraphics, _>>::new();

    for tile_index in K::enumerate_all() {
        let file_name = tile_index.to_string();
        load_texture_into_map(&mut texture_map, tile_index, &file_name);
    }

    texture_map
}

fn load_texture_into_map<K: Ord + Debug>(
    texture_map: &mut TextureMap<GlGraphics, K>,
    key: K,
    name: &str,
) {
    let assets = super::get_asset_path();
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
