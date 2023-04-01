use crate::{
    app::{Action, UpdateAction},
    game::GameState,
    game::{LevelTemplate, TileTextureIndex},
    gui::MenuState::InGame,
};
use nannou::prelude::*;
use nannou_egui::{
    egui::{self, Key},
    Egui, FrameCtx,
};

use std::collections::BTreeMap;
use std::fmt::Debug;
use std::rc::Rc;

use learning_conrod_core::{get_asset_path, gui::TextureMap};

#[derive(Debug)]
pub enum MenuState {
    InGame { state: GameState, paused: bool },
    LevelSelect(LevelSelectState),
}

#[derive(Debug)]
pub struct LevelSelectState(Vec<Rc<LevelTemplate>>);

pub trait Menu: Debug {
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

    fn handle_esc(&mut self, _window: WindowId) -> UpdateAction {
        match self {
            MenuState::InGame {
                state: _state,
                paused: true,
            } => *self = Self::open_level_selection(),
            MenuState::LevelSelect(_) => {
                return UpdateAction::Close;
            }
            InGame {
                state: _state,
                paused: paused @ false,
            } => *paused = true,
        }

        UpdateAction::Nothing
    }
}

impl MenuState {
    pub(crate) fn view(
        &self,
        app: &nannou::App,
        frame: &nannou::Frame,
        egui: &Egui,
        texture_map: &TextureMap<TileTextureIndex>,
    ) {
        match self {
            MenuState::InGame {
                state: game_state,
                paused: _,
            } => {
                let draw = app.draw();
                draw.background().color(nannou::color::named::RED);
                draw.to_frame(app, frame).unwrap();
                game_state.draw_game(app, frame, egui, texture_map);
                egui.draw_to_frame(frame).unwrap();
            }
            _ => {
                let draw = app.draw();
                draw.background().color(nannou::color::named::BLUE);
                draw.to_frame(app, frame).unwrap();
                egui.draw_to_frame(frame).unwrap();
            }
        }
    }

    pub(crate) fn update(
        &mut self,
        _app: &App,
        update: Update,
        ctx: &mut FrameCtx,
        main_window: WindowId,
    ) -> UpdateAction {
        if ctx.input().key_pressed(Key::Escape) {
            if let UpdateAction::Close = self.handle_esc(main_window) {
                return UpdateAction::Close;
            }
        }

        match self {
            MenuState::InGame {
                state: _,
                paused: paused @ true,
            } => {
                let back = egui::Window::new("Pause Menu")
                    .show(ctx, |ui| {
                        ui.label("Pause Menu");
                        if ui.button("Continue").clicked() {
                            *paused = false;
                            false
                        } else {
                            ui.button("Exit Level").clicked()
                        }
                    })
                    .and_then(|elem| elem.inner)
                    .unwrap_or(false);
                if back {
                    *self = Self::open_level_selection();
                }
                UpdateAction::Nothing
            }
            MenuState::LevelSelect(level_list) => {
                let result = egui::CentralPanel::default()
                    .show(ctx, |ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            ui.label("Level Selection");

                            ui.group(|ui| {
                                for level in level_list.0.iter() {
                                    if ui.button(&level.name).clicked() {
                                        return Some(level.clone());
                                    }
                                }
                                None
                            })
                            .inner
                        })
                    })
                    .inner;

                if let Some(level) = result.inner {
                    *self = MenuState::InGame {
                        state: GameState::new(level),
                        paused: false,
                    }
                }
                UpdateAction::Nothing
            }
            MenuState::InGame {
                state,
                paused: false,
            } => {
                match state {
                    GameState::Won { .. } => {
                        egui::Window::new("Won").show(ctx, |ui| {
                            ui.label("Congratulations!");
                            if ui.button("Retry Level").clicked() {
                                // TODO
                            }
                            if ui.button("Exit Level").clicked() {
                                // TODO
                            }
                        });

                        UpdateAction::Nothing
                    }
                    GameState::GameState {
                        show_hud, rotation, ..
                    } => {
                        // FIXME should be F1, but egui in the version used be nannou_egui does not have that key
                        if ctx.input().key_pressed(Key::H) {
                            *show_hud = !*show_hud;
                        }

                        if *show_hud {
                            egui::Window::new("").show(ctx, |ui| {
                                ui.label("HUD");
                            });
                        }

                        // Rotate 2 radians per second.
                        let delta: f32 = update.since_last.secs() as f32;
                        *rotation += 8.0 * delta;

                        let mut key_map: BTreeMap<Key, Action> = BTreeMap::new();

                        key_map.insert(Key::W, Action::Up);
                        key_map.insert(Key::A, Action::Left);
                        key_map.insert(Key::S, Action::Down);
                        key_map.insert(Key::D, Action::Right);

                        key_map
                            .iter()
                            .filter(|(&k, _)| ctx.input().key_down(k))
                            .for_each(|(_, action)| action.perform(state));

                        state.handle_input();

                        UpdateAction::Nothing
                    }
                }
            }
        }
    }
}
