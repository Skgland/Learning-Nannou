use crate::{
    app::{Action, UpdateAction},
    game::GameState,
    game::LevelTemplate,
    gui::MenuState::InGame,
};
use conrod_core::{
    position::Positionable, position::Sizeable, widget, widget::Widget, widget_ids, Labelable,
    UiCell,
};
use piston_window::{clear, Context, Events, Input, Key, PistonWindow, RenderArgs, UpdateArgs};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::rc::Rc;

use crate::game::TileTextureIndex;
use learning_conrod_core::gui::TextureMap;
use learning_conrod_core::{
    get_asset_path,
    gui::{Application, RenderContext, GUI},
};
use log::trace;
use opengl_graphics::{GlGraphics, Texture};

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct GameIds {
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

#[derive(Debug)]
pub enum MenuState {
    PauseMenu(GameState),
    InGame(GameState),
    LevelSelect(LevelSelectState),
}

#[derive(Debug)]
pub struct LevelSelectState(Vec<Rc<LevelTemplate>>);

pub trait Menu: Debug {
    fn menu_name(&self) -> Cow<'static, str>;

    fn handle_input(&self, event: piston_window::Input);

    fn handle_esc(&mut self, window: &mut PistonWindow) -> UpdateAction;
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

impl<'a> Application<'a> for MenuState {
    type RR = ();
    type IR = ();
    type UR = ();
    type GUI = ();
    type RP = TextureMap<GlGraphics, TileTextureIndex>;
    type UP = (UiCell<'a>, &'a mut GameIds);

    fn render<'font>(
        &self,
        _gui: &Self::GUI,
        texture_map: &Self::RP,
        gl: &mut GlGraphics,
        context: Context,
        _render_context: &mut RenderContext<'font, Texture>,
        render_args: &RenderArgs,
    ) -> Self::RR {
        match self {
            MenuState::InGame(game_state) => {
                clear(crate::game::color::D_RED, gl);
                game_state.draw_game(render_args, context, gl, texture_map);
            }
            _ => {
                clear(crate::game::color::BLUE, gl);
            }
        }
    }

    fn input(
        &mut self,
        _gui: &mut Self::GUI,
        _event: Input,
        _event_loop: &mut Events,
        _window: &mut PistonWindow,
    ) -> Self::IR {
        unimplemented!()
    }

    fn update(
        &mut self,
        _gui: &mut Self::GUI,
        (ui, ids): &mut Self::UP,
        update_args: UpdateArgs,
        _window: &mut PistonWindow,
    ) -> Self::UR {
        match self {
            MenuState::PauseMenu(_state) => {
                widget::Text::new("Pause Menu")
                    .font_size(30)
                    .mid_top_of(ids.main_canvas)
                    .set(ids.menu_title, ui);
                widget::Button::new()
                    .label("Continue")
                    .label_font_size(30)
                    .middle_of(ids.main_canvas)
                    .padded_kid_area_wh_of(ids.main_canvas, ui.win_h / 4.0)
                    .set(ids.contiue_button, ui);
            }
            MenuState::LevelSelect(level_list) => {
                widget::Text::new("Level Selection")
                    .font_size(30)
                    .mid_top_of(ids.main_canvas)
                    .set(ids.menu_title, ui);

                ids.level_buttons
                    .resize(level_list.0.len(), &mut ui.widget_id_generator());

                for (button_id, level) in ids.level_buttons.iter().zip(level_list.0.iter()) {
                    let clicked = widget::Button::new().label(&level.name).set(*button_id, ui);
                    if clicked.was_clicked() {
                        let state = GameState::new(level.clone());
                        *self = InGame(state);
                        break;
                    }
                }
            }
            MenuState::InGame(state) => {
                match state {
                    GameState::Won { .. } => widget::Text::new("Won")
                        .font_size(30)
                        .mid_top_of(ids.main_canvas)
                        .set(ids.menu_title, ui),
                    GameState::GameState {
                        show_hud,
                        rotation,
                        keys_down,
                        ..
                    } => {
                        if *show_hud {
                            widget::Text::new("HUD")
                                .font_size(30)
                                .mid_top_of(ids.main_canvas)
                                .set(ids.menu_title, ui)
                        }

                        // Rotate 2 radians per second.
                        *rotation += 8.0 * update_args.dt;

                        let mut key_map: BTreeMap<Key, Action> = BTreeMap::new();

                        key_map.insert(Key::W, Action::UP);
                        key_map.insert(Key::A, Action::LEFT);
                        key_map.insert(Key::S, Action::DOWN);
                        key_map.insert(Key::D, Action::RIGHT);

                        let down_clone = keys_down.clone();

                        key_map
                            .iter()
                            .filter(|(&k, _)| down_clone.borrow().contains(&k))
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
            MenuState::PauseMenu(_) => Cow::Borrowed("Pause Menu"),
            MenuState::LevelSelect(_) => Cow::Borrowed("Level Selection"),
            MenuState::InGame(_) => Cow::Borrowed(""),
        }
    }

    fn handle_input(&self, event: piston_window::Input) {
        use piston_window::{Button, ButtonArgs, ButtonState};
        match self {
            MenuState::InGame(state) => {
                if let GameState::GameState { keys_down, .. } = state {
                    if let piston_window::Input::Button(ButtonArgs {
                        button: Button::Keyboard(key),
                        state: button_state,
                        ..
                    }) = event
                    {
                        match button_state {
                            ButtonState::Press => keys_down.try_borrow_mut().unwrap().insert(key),
                            ButtonState::Release => {
                                keys_down.try_borrow_mut().unwrap().remove(&key)
                            }
                        };
                        trace!("{:?}", key);
                    };
                }
            }
            _ => {}
        }
    }

    fn handle_esc(&mut self, _window: &mut PistonWindow) -> UpdateAction {
        match self {
            MenuState::PauseMenu(_state) => *self = Self::open_level_selection(),
            MenuState::LevelSelect(_) => {
                return UpdateAction::Close;
            }
            InGame(state) => *self = MenuState::PauseMenu(state.clone()),
        }

        UpdateAction::Nothing
    }
}
