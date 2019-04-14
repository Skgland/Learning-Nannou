#![allow(dead_code)]

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
use piston_window::PistonWindow;
use glutin_window::GlutinWindow;
use piston::window::Window;
use crate::game::GameState;
use crate::game::LevelTemplate;
use crate::game::PlayerCoordinate;
use std::collections::btree_map::BTreeMap;
use crate::game::LevelState;
use crate::game::ObjectCoordinate;
use crate::game::TileType;
use crate::gui::GUIVisibility::HUD;
use conrod_core::image::Id;
use crate::game::Connections;


// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        canvas,
        title,
        contiue,
        level_select[],
        options,
        back,
        quit,
    }
}

pub struct RenderContext<'font> {
    pub gl: GlGraphics,
    pub text_texture_cache: opengl_graphics::Texture,
    pub text_vertex_data: Vec<u8>,
    pub glyph_cache: Cache<'font>,
}

pub struct GUI {
    pub image_map: Map<opengl_graphics::Texture>,
    pub image_ids: Vec<Id>,
    pub ui: Ui,
    pub ids: Ids,
    pub active_menu: GUIVisibility,
    pub fullscreen: bool,
}

#[allow(dead_code)]
pub enum GUIVisibility {
    //*NO GUI VISIBLE (ONLY GAME VISIBLE)
    GameOnly(GameState),
    //*NON-INTERACTIVE HUD VISIBLE ON TOP OF GAME
    //* E.g. Health, Hotbar
    HUD(GameState),
    //*INTERACTIVE MENU VISIBLE ON TOP OF GAME
    //* E.g. Inventory, Pause Menu
    OverlayMenu(MenuType, GameState),
    //*ONLY MENU VISIBLE (NO GAME VISIBLE)
    //* Main Menu, Level Selection, Options
    MenuOnly(MenuType),
}

impl Debug for GUIVisibility {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use self::GUIVisibility::*;
        match self {
            GameOnly(_) => {
                Ok(())
            }
            HUD(_) => {
                (&String::from("HUD") as &Debug).fmt(f)
            }
            MenuOnly(menu) |
            OverlayMenu(menu, _) => {
                (&menu.menu_name() as &Debug).fmt(f)
            }
        }
    }
}

impl GUIVisibility {
    pub fn handle_esc(&mut self, window: &mut PistonWindow<GlutinWindow>) -> () {
        match self {
            GUIVisibility::GameOnly(state) => {
                *self = GUIVisibility::HUD(state.clone())
            }
            GUIVisibility::HUD(state) => {
                *self = GUIVisibility::OverlayMenu(MenuType::Pause, state.clone())
            }
            GUIVisibility::MenuOnly(menu_type)|
                GUIVisibility::OverlayMenu(menu_type, _) => {
                let menu = menu_type.back();
                if let Some(menu) = menu {
                    *self = menu
                } else {
                    window.set_should_close(true);
                }
            }
        }
    }

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

    fn update(&self, ui: &mut UiCell, ids: &mut Ids) -> Option<GUIVisibility>;

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

    fn update(&self, ui: &mut UiCell, ids: &mut Ids) -> Option<GUIVisibility> {
        match self {
            MenuType::Custom(menu) => return menu.update(ui, ids),
            MenuType::Pause => {
                widget::Text::new("Pause Menu").font_size(30).mid_top_of(ids.canvas).set(ids.title, ui);
                widget::Button::new().label("Continue")
                                     .label_font_size(30)
                                     .middle_of(ids.canvas)
                                     .padded_kid_area_wh_of(ids.canvas, ui.win_h / 4.0)
                                     .set(ids.contiue, ui);
            }
            MenuType::LevelSelect => {
                widget::Text::new("Level Selection").font_size(30).mid_top_of(ids.canvas).set(ids.title, ui);

                ids.level_select.resize(1, &mut ui.widget_id_generator());

                let clicked = widget::Button::new().label("Test").mid_bottom_of(ids.canvas).set(ids.level_select[0], ui);
                if clicked.was_clicked() {
                    let mut tile_map = BTreeMap::new();
                    tile_map.insert(ObjectCoordinate { x: 0, y: 0 }, TileType::Start);
                    tile_map.insert(ObjectCoordinate { x: 0, y: 1 }, TileType::Path);
                    tile_map.insert(ObjectCoordinate { x: 1, y: 1 }, TileType::Path);
                    tile_map.insert(ObjectCoordinate { x: 2, y: 1 }, TileType::Path);
                    tile_map.insert(ObjectCoordinate { x: 2, y: 2 }, TileType::Path);
                    tile_map.insert(ObjectCoordinate { x: 2, y: 3 }, TileType::Path);
                    tile_map.insert(ObjectCoordinate { x: 1, y: 3 }, TileType::Path);
                    tile_map.insert(ObjectCoordinate { x: 0, y: 2 }, TileType::Wall(Connections{up:false,down:false,left:true,right:true}));
                    tile_map.insert(ObjectCoordinate { x: 1, y: 2 }, TileType::Wall(Connections { up: false, down: false, left: true, right: false }));
                    tile_map.insert(ObjectCoordinate { x: -1, y: 2 }, TileType::Wall(Connections { up: false, down: false, left: false, right: true }));
                    tile_map.insert(ObjectCoordinate { x: 0, y: 3 }, TileType::Goal { active: true });


                    let state = GameState::new(LevelTemplate { name: String::from("Test"), init_state: LevelState { tile_map }, start_position: PlayerCoordinate { x: 0.0, y: 0.0 } });

                    return Some(HUD(state));
                }
            }

            MenuType::Main =>
                widget::Text::new("Main Menu").font_size(30).mid_top_of(ids.canvas).set(ids.title, ui),
        }
        None
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



