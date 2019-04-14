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
use serde::Serializer;
use serde::Deserializer;
use std::convert::TryFrom;


pub mod color {
    pub type Color = [f32; 4];

    pub const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
    pub const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
    pub const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];
    pub const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
    pub const RED: Color = [1.0, 0.0, 0.0, 1.0];
    pub const D_RED: Color = [0.227, 0.513, 0.678, 1.0];
}

#[derive(Clone)]
pub struct PlayerCoordinate {
    pub x: f64,
    pub y: f64,
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct ObjectCoordinate {
    pub x: i64,
    pub y: i64,
}

#[derive(Clone)]
pub struct GameState {
    //current angle of the rotating square
    pub rotation: f64,
    //x and y offset of the rotating square
    pub position: PlayerCoordinate,

    //current level
    pub level_template: LevelTemplate,
    pub level_state: LevelState,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LevelTemplate {
    pub name: String,
    pub init_state: LevelState,
}


#[derive(Clone)]
pub struct LevelState {
    pub tile_map: BTreeMap<ObjectCoordinate, TileType>
}

impl Serialize for LevelState {
    fn serialize<S>(&self,serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        let map = self.tile_map.iter().flat_map(
            |(key, value)| {
                let mut  tmp = String::new();
                let mut tmp_ser = toml::Serializer::new(&mut tmp);
                if let Ok(()) = key.serialize(&mut tmp_ser){
                    Some((tmp, value))
                }else{
                    None
                }

            });
        serializer.collect_map(map)
    }
}

impl <'de> Deserialize<'de> for LevelState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {

        let map : BTreeMap<String,TileType>= BTreeMap::deserialize( deserializer)?;

        let map = map.iter().flat_map(
            |(key,value)| {

                let mut tmp_des = toml::de::Deserializer::new(key.as_str());
                if let Ok(result) = ObjectCoordinate::deserialize(&mut tmp_des){
                    Some((result, value.clone()))
                }else{
                    None
                }

        }).collect::<BTreeMap<ObjectCoordinate,TileType>>();

        Ok(LevelState{tile_map: map})
    }
}


impl GameState {
    pub fn new(level: LevelTemplate) -> GameState {
        GameState {
            // Rotation for the square.
            rotation: 0.0,
            position: PlayerCoordinate { x: 0.0, y: 0.0 },
            level_state: level.init_state.clone(),
            level_template: level,
        }
    }

    pub fn handle_input(&self) -> () {}

    pub fn draw_player<G: Graphics>(&self, context: Context, gl: &mut G, texture_map: &BTreeMap<TileTextureIndex, G::Texture>) -> () {
        let transform = context.rot_rad(self.rotation).trans(-PLAYER_SIZE / 2.0, -PLAYER_SIZE / 2.0).transform;
        rectangle(PLAYER_COLOR, PLAYER_SQUARE, transform, gl);
    }
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl TryFrom<u32> for Direction {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {


        Ok (match value {
            0 => Direction::UP,
            1 => Direction::DOWN,
            2 => Direction::NORTH,
            3 => Direction::EAST,
            4 => Direction::SOUTH,
            5 => Direction::WEST,
            _ => return Err(())
        })
    }
}

impl  Serialize for Direction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok,S::Error> where
        S: Serializer {
        (*self as u32).serialize(serializer)
    }
}


use serde::de::Error;

impl <'de>  Deserialize<'de> for Direction {


    fn deserialize<D>(deserializer: D) -> Result<Self,D::Error> where
        D: Deserializer<'de> {
        let int = u32::deserialize(deserializer)?;

        if let Ok(value) = Direction::try_from(int){
            Ok(value)
        }else{
            Err(D::Error::custom("Invalid enum variant!"))
        }
    }
}


impl Direction {
    pub fn inverted(&self) -> Self {
        use self::Direction::*;
        match self {
            Direction::UP => DOWN,
            Direction::DOWN => UP,
            Direction::EAST => WEST,
            Direction::WEST => EAST,
            Direction::SOUTH => NORTH,
            Direction::NORTH => SOUTH,
        }
    }
}

pub const TILE_SIZE: f64 = 64.0;
pub const PLAYER_SIZE: f64 = 45.0;
const PLAYER_SQUARE: graphics::types::Rectangle = [0.0, 0.0, PLAYER_SIZE, PLAYER_SIZE];
const PLAYER_COLOR: color::Color = color::RED;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Connections { pub up: bool, pub down: bool, pub left: bool, pub right: bool }


#[derive(Clone, Deserialize, Serialize)]
pub enum GateVisibility {
    Visible,
    Hidden(Box<TileType>),
}


#[derive(Clone,Serialize, Deserialize)]
pub enum TileType {
    Wall(Connections),
    Path,
    Ladder,
    Start,
    Goal { active: bool },
    Gate { open: bool, facing: Direction, hidden: GateVisibility },
    OneWay { inverted: bool, facing: Direction },
    Button { pressed: bool, inverted: bool, target: ObjectCoordinate },
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum TileTextureIndex {
    Wall(Connections),
    Path,
    Ladder,
    Start,
    Goal { active: bool },
    Gate { open: bool, facing: Direction },
    OneWay { facing: Direction },
    Button { pressed: bool },
}


impl TileType {
    pub fn apply_button(&mut self, active: bool) {
        match self {
            TileType::Goal { active: active_goal } => *active_goal = active,
            TileType::Gate { open, .. } => *open = active,
            TileType::OneWay { inverted, .. } => *inverted = active,
            _ => eprintln!("Tried to change the state of a single State Tile or Button Tile with a Button!"),
        }
    }

    pub fn draw_tile<G: Graphics>(&self, context: Context, gl: &mut G, texture_map: &BTreeMap<TileTextureIndex, G::Texture>, coord: &ObjectCoordinate, state: &GameState) where G::Texture: ImageSize {
        use graphics::*;

        use self::color::*;

        let rect = [0.0, 0.0, TILE_SIZE, TILE_SIZE];

        let adjusted = context.trans((coord.x as f64) * TILE_SIZE - state.position.x - TILE_SIZE / 2.0,
                                     (coord.y as f64) * TILE_SIZE - state.position.y - TILE_SIZE / 2.0);

        if let Some(texture) = texture_map.get(&self.tile_texture_id()) {
            let transform = adjusted.scale(TILE_SIZE / texture.get_width() as f64, TILE_SIZE / texture.get_height() as f64).transform;
            image(texture, transform, gl)
        } else {
            rectangle(D_RED, rect, adjusted.transform, gl)
        }
    }

    pub fn tile_texture_id(&self) -> TileTextureIndex {
        match self {
            TileType::Path => TileTextureIndex::Path,
            TileType::Start => TileTextureIndex::Start,
            TileType::Ladder => TileTextureIndex::Ladder,
            TileType::Goal { active } => TileTextureIndex::Goal { active: *active },
            TileType::Button { pressed, .. } => TileTextureIndex::Button { pressed: *pressed },
            TileType::OneWay { facing, inverted: false } => TileTextureIndex::OneWay { facing: *facing },
            TileType::OneWay { facing, inverted: true } => TileTextureIndex::OneWay { facing: facing.inverted() },
            TileType::Wall(connections) => TileTextureIndex::Wall(*connections),
            TileType::Gate { open, facing, hidden: GateVisibility::Visible } |
            TileType::Gate { open: open @ true, facing, .. } => TileTextureIndex::Gate { open: *open, facing: *facing },
            TileType::Gate { open: false, facing, hidden: GateVisibility::Hidden(mimic) } => mimic.tile_texture_id(),
        }
    }
}
