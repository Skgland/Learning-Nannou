use super::*;
use crate::TextureMap;
use derive_macros::*;
use derive_macros_helpers::*;


#[derive(Clone, Serialize, Deserialize,Debug)]
pub struct LevelTemplate {
    pub name: String,
    pub init_state: LevelState,
}


#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct LevelState {
    pub tile_map: BTreeMap<ObjectCoordinate, TileType>
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize,Bounded,Enumerable)]
pub enum Direction {
    UP,
    DOWN,
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Direction {
    pub fn inverted(self) -> Self {
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

    pub fn file_modifier(self) -> &'static str {
        match self {
            Direction::UP => "lower",
            Direction::DOWN => "upper",
            Direction::WEST => "right",
            Direction::EAST => "left",
            Direction::NORTH => "bottom",
            Direction::SOUTH => "top",
        }
    }
}

#[derive(Debug,Ord, PartialOrd, PartialEq, Eq, Clone, Copy, Serialize, Deserialize,Bounded,Enumerable)]
pub enum NorthSouthAxis {
    North,
    South,
}

impl NorthSouthAxis {
    pub fn file_modifier(self) -> &'static str {
        match self {
            NorthSouthAxis::North => "bottom",
            NorthSouthAxis::South => "top",
        }
    }
}

#[derive(Debug,Ord, PartialOrd, PartialEq, Eq, Clone, Copy, Serialize, Deserialize,Bounded,Enumerable)]
pub enum EastWestAxis {
    East,
    West,
}

impl EastWestAxis {
    pub fn file_modifier(self) -> &'static str {
        match self {
            EastWestAxis::East => "right",
            EastWestAxis::West => "left"
        }
    }
}

#[derive(Debug,Ord, PartialOrd, PartialEq, Eq, Clone, Copy, Serialize, Deserialize,Bounded,Enumerable)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    pub fn file_modifier(self) -> &'static str {
        match self {
            Orientation::Horizontal => "horizontal",
            Orientation::Vertical => "vertical",
        }
    }
}

#[derive(Debug,Ord, PartialOrd, PartialEq, Eq, Clone, Copy, Serialize, Deserialize,Bounded,Enumerable)]
pub enum WallType {
    Single { facing: Direction },
    Double { orientation: Orientation },
    Corner { north_south_facing: NorthSouthAxis, east_west_facing: EastWestAxis },
    //primary and secondary facing should be different
    End { facing: Direction },
    Lone,
    Center,
}

impl WallType {
    pub fn file_modifier(self) -> String {
        match self {
            WallType::Lone => "rock".to_string(),
            WallType::Center => "center".to_string(),
            WallType::Single { facing } => format!("single_{}",facing.file_modifier()),
            WallType::Double { orientation } => format!("double_{}",orientation.file_modifier()),
            WallType::Corner { north_south_facing, east_west_facing } => { format!("{}{}_{}", if false { "inner_" } else { "" }, north_south_facing.file_modifier(), east_west_facing.file_modifier()) }
            WallType::End { facing } => {format!("end_{}",facing.file_modifier())}
        }
    }
}


#[derive(Debug,Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Bounded,Enumerable)]
pub enum TileTextureIndex {
    Wall { kind: WallType },
    Path,
    Ladder,
    Start,
    Goal { active: bool },
    Gate { open: bool, facing: Direction },
    OneWay { facing: Direction },
    Button { pressed: bool },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileType {
    Wall { kind: WallType },
    Path,
    Ladder,
    Start,
    Goal { active: bool },
    Gate { open: bool, facing: Direction, hidden: GateVisibility },
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

    pub fn step_on(&mut self) -> Option<Box<dyn Fn(&mut GameState) -> ()>> {
        match self {
            TileType::Goal { active: true } => {
                println!("Goal reached!");
                None
            }
            TileType::Button { pressed, inverted, target } => {
                println!("Stepping on a Button");
                *pressed = !*pressed;
                let power = *pressed ^ *inverted;
                let t = *target;
                Some(Box::new(move |game: &mut GameState| {
                    if let Some(tile) = game.level_state.tile_map.get_mut(&t) {
                        tile.apply_button(power)
                    }
                }))
            }
            _ => { None }
        }
    }

    pub fn draw_tile<G: Graphics>(&self, context: Context, gl: &mut G, texture_map: &TextureMap<G>, coord: &ObjectCoordinate, state: &GameState) where G::Texture: ImageSize {
        use graphics::*;

        use self::color::*;

        let rect = [0.0, 0.0, TILE_SIZE, TILE_SIZE];

        let adjusted = context.trans((coord.x as f64) * TILE_SIZE - state.position.x - TILE_SIZE / 2.0,
                                     (coord.y as f64) * TILE_SIZE - state.position.y - TILE_SIZE / 2.0);

        if let Some(texture) = texture_map.get(&self.tile_texture_id()) {
            let transform = adjusted.scale(TILE_SIZE / f64::from(texture.get_width()), TILE_SIZE / f64::from(texture.get_height())).transform;
            image(texture, transform, gl)
        } else {
            rectangle(D_RED, rect, adjusted.transform, gl)
        }
    }

    //TODO directionality for OneWay
    pub fn is_solid(&self) -> bool {
        match self {
            TileType::Wall { .. } => true,
            TileType::Button { .. } => false,
            TileType::Path => false,
            TileType::Start => false,
            TileType::Goal { .. } => false,
            TileType::Gate { open, .. } => !open,
            TileType::OneWay { .. } => false,
            TileType::Ladder => false,
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
            TileType::Wall { kind } => TileTextureIndex::Wall { kind: *kind },
            TileType::Gate { open, facing, hidden: GateVisibility::Visible } |
            TileType::Gate { open: open @ true, facing, .. } => TileTextureIndex::Gate { open: *open, facing: *facing },
            TileType::Gate { open: false, facing, hidden: GateVisibility::Hidden(mimic) } => mimic.tile_texture_id(),
        }
    }
}

impl TileTextureIndex {
    pub fn file_name(&self) -> String {
        match self {
            TileTextureIndex::Path => "path".to_string(),
            TileTextureIndex::Start => "start".to_string(),
            TileTextureIndex::Goal { active } => format!("goal{}", if !active { "_inactive" } else { "" }),
            // we should never need a texture for a hidden and closed gate because it is hidden
            TileTextureIndex::Gate { open, facing } => format!("{}_gate_{}", if *open {  "open"  } else { "closed" }, facing.file_modifier()),
            TileTextureIndex::Ladder => "ladder".to_string(),
            TileTextureIndex::OneWay { facing} => format!("one_way{}", facing.file_modifier()),
            TileTextureIndex::Wall { kind } => { format!("wall_{}", kind.file_modifier()) }
            TileTextureIndex::Button { pressed } => { format!("button{}", if *pressed { "_pressed" } else { "" }) }
        }
    }
}

#[derive(Debug,Ord, PartialOrd, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub struct ObjectCoordinate {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug,Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Connections { pub up: bool, pub down: bool, pub left: bool, pub right: bool }


#[derive(Debug,Clone, Serialize, Deserialize)]
pub enum GateVisibility {
    Visible,
    Hidden( Box<level::TileType>),
}
