use dmmtools::dmm::{self};

fn coord3_to_tuple(coord: dmm::Coord3) -> (usize, usize, usize) {
    (coord.x as usize, coord.y as usize, coord.z as usize)
}

pub fn to_dict_map(grid_map: &crate::GridMap) -> dmm::Map {
    dbg!("fuck");
    let mut dict_map = dmm::Map::new(
        grid_map.size.x as usize,
        grid_map.size.y as usize,
        grid_map.size.z as usize,
        "".to_string(),
        "".to_string(),
    );
    dict_map.dictionary.clear();
    dbg!("fuck");

    for (coord, tile) in &grid_map.grid {
        dict_map
            .dictionary
            .insert(tile.key_suggestion, tile.prefabs.clone());
        dict_map.grid[coord3_to_tuple(coord.z(1))] = tile.key_suggestion.clone();
    }

    dict_map.adjust_key_length();
    dict_map
}
