use conrod_core::widget_ids;
use derive_macros::{Bounded, Enumerable};
use derive_macros_helpers::{Bounded, Enumerable};
use learning_conrod_core::gui::{
    cache_queued_glyphs, create_ui, load_textures, Application, RenderContext, TextureMap, GUI,
};
use learning_conrod_game::game::color::{BLUE, D_RED};
use learning_conrod_game::game::{LevelTemplate, TileTextureIndex};
use learning_conrod_game::gui::GameIds;
use learning_conrod_game::GameApp;
use opengl_graphics::{GlGraphics, Texture};
use piston_window::{clear, Context, Events, Input, PistonWindow, RenderArgs, UpdateArgs};
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

widget_ids! {
    pub struct EditorIds {
        main_canvas,
        menu_title,
        level_buttons[],
        level_load_button,
        level_create_button,
        level_name_textbox,
        options_button,
        back_button,
    }
}

pub struct EditorApp {
    texture_map: TextureMap<GlGraphics, EditorTextureIndex>,
    state: EditorState,
}

pub enum EditorState {
    MainMenu,
    CreateLevel,
    LoadLevel(Vec<(LevelTemplate, PathBuf)>),
    Editor(Editor),
    Game(Editor, GameApp, GUI<GameIds>),
}

pub struct Editor {
    level: LevelTemplate,
    saved: bool,
    file: Option<PathBuf>,
}

impl Application<'_> for EditorState {
    type RR = ();
    type IR = ();
    type UR = ();
    type GUI = ();
    type RP = ();
    type UP = ();

    fn render(
        &self,
        gui: &Self::GUI,
        render_param: &Self::RP,
        gl: &mut GlGraphics,
        context: Context,
        render_context: &mut RenderContext<'_, Texture>,
        render_args: &RenderArgs,
    ) -> Self::RR {
        match self {
            EditorState::Editor(_) => {
                clear(D_RED, gl);
                //TOD draw editor content
            }
            EditorState::Game(editor, game, gui) => {
                game.render(gui, &(), gl, context, render_context, render_args);
            }
            _ => clear(BLUE, gl),
        }
    }

    fn input(
        &mut self,
        gui: &mut Self::GUI,
        event: Input,
        event_loop: &mut Events,
        window: &mut PistonWindow,
    ) -> Self::IR {
    }

    fn update(
        &mut self,
        gui: &mut Self::GUI,
        _up: &mut Self::UP,
        update_args: UpdateArgs,
        window: &mut PistonWindow,
    ) -> Self::UR {
    }
}

impl EditorApp {
    fn new(texture_map: TextureMap<GlGraphics, EditorTextureIndex>) -> EditorApp {
        EditorApp {
            texture_map,
            state: EditorState::MainMenu,
        }
    }
}

impl Application<'_> for EditorApp {
    type RR = ();
    type IR = ();
    type UR = ();
    type GUI = GUI<EditorIds>;
    type RP = ();
    type UP = ();

    fn render(
        &self,
        gui: &Self::GUI,
        _rp: &Self::RP,
        gl: &mut GlGraphics,
        context: Context,
        render_context: &mut RenderContext<'_>,
        render_args: &RenderArgs,
    ) -> Self::RR {
        self.state
            .render(&(), &(), gl, context, render_context, render_args);

        let cached_queued_glyphs = cache_queued_glyphs(&mut render_context.text_vertex_data);

        gui.draw(
            &mut render_context.text_texture_cache,
            &mut render_context.glyph_cache,
            cached_queued_glyphs,
            context,
            gl,
        )
    }

    fn input(
        &mut self,
        _gui: &mut Self::GUI,
        _event: Input,
        _event_loop: &mut Events,
        _window: &mut PistonWindow,
    ) -> Self::IR {
        //TODO
    }

    fn update(
        &mut self,
        _gui: &mut Self::GUI,
        _up: &mut Self::UP,
        _update_args: UpdateArgs,
        _window: &mut PistonWindow,
    ) -> Self::UR {
        //TODO
    }
}

pub fn create_editor_app(window: &PistonWindow) -> Result<(EditorApp, GUI<EditorIds>), String> {
    let mut ui = create_ui(window);

    let image_map = conrod_core::image::Map::new();

    let texture_map = load_textures::<EditorTextureIndex>();

    let generator = ui.widget_id_generator();
    let ids = EditorIds::new(generator);

    Ok((
        EditorApp::new(texture_map),
        GUI {
            ui,
            image_map,
            image_ids: vec![],
            ids,
            fullscreen: false,
        },
    ))
}
