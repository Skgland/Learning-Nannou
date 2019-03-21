
pub struct GameState {
    pub rotation: f64,
    //x and y offset of the rotating square
    pub x_offset: f64,
    pub y_offset: f64,
    pub level :Level,
}

pub struct Level{



}

impl  GameState {

    pub fn new() -> GameState {
        GameState{
            // Rotation for the square.
            rotation:0.0,
            x_offset:0.0,
            y_offset:0.0}
    }

    pub fn handle_input(&self) -> () {

    }
}
