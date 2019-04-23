use super::*;
use ::toml_fix::*;


#[derive(Clone, Serialize, Deserialize)]
pub struct LevelTemplate {
    pub name: String,
    pub init_state: LevelState,
}


#[derive(Clone)]
pub struct LevelState {
    pub tile_map: BTreeMap<ObjectCoordinate, TileType>
}


#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq,Debug,TomlFix)]
pub enum Direction {
    UP,
    DOWN,
    NORTH,
    EAST,
    SOUTH,
    WEST,
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

#[derive(Ord, PartialOrd, PartialEq, Eq, Clone, Copy,TomlFix)]
pub enum NorthSouthAxis {
    North,
    South
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Clone, Copy,TomlFix)]
pub enum EastWestAxis {
    East,
    West
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Clone, Copy,TomlFix)]
pub enum Orientation {
    Horizontal,
    Vertical
}

#[derive(Ord,PartialOrd, PartialEq,Eq,Clone,Copy,TomlFix)]
pub enum WallType{
    Single{facing:Direction},
    Wall{orientation:Orientation},
    Corner{north_south_facing:NorthSouthAxis,east_west_facing: EastWestAxis },//primary and secondary facing should be different
    End{facing:Direction},
    Pillar,
}


#[derive(Ord, PartialOrd, Eq, PartialEq,TomlFix)]
pub enum TileTextureIndex {
    TileMap,
    Wall{kind:WallType},
    Path,
    Ladder,
    Start,
    Goal { active: bool },
    Gate { open: bool, facing: Direction },
    OneWay { facing: Direction },
    Button { pressed: bool },
}

#[derive(Clone,TomlFix)]
pub enum TileType {
    Wall{kind:WallType},
    Path,
    Ladder,
    Start,
    Goal { active: bool },
    Gate { open: bool, facing: Direction, #[clone] hidden: GateVisibility },
    OneWay { inverted: bool, facing: Direction },
    Button { pressed: bool, inverted: bool, target: ObjectCoordinate },
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
            TileType::Wall{kind} => TileTextureIndex::Wall{kind:*kind},
            TileType::Gate { open, facing, hidden: GateVisibility::Visible } |
            TileType::Gate { open: open @ true, facing, .. } => TileTextureIndex::Gate { open: *open, facing: *facing },
            TileType::Gate { open: false, facing, hidden: GateVisibility::Hidden(mimic) } => mimic.tile_texture_id(),
        }
    }
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub struct ObjectCoordinate {
    pub x: i64,
    pub y: i64,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Connections { pub up: bool, pub down: bool, pub left: bool, pub right: bool }


#[derive(Clone, TomlFix)]
pub enum GateVisibility {
    Visible,
    Hidden(#[clone] Box<level::TileType>),
}
