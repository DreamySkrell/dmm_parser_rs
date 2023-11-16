use dmmtools::dmm::{self, Coord2};
use fxhash::FxHashMap;
use std::collections::BTreeSet;

fn coord3_to_index(coord: dmm::Coord3, size: dmm::Coord3) -> (usize, usize, usize) {
    (
        coord.z as usize - 1,
        (size.y - coord.y) as usize,
        coord.x as usize - 1,
    )
}

fn int_to_key(i: u16) -> dmm::Key {
    // because `dmm::Key` interior var is private
    unsafe { std::mem::transmute::<u16, dmm::Key>(i) }
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

    let mut used_dict_keys = BTreeSet::<dmm::Key>::new();

    let mut dictionary_reverse = FxHashMap::<Vec<dmm::Prefab>, dmm::Key>::default();

    for tile in grid_map.grid.values() {
        if !dictionary_reverse.contains_key(&tile.prefabs) {
            if used_dict_keys.contains(&tile.key_suggestion) {
                let next_free_key = (0..65534)
                    .map(int_to_key)
                    .filter(|k| !used_dict_keys.contains(k))
                    .next()
                    .unwrap();
                dictionary_reverse.insert(tile.prefabs.clone(), next_free_key);
                used_dict_keys.insert(next_free_key);
            } else {
                dictionary_reverse.insert(tile.prefabs.clone(), tile.key_suggestion.clone());
                used_dict_keys.insert(tile.key_suggestion.clone());
            }
        }
    }

    for x in 1..(grid_map.size.x + 1) {
        for y in 1..(grid_map.size.y + 1) {
            let coord = Coord2::new(x, y);
            if let Some(tile) = grid_map.grid.get(&coord) {
                let key = dictionary_reverse.get(&tile.prefabs).unwrap().clone();
                dict_map.dictionary.insert(key, tile.prefabs.clone());
                dict_map.grid[coord3_to_index(coord.z(1), grid_map.size)] = key;
            } else {
                panic!();
            }
        }
    }

    dict_map.set_key_length(3);
    dict_map
}
