use rusttype::gpu_cache::Cache;
use conrod_core::image::Map;
use conrod_core::Ui;
use core::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Debug;
use conrod_core::widget_ids;
use opengl_graphics::GlGraphics;
use conrod_core::widget;
use conrod_core::position::Positionable;
use conrod_core::Labelable;
use conrod_core::position::Sizeable;
use conrod_core::widget::Widget;
use conrod_core::UiCell;


// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        canvas,
        title,
        contiue,
        level_select,
        options,
        back,
        quit,
    }
}

pub struct RenderContext<'font> {
    pub gl:  GlGraphics,
    pub text_texture_cache: opengl_graphics::Texture,
    pub text_vertex_data: Vec<u8>,
    pub glyph_cache: Cache<'font>,
}

pub struct GUI{
    pub image_map: Map<opengl_graphics::Texture>,
    pub ui: Ui,
    pub ids: Ids,
    pub active_menu: GUIVisibility,
    pub fullscreen: bool,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum GUIVisibility {
    //*NO GUI VISIBLE (ONLY GAME VISIBLE)
    GameOnly,
    //*NON-INTERACTIVE HUD VISIBLE ON TOP OF GAME
    HUD,
    //*INTERACTIVE MENU VISIBLE ON TOP OF GAME
    OverlayMenu(MenuType),
    //*ONLY MENU VISIBLE (NO GAME VISIBLE)
    MenuOnly(MenuType),
}

impl Display for GUIVisibility {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        (self as &Debug).fmt(f)
    }
}


#[derive(Debug)]
pub enum MenuType {
    Main,
    Pause,
    LevelSelect,
    Custom(Box<dyn Menu>),
}


impl Display for MenuType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        (self as &Debug).fmt(f)
    }
}

pub trait Menu: Debug {
    fn menu_name(&self) -> String;

    fn handle_input(&self) -> ();

    fn update(&self, ui: &mut UiCell, ids: &Ids) -> ();

    fn back(&self) -> Option<GUIVisibility>;
}

impl Menu for MenuType {
    fn menu_name(&self) -> String {
        match self {
            MenuType::Main => String::from("Main Menu"),
            MenuType::Pause => String::from("Pause Menu"),
            MenuType::LevelSelect => String::from("Level Selection"),
            MenuType::Custom(menu) => menu.menu_name(),
        }
    }

    fn handle_input(&self) -> () {
        match self {
            MenuType::Main => unimplemented!(),
            MenuType::Pause => unimplemented!(),
            MenuType::LevelSelect => unimplemented!(),
            MenuType::Custom(menu) => menu.handle_input()
        }
    }

    fn update(&self,ui: &mut UiCell, ids:&Ids) -> () {
        match self {
            MenuType::Custom(menu) => menu.update(ui,ids),
            MenuType::Pause => {
                widget::Text::new("Pause Menu").font_size(30).mid_top_of(ids.canvas).set(ids.title, ui);
                widget::Button::new().label("Continue")
                                     .label_font_size(30)
                                     .middle_of(ids.canvas)
                                     .padded_kid_area_wh_of(ids.canvas, ui.win_h / 4.0)
                                     .set(ids.contiue, ui);
            },
            MenuType::LevelSelect =>
                widget::Text::new("Level Selection").font_size(30).mid_top_of(ids.canvas).set(ids.title, ui),
            MenuType::Main =>
                widget::Text::new("Main Menu").font_size(30).mid_top_of(ids.canvas).set(ids.title, ui),

        }
    }

    fn back(&self) -> Option<GUIVisibility> {
        match self {
            MenuType::Main => None,
            MenuType::Pause => Some(GUIVisibility::MenuOnly(MenuType::LevelSelect)),
            MenuType::LevelSelect => Some(GUIVisibility::MenuOnly(MenuType::Main)),
            MenuType::Custom(menu) => menu.back()
        }
    }
}



