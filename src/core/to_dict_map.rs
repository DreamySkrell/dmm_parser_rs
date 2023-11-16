use dmmtools::dmm::{self, Coord2};

fn coord3_to_index(coord: dmm::Coord3, size: dmm::Coord3) -> (usize, usize, usize) {
    (
        coord.z as usize - 1,
        (size.y - coord.y) as usize,
        coord.x as usize - 1,
    )
}

pub fn to_dict_map(grid_map: &crate::GridMap) -> dmm::Map {
    let mut dict_map = dmm::Map::new(
        grid_map.size.x as usize,
        grid_map.size.y as usize,
        grid_map.size.z as usize,
        "".to_string(),
        "".to_string(),
    );
    dict_map.dictionary.clear();

    // let mut free_dict_keys = {};

    for x in 1..(grid_map.size.x + 1) {
        for y in 1..(grid_map.size.y + 1) {
            let coord = Coord2::new(x, y);
            if let Some(tile) = grid_map.grid.get(&coord) {
                dict_map
                    .dictionary
                    .insert(tile.key_suggestion, tile.prefabs.clone());
                dict_map.grid[coord3_to_index(coord.z(1), grid_map.size)] =
                    tile.key_suggestion.clone();
            } else {
                panic!();
            }
        }
    }

    dict_map.adjust_key_length();
    dict_map
}
