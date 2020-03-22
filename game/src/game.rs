#![allow(dead_code, unused_variables)]

use crate::TextureMap;
use conrod_core::input::RenderArgs;
pub use level::*;
use piston_window::{rectangle, Context, Graphics, Key, Transformed};
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;

use log::trace;

pub mod color;
pub mod level;
pub mod test_level;

#[derive(Clone, Debug)]
pub struct PlayerCoordinate {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug)]
pub enum GameState {
    Won {
        level_template: Rc<level::LevelTemplate>,
    },
    GameState {
        //current angle of the rotating square
        rotation: f64,

        show_hud: bool,

        keys_down: Rc<RefCell<BTreeSet<Key>>>,

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
            keys_down: Rc::new(RefCell::new(BTreeSet::new())),
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

    pub fn draw_game<G: Graphics>(
        &self,
        args: &RenderArgs,
        context: Context,
        gl: &mut G,
        texture_map: &TextureMap<G>,
    ) {
        if let GameState::GameState { level_state, .. } = self {
            let (x, y) = (args.width / 2.0, args.height / 2.0);

            let c = context.trans(x, y);

            for (coord, tile) in &level_state.tile_map {
                tile.draw_tile(c, gl, texture_map, coord, self);
            }

            // Draw a box rotating around the middle of the screen.

            self.draw_player(c, gl, texture_map);
        }
    }

    pub fn draw_player<G: Graphics>(
        &self,
        context: Context,
        gl: &mut G,
        texture_map: &TextureMap<G>,
    ) {
        if let GameState::GameState { rotation, .. } = self {
            let transform = context
                .rot_rad(*rotation)
                .trans(-PLAYER_SIZE / 2.0, -PLAYER_SIZE / 2.0)
                .transform;
            rectangle(PLAYER_COLOR, PLAYER_SQUARE, transform, gl);
        }
    }
}

pub const TILE_SIZE: f64 = 64.0;
pub const PLAYER_SIZE: f64 = 45.0;
const PLAYER_SQUARE: piston_window::types::Rectangle = [0.0, 0.0, PLAYER_SIZE, PLAYER_SIZE];
const PLAYER_COLOR: color::Color = color::RED;
