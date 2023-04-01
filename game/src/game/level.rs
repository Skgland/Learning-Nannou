use nannou::Draw;

use derive_macros::*;
use derive_macros_helpers::*;

use crate::game::{GameState, TILE_SIZE};
use learning_conrod_core::gui::TextureMap;
use log::{error, trace};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LevelTemplate {
    pub name: String,
    pub init_state: LevelState,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LevelState {
    pub tile_map: BTreeMap<ObjectCoordinate, TileType>,
}

#[derive(
    Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, Deserialize, Bounded, Enumerable,
)]
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

#[derive(
    Debug, Ord, PartialOrd, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Bounded, Enumerable,
)]
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

#[derive(
    Debug, Ord, PartialOrd, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Bounded, Enumerable,
)]
pub enum EastWestAxis {
    East,
    West,
}

impl EastWestAxis {
    pub fn file_modifier(self) -> &'static str {
        match self {
            EastWestAxis::East => "right",
            EastWestAxis::West => "left",
        }
    }
}

#[derive(
    Debug, Ord, PartialOrd, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Bounded, Enumerable,
)]
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

#[derive(
    Debug, Ord, PartialOrd, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Bounded, Enumerable,
)]
pub enum WallType {
    Single {
        facing: Direction,
    },
    Double {
        orientation: Orientation,
    },
    Corner {
        north_south_facing: NorthSouthAxis,
        east_west_facing: EastWestAxis,
    },
    //primary and secondary facing should be different
    End {
        facing: Direction,
    },
    Lone,
    Center,
}

impl WallType {
    pub fn file_modifier(self) -> String {
        match self {
            WallType::Lone => "rock".to_string(),
            WallType::Center => "center".to_string(),
            WallType::Single { facing } => format!("single_{}", facing.file_modifier()),
            WallType::Double { orientation } => format!("double_{}", orientation.file_modifier()),
            WallType::Corner {
                north_south_facing,
                east_west_facing,
            } => format!(
                "{}{}_{}",
                if false { "inner_" } else { "" },
                north_south_facing.file_modifier(),
                east_west_facing.file_modifier()
            ),
            WallType::End { facing } => format!("end_{}", facing.file_modifier()),
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Bounded, Enumerable)]
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
    Wall {
        kind: WallType,
    },
    Path,
    Ladder,
    Start,
    Goal {
        active: bool,
    },
    Gate {
        open: bool,
        facing: Direction,
        hidden: GateVisibility,
    },
    OneWay {
        inverted: bool,
        facing: Direction,
    },
    Button {
        pressed: bool,
        inverted: bool,
        target: ObjectCoordinate,
    },
}

impl TileType {
    pub fn apply_button(&mut self, active: bool) {
        match self {
            TileType::Goal {
                active: active_goal,
            } => *active_goal = active,
            TileType::Gate { open, .. } => *open = active,
            TileType::OneWay { inverted, .. } => *inverted = active,
            _ => error!(
                "Tried to change the state of a single State Tile or Button Tile with a Button!"
            ),
        }
    }

    pub fn step_on(&mut self) -> Option<Box<dyn Fn(&mut GameState)>> {
        match self {
            TileType::Goal { active: true } => {
                trace!("Goal reached!");
                Some(Box::new(|game| {
                    if let GameState::GameState { level_template, .. } = game {
                        *game = GameState::Won {
                            level_template: level_template.clone(),
                        }
                    }
                }))
            }
            TileType::Button {
                pressed,
                inverted,
                target,
            } => {
                trace!("Stepping on a Button");
                *pressed = !*pressed;
                let power = *pressed ^ *inverted;
                let t = *target;
                Some(Box::new(move |game: &mut GameState| {
                    if let GameState::GameState { level_state, .. } = game {
                        if let Some(tile) = level_state.tile_map.get_mut(&t) {
                            tile.apply_button(power)
                        }
                    }
                }))
            }
            _ => None,
        }
    }

    pub fn draw_tile(
        &self,
        draw: &Draw,
        texture_map: &TextureMap<TileTextureIndex>,
        coord: &ObjectCoordinate,
        state: &GameState,
    ) {
        if let GameState::GameState { position, .. } = state {
            let x = (coord.x as f32) * TILE_SIZE - position.x * 64.0 - TILE_SIZE / 2.0;
            let y = (-coord.y as f32) * TILE_SIZE + position.y * 64.0 - TILE_SIZE / 2.0;

            if let Some(texture) = texture_map.get(&self.tile_texture_id()) {
                draw.texture(texture).x_y(x, y).w_h(TILE_SIZE, TILE_SIZE);
            } else {
                draw.rect()
                    .x_y(x, y)
                    .w_h(TILE_SIZE, TILE_SIZE)
                    .color(nannou::color::named::RED);
            }
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
            TileType::OneWay {
                facing,
                inverted: false,
            } => TileTextureIndex::OneWay { facing: *facing },
            TileType::OneWay {
                facing,
                inverted: true,
            } => TileTextureIndex::OneWay {
                facing: facing.inverted(),
            },
            TileType::Wall { kind } => TileTextureIndex::Wall { kind: *kind },
            TileType::Gate {
                open,
                facing,
                hidden: GateVisibility::Visible,
            }
            | TileType::Gate {
                open: open @ true,
                facing,
                ..
            } => TileTextureIndex::Gate {
                open: *open,
                facing: *facing,
            },
            TileType::Gate {
                open: false,
                facing: _,
                hidden: GateVisibility::Hidden(mimic),
            } => mimic.tile_texture_id(),
        }
    }
}

impl TileTextureIndex {
    pub fn file_name(&self) -> String {
        match self {
            TileTextureIndex::Path => "path".to_string(),
            TileTextureIndex::Start => "start".to_string(),
            TileTextureIndex::Goal { active } => {
                format!("goal{}", if !active { "_inactive" } else { "" })
            }
            // we should never need a texture for a hidden and closed gate because it is hidden
            TileTextureIndex::Gate { open, facing } => format!(
                "{}_gate_{}",
                if *open { "open" } else { "closed" },
                facing.file_modifier()
            ),
            TileTextureIndex::Ladder => "ladder".to_string(),
            TileTextureIndex::OneWay { facing } => format!("one_way_{}", facing.file_modifier()),
            TileTextureIndex::Wall { kind } => format!("wall_{}", kind.file_modifier()),
            TileTextureIndex::Button { pressed } => {
                format!("button{}", if *pressed { "_pressed" } else { "" })
            }
        }
    }
}

impl ToString for TileTextureIndex {
    fn to_string(&self) -> String {
        self.file_name()
    }
}

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub struct ObjectCoordinate {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Connections {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GateVisibility {
    Visible,
    Hidden(Box<TileType>),
}

pub mod saving {
    use crate::game::LevelTemplate;
    use std::fmt::{Display, Formatter};
    use std::fs::File;
    use std::io::Write;

    use log::info;

    pub enum SavingError {
        IO(std::io::Error),
        Serialize(ron::Error),
    }

    impl Display for SavingError {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            match self {
                SavingError::IO(err) => Display::fmt(err, f),
                SavingError::Serialize(err) => Display::fmt(err, f),
            }
        }
    }

    impl From<std::io::Error> for SavingError {
        fn from(io_err: std::io::Error) -> Self {
            SavingError::IO(io_err)
        }
    }

    impl From<ron::Error> for SavingError {
        fn from(ser_err: ron::Error) -> Self {
            SavingError::Serialize(ser_err)
        }
    }

    pub(crate) fn save_level(
        path: &std::path::Path,
        level: &LevelTemplate,
    ) -> Result<(), SavingError> {
        let pretty = ron::ser::PrettyConfig::default()
            .depth_limit(!0)
            .new_line("\n".into())
            .indentor("\t".into())
            .separate_tuple_members(false)
            .enumerate_arrays(false);

        let out = ron::ser::to_string_pretty(level, pretty)?;

        if let Some(parent) = path.parent() {
            //path does not exist try to create it
            if !parent.exists() {
                std::fs::create_dir_all(parent)?
            }
        }

        let mut file = File::create(path)?;

        info!("Writing level {} to {:?}.", level.name, path);

        file.write_all(out.as_bytes())?;
        Ok(())
    }
}

pub mod loading {
    use crate::game::LevelTemplate;
    use std::fs::File;
    use std::io::Read;

    pub enum LoadingError {
        IO(std::io::Error),
        Deserialize(ron::de::Error),
        Spanned(ron::error::SpannedError),
    }

    impl From<std::io::Error> for LoadingError {
        fn from(io_err: std::io::Error) -> Self {
            LoadingError::IO(io_err)
        }
    }

    impl From<ron::de::Error> for LoadingError {
        fn from(de_err: ron::de::Error) -> Self {
            LoadingError::Deserialize(de_err)
        }
    }

    impl From<ron::error::SpannedError> for LoadingError {
        fn from(de_err: ron::error::SpannedError) -> Self {
            LoadingError::Spanned(de_err)
        }
    }

    pub fn load_levels(asset_path: &std::path::Path) -> Result<Vec<LevelTemplate>, LoadingError> {
        log::info!("Loading Levels!");
        let path = asset_path.join("levels");
        let mut levels = vec![];

        if !path.exists() {
            //path does not exist try to create it
            std::fs::create_dir_all(&path)?;
        }

        let dir = path.read_dir()?;

        for entry in dir.flatten() {
            if let Ok(f_type) = entry.file_type() {
                if f_type.is_file() {
                    if let Ok(level) = load_level(entry.path().as_path()) {
                        levels.push(level);
                    }
                }
            }
        }
        log::info!("Loaded {} levels!", levels.len());
        Ok(levels)
    }

    fn load_level(path: &std::path::Path) -> Result<LevelTemplate, LoadingError> {
        log::info!("Loading level at '{}'!", path.display());
        let mut content = vec![];

        use serde::Deserialize;

        File::open(path)?.read_to_end(&mut content)?;

        let mut des = ron::de::Deserializer::from_bytes(content.as_slice())?;

        let level = LevelTemplate::deserialize(&mut des)?;
        Ok(level)
    }
}
