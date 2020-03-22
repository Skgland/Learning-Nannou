use conrod_core::image::{Id, Map};
use conrod_core::Ui;

pub struct GUI<Ids> {
    pub image_map: Map<opengl_graphics::Texture>,
    pub image_ids: Vec<Id>,
    pub ui: Ui,
    pub ids: Ids,
    pub fullscreen: bool,
}

use opengl_graphics::{GlGraphics, Texture};

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

use conrod_core::text::rt::gpu_cache::Cache;
use conrod_piston::draw::Context;

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
}

use piston_window::PistonWindow;

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

use piston_window::Graphics;

pub struct RenderContext<'font, G: Graphics> {
    pub gl: G,
    pub text_texture_cache: opengl_graphics::Texture,
    pub text_vertex_data: Vec<u8>,
    pub glyph_cache: Cache<'font>,
}

use piston_window::Events;
use piston_window::{Input, RenderArgs, UpdateArgs};

pub trait Application {
    type RR;
    type IR;
    type UR;

    fn render(
        &self,
        render_context: &mut RenderContext<opengl_graphics::GlGraphics>,
        render_args: &RenderArgs,
    ) -> Self::RR;
    fn input(
        &mut self,
        event: Input,
        event_loop: &mut Events,
        window: &mut PistonWindow,
    ) -> Self::IR;
    fn update(&mut self, update_args: UpdateArgs, window: &mut PistonWindow) -> Self::UR;
}
