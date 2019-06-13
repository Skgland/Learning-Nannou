#![allow(dead_code, unused_variables)]



use std::collections::btree_map::BTreeMap;
use graphics::{
    Context,
    Graphics,
    ImageSize,
    rectangle,
};
use piston_window::Transformed;
use serde::{
    Serialize,
    Deserialize,
};

pub use level::*;
use crate::TextureMap;

pub mod test_level;
pub mod level;
pub mod color;


#[derive(Clone,Debug)]
pub struct PlayerCoordinate {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone,Debug)]
pub struct GameState {
    //current angle of the rotating square
    pub rotation: f64,

    //x and y offset of the rotating square
    pub position: PlayerCoordinate,
    old_position: ObjectCoordinate,

    //current level
    pub level_template: level::LevelTemplate,
    pub level_state: level::LevelState,
}

impl From<&PlayerCoordinate> for ObjectCoordinate {
    fn from(player: &PlayerCoordinate) -> Self {
        ObjectCoordinate{x:(player.x/64.0 - TILE_SIZE / 128.0) as i64,y:(player.y/64.0 - TILE_SIZE / 128.0) as i64}
    }
}


impl GameState {
    pub fn new(level: level::LevelTemplate) -> GameState {
        GameState {
            // Rotation for the square.
            rotation: 0.0,
            position: PlayerCoordinate { x: 0.0, y: 0.0 },
            old_position: ObjectCoordinate{x:0,y:0},

            level_state: level.init_state.clone(),
            level_template: level,
        }
    }

    pub fn handle_input(&mut self) {
        let new_pos = (&self.position).into();
        if self.old_position != new_pos {
            self.old_position = new_pos;
            println!("Stepping on {:?}", &self.old_position);
            if let Some(fun) = self.level_state.tile_map.get_mut(&self.old_position).and_then(TileType::step_on){
                fun(self);
            }
        }
    }

    pub fn draw_player<G: Graphics>(&self, context: Context, gl: &mut G, texture_map: &TextureMap<G>) {
        let transform = context.rot_rad(self.rotation).trans(-PLAYER_SIZE / 2.0, -PLAYER_SIZE / 2.0).transform;
        rectangle(PLAYER_COLOR, PLAYER_SQUARE, transform, gl);
    }
}


pub const TILE_SIZE: f64 = 64.0;
pub const PLAYER_SIZE: f64 = 45.0;
const PLAYER_SQUARE: graphics::types::Rectangle = [0.0, 0.0, PLAYER_SIZE, PLAYER_SIZE];
const PLAYER_COLOR: color::Color = color::RED;
