use crate::{
    app::{Action, UpdateAction},
    game::GameState,
    game::{LevelTemplate, TileTextureIndex},
    gui::MenuState::InGame,
};
use nannou::prelude::*;
use nannou_egui::{Egui, egui::{Ui, Key}};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::rc::Rc;

use learning_conrod_core::{
    get_asset_path,
    gui::{Application, TextureMap},
};
use log::trace;


#[derive(Debug)]
pub enum MenuState {
    InGame(GameState, bool),
    LevelSelect(LevelSelectState),
}

#[derive(Debug)]
pub struct LevelSelectState(Vec<Rc<LevelTemplate>>);

pub trait Menu: Debug {
    fn menu_name(&self) -> Cow<'static, str>;

    fn view_menu(&self, ui: &mut Ui);

    fn handle_esc(&mut self, window: WindowId) -> UpdateAction;
}

impl MenuState {
    pub(crate) fn open_level_selection() -> Self {
        let levels = crate::game::level::loading::load_levels(get_asset_path().as_path())
            .unwrap_or_else(|_err| Vec::new())
            .into_iter()
            .map(Rc::new)
            .collect();

        MenuState::LevelSelect(LevelSelectState(levels))
    }
}

impl MenuState {

    pub(crate) fn view(
        &self,
        app: &nannou::App,
        frame: &nannou::Frame,
        egui: &Egui,
        texture_map: &TextureMap<TileTextureIndex>
    ){
    match self {
        MenuState::InGame(game_state,false) => {
            let draw = app.draw();
                draw.background().color(nannou::color::named::RED);
            draw.to_frame(app, frame).unwrap();
            game_state.draw_game(app, frame, texture_map);
        }
        _ => {
            let draw = app.draw();
                draw.background().color(nannou::color::named::BLUE);
            draw.to_frame(app, frame).unwrap();
        }
    }}

    pub(crate) fn update(
        &mut self,
        app: &App,
        update: Update,
        main_window: WindowId,
        egui: &mut Ui
    ) {
        match self {
            MenuState::InGame(state, paused@true) => {
                egui.label("Pause Menu");
                if egui.button("Continue").clicked() {
                    *paused = false;
                }
            }
            MenuState::LevelSelect(level_list) => {
                egui.label("Level Selection");

                for level in level_list.0.iter() {
                    if egui.button(&level.name).clicked() {
                        let state = GameState::new(level.clone());
                        *self = InGame(state, true);
                        break;
                    }
                }
            }
            MenuState::InGame(state, paused@false) => {
                match state {
                    GameState::Won { .. } => {
                        egui.label("Won");
                    }
                    GameState::GameState {
                        show_hud,
                        rotation,
                        ..
                    } => {
                        if *show_hud {
                            egui.label("HUD");
                        }
                        // Rotate 2 radians per second.
                        let delta: f32 = todo!();
                        *rotation += 8.0 * delta;

                        let mut key_map: BTreeMap<Key, Action> = BTreeMap::new();

                        key_map.insert(Key::W, Action::Up);
                        key_map.insert(Key::A, Action::Left);
                        key_map.insert(Key::S, Action::Down);
                        key_map.insert(Key::D, Action::Right);


                        key_map
                            .iter()
                            .filter(|(&k, _)|  egui.input().key_down(k))
                            .for_each(|(_, action)| action.perform(state));
                    }
                }
                state.handle_input();
            }
        }
    }
}


impl Menu for MenuState {
    fn menu_name(&self) -> Cow<'static, str> {
        match self {
            MenuState::InGame(_, true) => Cow::Borrowed("Pause Menu"),
            MenuState::LevelSelect(_) => Cow::Borrowed("Level Selection"),
            MenuState::InGame(_, false) => Cow::Borrowed(""),
        }
    }

    fn handle_esc(&mut self, _window: WindowId) -> UpdateAction {
        match self {
            MenuState::InGame(_state, true) => *self = Self::open_level_selection(),
            MenuState::LevelSelect(_) => {
                return UpdateAction::Close;
            }
            InGame(state, paused@false) => *paused = true,
        }

        UpdateAction::Nothing
    }

    fn view_menu(&self, ui: &mut Ui) {
        todo!()
    }
}
