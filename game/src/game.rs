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

pub mod toml_fix;
pub mod level;
pub mod color;


#[derive(Clone)]
pub struct PlayerCoordinate {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone)]
pub struct GameState {
    //current angle of the rotating square
    pub rotation: f64,
    //x and y offset of the rotating square
    pub position: PlayerCoordinate,

    //current level
    pub level_template: level::LevelTemplate,
    pub level_state: level::LevelState,
}


impl GameState {
    pub fn new(level: level::LevelTemplate) -> GameState {
        GameState {
            // Rotation for the square.
            rotation: 0.0,
            position: PlayerCoordinate { x: 0.0, y: 0.0 },
            level_state: level.init_state.clone(),
            level_template: level,
        }
    }

    pub fn handle_input(&self) -> () {}

    pub fn draw_player<G: Graphics>(&self, context: Context, gl: &mut G, texture_map: &TextureMap) -> () {
        let transform = context.rot_rad(self.rotation).trans(-PLAYER_SIZE / 2.0, -PLAYER_SIZE / 2.0).transform;
        rectangle(PLAYER_COLOR, PLAYER_SQUARE, transform, gl);
    }
}


pub const TILE_SIZE: f64 = 64.0;
pub const PLAYER_SIZE: f64 = 45.0;
const PLAYER_SQUARE: graphics::types::Rectangle = [0.0, 0.0, PLAYER_SIZE, PLAYER_SIZE];
const PLAYER_COLOR: color::Color = color::RED;
