use derive_macros::*;
use nannou::prelude::*;
use nannou_egui::{Egui, egui};
use nannou_egui::egui::Key;

use crate::game::TileTextureIndex;
use crate::{game::GameState, gui::*};
use learning_conrod_core::gui::{ Application, TextureMap,};

pub struct GameApp {
    egui: Egui,
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

    fn view(
            &self,
            app: &nannou::App,
            frame: &nannou::Frame,
        ) -> Self::ViewResult {
            self.current_menu.view(app, frame, &self.egui, &self.texture_map);
    }


    fn raw_window_event(&mut self, app: &nannou::App, event: &nannou::winit::event::WindowEvent) -> Self::RawEventResult {
        self.egui.handle_raw_event(event);
    }


    fn update(
        &mut self,
        app: &App,
        update: Update,
        main_window: WindowId
    ) -> Self::UpdateResult {

        let ctx = self.egui.begin_frame();

        if ctx.input().key_pressed(Key::Escape) {
            if let UpdateAction::Close  = self.current_menu.handle_esc(main_window) {
                return UpdateAction::Close;
            }
        }

        // FIXME should be F1, but egui in the version used be nannou_egui does not have that key
        if ctx.input().key_pressed(Key::H) {
            // TODO toogle hud
        }

        egui::Window::new("Learning Conrod").show(&ctx, |ui| {
            self.current_menu.update(app, update, main_window, ui);
        });

        UpdateAction::Nothing
    }
}

impl GameApp {
    pub fn new(
        app: &App,
        main_window: WindowId,
        texture_map: TextureMap<TileTextureIndex>,
        init_menu: MenuState,
    ) -> Self {
        let main_window = app.window(main_window).unwrap();
        let egui = Egui::from_window(&main_window);
        GameApp {
            egui,
            texture_map,
            current_menu: init_menu,
        }
    }
}

pub enum UpdateAction {
    Nothing,
    Close,
}
