pub use level::*;
use std::rc::Rc;
use nannou::prelude::*;

use learning_conrod_core::gui::TextureMap;
use log::trace;

pub mod color;
pub mod level;
pub mod test_level;

#[derive(Clone, Debug)]
pub struct PlayerCoordinate {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug)]
pub enum GameState {
    Won {
        level_template: Rc<level::LevelTemplate>,
    },
    GameState {
        //current angle of the rotating square
        rotation: f32,

        show_hud: bool,

        //x and y offset of the rotating square
        position: PlayerCoordinate,
        old_position: ObjectCoordinate,

        //current level
        level_template: Rc<level::LevelTemplate>,
        level_state: level::LevelState,
    },
}

impl From<&PlayerCoordinate> for ObjectCoordinate {
    fn from(player: &PlayerCoordinate) -> Self {
        ObjectCoordinate {
            x: player.x.round() as i64,
            y: player.y.round() as i64,
        }
    }
}

impl From<&mut PlayerCoordinate> for ObjectCoordinate {
    fn from(player: &mut PlayerCoordinate) -> Self {
        (&*player).into()
    }
}

impl GameState {
    pub fn new(level: Rc<level::LevelTemplate>) -> GameState {
        GameState::GameState {
            // Rotation for the square.
            rotation: 0.0,
            show_hud: true,
            position: PlayerCoordinate { x: 0.0, y: 0.0 },
            old_position: ObjectCoordinate { x: 0, y: 0 },

            level_state: level.init_state.clone(),
            level_template: level,
        }
    }

    pub fn handle_input(&mut self) {
        if let GameState::GameState {
            position,
            old_position,
            level_state,
            ..
        } = self
        {
            let new_pos: ObjectCoordinate = position.into();
            if *old_position != new_pos {
                *old_position = new_pos;
                trace! {"Stepping on {:?} with {:?}", old_position, position}
                if let Some(fun) = level_state
                    .tile_map
                    .get_mut(old_position)
                    .and_then(TileType::step_on)
                {
                    fun(self);
                }
            }
        }
    }

    pub fn draw_game(
        &self,
        app: &App,
        frame: &Frame,
        texture_map: &TextureMap<TileTextureIndex>,
    ) {
        let size = frame.rect();
        if let GameState::GameState { level_state, .. } = self {
            let (x, y) = (size.w() / 2.0, size.h() / 2.0);

            let draw = app.draw().translate(Vec3::new(x,y,0.0));

            for (coord, tile) in &level_state.tile_map {
                tile.draw_tile(&draw, texture_map, coord, self);
            }

            // Draw a box rotating around the middle of the screen.

            self.draw_player(&draw,  texture_map);
        }
    }

    pub fn draw_player(
        &self,
        draw: &Draw,
        _texture_map: &TextureMap<TileTextureIndex>,
    ) {
        if let GameState::GameState { rotation, .. } = self {
            draw.rect()
                .rotate(*rotation)
                .x_y(-PLAYER_SIZE / 2.0, -PLAYER_SIZE / 2.0)
                .w_h(PLAYER_SIZE, PLAYER_SIZE).color(nannou::color::named::RED);
        }
    }
}

pub const TILE_SIZE: f32 = 64.0;
pub const PLAYER_SIZE: f32 = 45.0;
const PLAYER_COLOR: color::Color = color::RED;
