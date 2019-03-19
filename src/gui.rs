use rusttype::gpu_cache::Cache;
use conrod_core::image::Map;
use conrod_core::Ui;
use crate::Ids;

#[allow(dead_code)]
pub enum GUIVisibility {
    //*NO GUI VISIBLE (ONLY GAME VISIBLE)
    HIDDEN,
    //*NON-INTERACTIVE HUD VISIBLE ON TOP OF GAME
    HUD,
    //*INTERACTIVE MENU VISIBLE ON TOP OF GAME
    MENU,
    //*ONLY MENU VISIBLE (NO GAME VISIBLE)
    FULL,
}

pub struct GUI<'font> {
    pub text_vertex_data: Vec<u8>,
    pub glyph_cache: Cache<'font>,
    pub image_map: Map<opengl_graphics::Texture>,
    pub text_texture_cache: opengl_graphics::Texture,
    pub ui: Ui,
    pub ids: Ids,
    pub visibility: GUIVisibility,
}
