
use derive_macros::{Bounded, Enumerable};
use derive_macros_helpers::{Bounded, Enumerable};
use learning_conrod_core::gui::{TextureMap, Application, load_textures};
use learning_conrod_game::game::{LevelTemplate, TileTextureIndex};
use learning_conrod_game::GameApp;
use nannou::prelude::*;
use nannou_egui::Egui;
use std::path::PathBuf;

#[derive(Enumerable, Bounded, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum EditorTextureIndex {
    GameTile(TileTextureIndex),
    MapCenter,
}

impl From<TileTextureIndex> for EditorTextureIndex {
    fn from(tti: TileTextureIndex) -> Self {
        Self::GameTile(tti)
    }
}

impl ToString for EditorTextureIndex {
    fn to_string(&self) -> String {
        match self {
            EditorTextureIndex::GameTile(tile) => tile.to_string(),
            EditorTextureIndex::MapCenter => "editor_map_center".to_string(),
        }
    }
}

pub struct EditorApp {
    texture_map: TextureMap<EditorTextureIndex>,
    state: EditorState,
    egui: Egui
}

pub enum EditorState {
    MainMenu,
    CreateLevel,
    LoadLevel(Vec<(LevelTemplate, PathBuf)>),
    Editor(Editor, Option<GameApp>),
}

pub struct Editor {
    level: LevelTemplate,
    saved: bool,
    file: Option<PathBuf>,
}

impl EditorState {

    fn view(
        &self,
        app: &App,
        frame: &nannou::Frame,egui: &Egui) {

        match self {
            EditorState::Editor(_, None) => {
                let draw = app.draw();
                draw.background().color(nannou::color::named::RED);
                draw.to_frame(app, &frame).unwrap();

                //TOD draw editor content
            }
            EditorState::Editor(editor, Some(game)) => {
                game.view(app, frame);
            }
            _ => {
                let draw = app.draw();
                draw.background().color(nannou::color::named::BLUE);
                draw.to_frame(app, &frame).unwrap();
            },
        }
        egui.draw_to_frame(&frame).unwrap();
    }

}


impl EditorApp {
    fn new(app: &App, window_id: WindowId, texture_map: TextureMap<EditorTextureIndex>) -> EditorApp {
        let window = app.window(window_id).unwrap();
        let egui = Egui::from_window(&window);
        EditorApp {
            egui,
            texture_map,
            state: EditorState::MainMenu,
        }
    }
}

impl Application<'_> for EditorApp {
    type ViewResult = ();
    type RawEventResult = ();
    type UpdateResult = ();

    fn view(&self, app: &App, frame: &Frame) {
        self.state.view(app, frame, &self.egui);
    }
}

pub fn create_editor_app(app:&App, main_window: WindowId) -> Result<EditorApp, String> {

    let texture_map = load_textures::<EditorTextureIndex>(app);

    Ok(EditorApp::new(app, main_window, texture_map))
}
