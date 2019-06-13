use std::collections::BTreeMap;
use crate::game::level::LevelTemplate;


pub fn test_level() -> LevelTemplate {

    use crate::game::level::*;
    use Orientation::*;
    use WallType::*;
    use NorthSouthAxis::*;
    use EastWestAxis::*;

    let mut tile_map = BTreeMap::new();
    tile_map.insert(ObjectCoordinate { x: 0, y: 0 }, TileType::Start);
    tile_map.insert(ObjectCoordinate { x: 0, y: 1 }, TileType::Path);
    tile_map.insert(ObjectCoordinate { x: 1, y: 1 }, TileType::Path);
    tile_map.insert(ObjectCoordinate { x: 2, y: 1 }, TileType::Path);
    tile_map.insert(ObjectCoordinate { x: 2, y: 2 }, TileType::Path);
    tile_map.insert(ObjectCoordinate { x: 2, y: 3 }, TileType::Path);
    tile_map.insert(ObjectCoordinate { x: 1, y: 3 }, TileType::Path);
    tile_map.insert(ObjectCoordinate { x: 0, y: 2 }, TileType::Wall { kind: Corner { north_south_facing: North, east_west_facing: West } });
    tile_map.insert(ObjectCoordinate { x: 1, y: 2 }, TileType::Wall { kind: Double { orientation: Horizontal } });
    tile_map.insert(ObjectCoordinate { x: -1, y: 2 }, TileType::Wall { kind: Double { orientation: Horizontal } });
    tile_map.insert(ObjectCoordinate { x: 0, y: 3 }, TileType::Goal { active: false });
    tile_map.insert(ObjectCoordinate { x: -1, y: -1 }, TileType::Button { pressed: false, inverted: false, target: ObjectCoordinate { x: 0, y: 3 } });
    LevelTemplate { name: String::from("Test"), init_state: LevelState { tile_map } }
}