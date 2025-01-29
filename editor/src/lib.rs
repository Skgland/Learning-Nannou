use derive_macros::{Bounded, Enumerable};
use derive_macros_helpers::{Bounded, Enumerable};
use learning_conrod_core::gui::{load_textures, Application, TextureMap};
use learning_conrod_game::game::color::{IN_GAME_BACKGROUND, MENU_BACKGROUND};
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
    _texture_map: TextureMap<EditorTextureIndex>,
    state: EditorState,
}

pub enum EditorState {
    MainMenu,
    CreateLevel,
    LoadLevel(Vec<(LevelTemplate, PathBuf)>),
    Editor(Editor, Option<GameApp>),
}

pub struct Editor {
    _level: LevelTemplate,
    _saved: bool,
    _file: Option<PathBuf>,
}

impl EditorState {
    fn view(&self, app: &App, frame: &nannou::Frame, egui: &Egui) {
        match self {
            EditorState::Editor(_, None) => {
                let draw = app.draw();
                draw.background().color(MENU_BACKGROUND);
                draw.to_frame(app, frame).unwrap();

                //TOD draw editor content
            }
            EditorState::Editor(_editor, Some(game)) => {
                game.view(app, frame, egui);
            }
            _ => {
                let draw = app.draw();
                draw.background().color(IN_GAME_BACKGROUND);
                draw.to_frame(app, frame).unwrap();
            }
        }
        egui.draw_to_frame(frame).unwrap();
    }
}

impl EditorApp {
    fn new(texture_map: TextureMap<EditorTextureIndex>) -> EditorApp {
        EditorApp {
            _texture_map: texture_map,
            state: EditorState::MainMenu,
        }
    }
}

impl Application<'_> for EditorApp {
    type ViewResult = ();
    type RawEventResult = ();
    type UpdateResult = ();

    fn view(&self, app: &App, frame: &Frame, egui: &Egui) {
        self.state.view(app, frame, egui);
    }

    fn update(
        &mut self,
        _app: &App,
        _update: Update,
        _egui: &mut Egui,
        _main_window: WindowId,
    ) -> Self::UpdateResult {
        // TODO
    }
}

pub fn create_editor_app(app: &App) -> Result<EditorApp, String> {
    let texture_map = load_textures::<EditorTextureIndex>(app);

    Ok(EditorApp::new(texture_map))
}
