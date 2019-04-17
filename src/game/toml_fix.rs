use super::level::*;
use super::ObjectCoordinate;
use std::collections::btree_map::BTreeMap;
use std::convert::TryFrom;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use toml;

mod direction_fix {
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
}

mod level_state_fix {
    use super::*;


    impl Serialize for LevelState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
            S: Serializer {
            let map = self.tile_map.iter().flat_map(
                |(key, value)| {
                    let mut tmp = String::new();
                    let mut tmp_ser = toml::Serializer::new(&mut tmp);
                    if let Ok(()) = key.serialize(&mut tmp_ser) {
                        Some((tmp, value))
                    } else {
                        None
                    }
                });
            serializer.collect_map(map)
        }
    }

    impl<'de> Deserialize<'de> for LevelState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
            let map: BTreeMap<String, TileType> = BTreeMap::deserialize(deserializer)?;

            let map = map.iter().flat_map(
                |(key, value)| {
                    let mut tmp_des = toml::de::Deserializer::new(key.as_str());
                    if let Ok(result) = ObjectCoordinate::deserialize(&mut tmp_des) {
                        Some((result, value.clone()))
                    } else {
                        None
                    }
                }).collect::<BTreeMap<ObjectCoordinate, TileType>>();

            Ok(LevelState { tile_map: map })
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
        connections: Connections,
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
                TileType::Wall(connections) => {
                    s.serialize_field(VARIANT, WALL)?;
                    s.serialize_field(CONTENT, &TileWall { connections: *connections })?;
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
}