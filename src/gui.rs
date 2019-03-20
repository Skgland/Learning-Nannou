use rusttype::gpu_cache::Cache;
use conrod_core::image::Map;
use conrod_core::Ui;
use core::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Debug;
use conrod_core::widget_ids;

#[allow(dead_code)]
#[derive(Debug)]
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

impl  Display for GUIVisibility {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        (self as &Debug).fmt(f)
    }
}

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        canvas,
        title,
        pause_menu
    }
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
