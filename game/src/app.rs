use derive_macros::*;
use nannou::prelude::*;
use nannou_egui::Egui;

use crate::game::TileTextureIndex;
use crate::{game::GameState, gui::*};
use learning_conrod_core::gui::{Application, TextureMap};

pub struct GameApp {
    pub(crate) texture_map: TextureMap<TileTextureIndex>,
    pub(crate) current_menu: MenuState,
}

#[derive(Bounded)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
}

impl Action {
    pub fn perform(&self, state: &mut GameState) {
        if let GameState::GameState { position, .. } = state {
            match self {
                Action::Up => position.y -= 0.5 / 64.0,
                Action::Down => position.y += 0.5 / 64.0,
                Action::Left => position.x -= 0.5 / 64.0,
                Action::Right => position.x += 0.5 / 64.0,
            }
        }
    }
}

impl Application<'_> for GameApp {
    type ViewResult = ();
    type RawEventResult = ();
    type UpdateResult = UpdateAction;

    fn view(&self, app: &nannou::App, frame: &nannou::Frame, egui: &Egui) -> Self::ViewResult {
        self.current_menu.view(app, frame, egui, &self.texture_map);
    }

    fn update(
        &mut self,
        app: &App,
        update: Update,
        egui: &mut Egui,
        main_window: WindowId,
    ) -> Self::UpdateResult {
        let mut ctx = egui.begin_frame();
        self.current_menu.update(app, update, &mut ctx, main_window)
    }
}

impl GameApp {
    pub fn new(texture_map: TextureMap<TileTextureIndex>, init_menu: MenuState) -> Self {
        GameApp {
            texture_map,
            current_menu: init_menu,
        }
    }
}

pub enum UpdateAction {
    Nothing,
    Close,
}
