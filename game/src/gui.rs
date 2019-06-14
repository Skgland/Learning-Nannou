#![allow(dead_code)]

use rusttype::gpu_cache::Cache;
use conrod_core::image::Map;
use conrod_core::Ui;
use core::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Debug;
use conrod_core::widget_ids;
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
use crate::gui::GUIVisibility::HUD;
use conrod_core::image::Id;
use graphics::Graphics;

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        main_canvas,
        menu_title,
        level_buttons[],
        level_selection_button,
        editor_button,
        contiue_button,
        options_button,
        back_button,
        quit_button,
    }
}

pub struct RenderContext<'font, G: Graphics> {
    pub gl: G,
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
                Debug::fmt(&String::from("HUD"), f)
            }
            MenuOnly(menu) |
            OverlayMenu(menu, _) => {
                Debug::fmt(&menu.menu_name(), f)
            }
        }
    }
}

impl GUIVisibility {
    pub fn handle_esc(&mut self, window: &mut PistonWindow<GlutinWindow>) {
        match self {
            GUIVisibility::GameOnly(state) => {
                *self = GUIVisibility::HUD(state.clone())
            }
            GUIVisibility::HUD(state) => {
                *self = GUIVisibility::OverlayMenu(MenuType::Pause, state.clone())
            }
            GUIVisibility::MenuOnly(menu_type) |
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
        Debug::fmt(self, f)
    }
}


#[derive(Debug)]
pub enum MenuType {
    Main,
    Pause,
    LevelSelect,
    Editor(GameState),
    Custom(Box<dyn Menu>),
}


impl Display for MenuType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Debug::fmt(self, f)
    }
}

pub trait Menu: Debug {
    fn menu_name(&self) -> String;

    fn handle_input(&self);

    fn update(&self, ui: &mut UiCell, ids: &mut Ids, level_list: &[LevelTemplate]) -> Option<GUIVisibility>;

    fn back(&self) -> Option<GUIVisibility>;
}

impl Menu for MenuType {
    fn menu_name(&self) -> String {
        match self {
            MenuType::Main => String::from("Main Menu"),
            MenuType::Pause => String::from("Pause Menu"),
            MenuType::LevelSelect => String::from("Level Selection"),
            MenuType::Editor(GameState::GameState { level_template, .. })
            | MenuType::Editor(GameState::Won { level_template, .. }) => format!("Level Editor: {}", level_template.name),
            MenuType::Custom(menu) => menu.menu_name(),
        }
    }

    fn handle_input(&self) {
        match self {
            MenuType::Main => unimplemented!(),
            MenuType::Pause => unimplemented!(),
            MenuType::LevelSelect => unimplemented!(),
            MenuType::Editor(_) => unimplemented!(),
            MenuType::Custom(menu) => menu.handle_input()
        }
    }

    fn update(&self, ui: &mut UiCell, ids: &mut Ids, level_list: &[LevelTemplate]) -> Option<GUIVisibility> {
        match self {
            MenuType::Custom(menu) => menu.update(ui, ids, level_list),
            MenuType::Editor(_) => {
                None
            }
            MenuType::Pause => {
                widget::Text::new("Pause Menu").font_size(30).mid_top_of(ids.main_canvas).set(ids.menu_title, ui);
                widget::Button::new().label("Continue")
                    .label_font_size(30)
                    .middle_of(ids.main_canvas)
                    .padded_kid_area_wh_of(ids.main_canvas, ui.win_h / 4.0)
                    .set(ids.contiue_button, ui);
                None
            }
            MenuType::LevelSelect => {
                widget::Text::new("Level Selection").font_size(30).mid_top_of(ids.main_canvas).set(ids.menu_title, ui);


                ids.level_buttons.resize(level_list.len(), &mut ui.widget_id_generator());


                let mut result = None;

                for (button_id, level) in ids.level_buttons.iter().zip(level_list.iter()) {
                    let clicked = widget::Button::new().label(&level.name).set(*button_id, ui);
                    if clicked.was_clicked() {
                        let state = GameState::new(level.clone());
                        result = Some(HUD(state))
                    }
                }

                result
            }

            MenuType::Main => {
                widget::Button::new().label("Level Editor").middle_of(ids.main_canvas).set(ids.editor_button, ui);
                widget::Text::new("Main Menu").font_size(30).mid_top_of(ids.main_canvas).set(ids.menu_title, ui);
                None
            }
        }
    }

    fn back(&self) -> Option<GUIVisibility> {
        match self {
            MenuType::Main => None,
            MenuType::Editor(state) => Some(GUIVisibility::HUD(state.clone())),
            MenuType::Pause => Some(GUIVisibility::MenuOnly(MenuType::LevelSelect)),
            MenuType::LevelSelect => Some(GUIVisibility::MenuOnly(MenuType::Main)),
            MenuType::Custom(menu) => menu.back()
        }
    }
}

