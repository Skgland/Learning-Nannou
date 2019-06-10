use super::level::*;
use std::collections::btree_map::BTreeMap;
//use std::convert::TryFrom;
use serde::{Serialize, Deserialize, Serializer, Deserializer};

/*mod direction_fix {
    use super::*;

    pub const UP_NAME: &'static str = "UP";
    pub const DOWN_NAME: &'static str = "DOWN";
    pub const EAST_NAME: &'static str = "EAST";
    pub const WEST_NAME: &'static str = "WEST";
    pub const NORTH_NAME: &'static str = "NORTH";
    pub const SOUTH_NAME: &'static str = "SOUTH";


    impl Into<&'static str> for Direction {
        fn into(self) -> &'static str {
            match self {
                Direction::DOWN => DOWN_NAME,
                Direction::UP => UP_NAME,
                Direction::WEST => WEST_NAME,
                Direction::EAST => EAST_NAME,
                Direction::NORTH => NORTH_NAME,
                Direction::SOUTH => SOUTH_NAME,
            }
        }
    }


    impl TryFrom<String> for Direction {
        type Error = ();

        //the following noinspect is to not have IntelliJ hightlight the Patterns as it thinks they are wrong,
        // though they are working and the suggested fix isn't
        //noinspection ALL
        fn try_from(value: String) -> Result<Self, Self::Error> {
            use self::direction_fix as Dirname;

            let value = match value.as_str() {
                Dirname::DOWN_NAME => Direction::DOWN,
                Dirname::UP_NAME => Direction::UP,
                Dirname::SOUTH_NAME => Direction::SOUTH,
                Dirname::NORTH_NAME => Direction::NORTH,
                Dirname::EAST_NAME => Direction::EAST,
                Dirname::WEST_NAME => Direction::WEST,
                _ => return Err(()),
            };
            Ok(value)
        }
    }

    #[test]
    fn test_into_from() {
        for dir in vec![Direction::UP, Direction::DOWN, Direction::SOUTH, Direction::NORTH, Direction::EAST, Direction::WEST] {
            let expected = Ok(dir);
            let into_str: &str = Direction::into(dir);
            let try_from = Direction::try_from(String::from(into_str));
            assert_eq!(expected, try_from);
        }
    }

    impl Serialize for Direction {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
            S: Serializer {
            let value: &str = Self::into(*self);
            value.serialize(serializer)
        }
    }

    use serde::de::Error;

    impl<'de> Deserialize<'de> for Direction {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
            D: Deserializer<'de> {
            let value = String::deserialize(deserializer)?;

            if let Ok(value) = Direction::try_from(value) {
                Ok(value)
            } else {
                Err(D::Error::custom("Invalid enum variant!"))
            }
        }
    }
}*/

mod level_state_fix {
    use super::*;
    use crate::game::level::ObjectCoordinate;


    impl Serialize for LevelState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
            S: Serializer {
            let mut outer_map: BTreeMap<String, BTreeMap<String, TileType>> = BTreeMap::new();

            let omap = &mut outer_map;

            self.tile_map.iter().for_each(|(ObjectCoordinate { x, y }, tile_type)| {
                let mut inner_map = omap.remove(&x.to_string()).unwrap_or_else(BTreeMap::new);
                inner_map.insert(y.to_string(), tile_type.clone());

                omap.insert(x.to_string(), inner_map);
            });

            serializer.collect_map(outer_map)
        }
    }

    impl<'de> Deserialize<'de> for LevelState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
            let input_map: BTreeMap<String, BTreeMap<String,TileType>> = BTreeMap::deserialize(deserializer)?;

            let output_map = input_map.iter().flat_map(|(x,inner_map)| {
                inner_map.iter().flat_map(|(y,value)| {
                    use std::str::FromStr;

                    if let (Ok(x), Ok(y)) = (i64::from_str(x),i64::from_str(y)) {
                        Some((ObjectCoordinate { x, y }, value.clone()))
                    }else {
                        None
                    }

                    }).collect::<Vec<_>>()
            }).collect();

            Ok(LevelState { tile_map: output_map })
        }
    }
}

/*
mod wall_type_fix {
    use serde::Serialize;
    use crate::game::level::WallType;
    use serde::Serializer;

    impl Serialize for WallType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
            S: Serializer {
            unimplemented!()
        }
    }
}

mod tile_type_fix {
    use super::*;
    use serde::ser::SerializeStruct;

    const VARIANT: &'static str = "Variant";
    const CONTENT: &'static str = "Content";

    const PATH: &'static str = "Path";
    const START: &'static str = "Start";
    const LADDER: &'static str = "Ladder";
    const WALL: &'static str = "Wall";
    const CONNECTIONS: &'static str = "Connections";
    const ACTIVE: &'static str = "Active";
    const GOAL: &'static str = "Goal";
    const GATE: &'static str = "Gate";
    const ONE_WAY: &'static str = "OneWay";
    const BUTTON: &'static str = "Button";

    #[derive(Serialize, Deserialize)]
    struct TileWall {
        wall_type: WallType
    }

    #[derive(Serialize, Deserialize)]
    struct TileGate {
        open: bool,
        facing: Direction,
        hidden: GateVisibility,
    }


    #[derive(Serialize, Deserialize)]
    struct TileOneWay {
        inverted: bool,
        facing: Direction,
    }

    #[derive(Serialize, Deserialize)]
    struct TileButton { pressed: bool, inverted: bool, target: ObjectCoordinate }

    impl Serialize for TileType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
            S: Serializer {
            let mut s = serializer.serialize_struct("TileTuple", 2)?;

            match self {
                TileType::Wall{kind} => {
                    s.serialize_field(VARIANT, WALL)?;
                    s.serialize_field(CONTENT, &TileWall { wall_type: *kind })?;
                }
                TileType::Path => {
                    s.serialize_field(VARIANT, PATH)?;
                    s.skip_field(CONTENT)?;
                }
                TileType::Ladder => {
                    s.serialize_field(VARIANT, LADDER)?;
                    s.skip_field(CONTENT)?;
                }
                TileType::Start => {
                    s.serialize_field(VARIANT, START)?;
                    s.skip_field(CONTENT)?;
                }
                TileType::Goal { active: bool } => {
                    s.serialize_field(VARIANT, GOAL)?;
                    s.serialize_field(CONTENT, bool)?;
                }
                TileType::Gate { open, facing, hidden } => {
                    s.serialize_field(VARIANT, GATE)?;
                    s.serialize_field(CONTENT, &TileGate {
                        open: *open,
                        facing: *facing,
                        hidden: hidden.clone(),
                    })?;
                },
                TileType::OneWay { inverted, facing } => {
                    s.serialize_field(VARIANT, ONE_WAY)?;
                    s.serialize_field(CONTENT, &TileOneWay { inverted: *inverted, facing: *facing })?;
                },
                TileType::Button {pressed,inverted,target} => {
                    s.serialize_field(VARIANT, BUTTON)?;
                    s.serialize_field(CONTENT, &TileButton{
                        pressed: *pressed,
                        inverted: *inverted,
                        target: *target
                    })?;
                },
            }

            s.end()
        }
    }

    impl<'de> Deserialize<'de> for TileType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
            D: Deserializer<'de> {

            let visitor = unimplemented!();
            let mut d = deserializer.deserialize_struct("TileTuple",&[VARIANT,CONTENT],  visitor );



        }
    }
}*/
