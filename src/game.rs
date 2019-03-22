use std::collections::btree_set::BTreeSet;

pub struct GameState {
    pub rotation: f64,
    //x and y offset of the rotating square
    pub x_offset: f64,
    pub y_offset: f64,
    pub level: Level,
}

pub struct Level {
    pub name: String
}

impl GameState {
    pub fn new(level: Level) -> GameState {
        GameState {
            // Rotation for the square.
            rotation: 0.0,
            x_offset: 0.0,
            y_offset: 0.0,
            level,
        }
    }

    pub fn handle_input(&self) -> () {}
}

pub enum DIRECTION {
    UP,DOWN,NORTH,EAST,SOUTH,WEST
}

pub enum TileType {
    Button,
    Start,
    Goal,
    Wall,
    OneWay(self::DIRECTION),
    Gate{hidden:bool},
    Ladder,
    Path,
}
