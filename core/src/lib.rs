use conrod_core::image::{Map, Id};
use conrod_core::Ui;

use piston::input::{RenderArgs, UpdateArgs, Input};
use piston_window::PistonWindow;
use conrod_core::text::rt::gpu_cache::Cache;
use graphics::Graphics;
use glutin_window::GlutinWindow;
use piston::event_loop::Events;

pub mod  error;

pub struct GUI<Ids> {
    pub image_map: Map<opengl_graphics::Texture>,
    pub image_ids: Vec<Id>,
    pub ui: Ui,
    pub ids: Ids,
    pub fullscreen: bool,
}

pub struct RenderContext<'font, G: Graphics> {
    pub gl: G,
    pub text_texture_cache: opengl_graphics::Texture,
    pub text_vertex_data: Vec<u8>,
    pub glyph_cache: Cache<'font>,
}

pub trait Application {

    type RR;
    type IR;
    type UR;

    fn render(&self,render_context: &mut RenderContext<opengl_graphics::GlGraphics>, render_args: &RenderArgs) -> Self::RR;
    fn input(&mut self, event: Input, event_loop: &mut Events, window: &mut PistonWindow<GlutinWindow>) -> Self::IR;
    fn update(&mut self,update_args: &UpdateArgs, window: &mut PistonWindow<GlutinWindow>) -> Self::UR;

}
