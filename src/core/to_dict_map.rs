use std::collections::{BTreeMap, BTreeSet, HashMap};

use dmmtools::dmm::{self, Coord2};

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
    let mut grid_map = grid_map.clone();

    let mut dict_map = dmm::Map::new(
        grid_map.size.x as usize,
        grid_map.size.y as usize,
        grid_map.size.z as usize,
        "".to_string(),
        "".to_string(),
    );
    dict_map.dictionary.clear();

    let mut used_dict_keys = BTreeSet::<dmm::Key>::new();

    let mut dictionary_reverse = HashMap::<Vec<dmm::Prefab>, dmm::Key>::new();
    for tile in grid_map.grid.values_mut() {
        if dictionary_reverse.contains_key(&tile.prefabs) {
            tile.key_suggestion = dictionary_reverse.get(&tile.prefabs).unwrap().clone();
        } else {
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
                // if dict_map.dictionary.contains_key(&tile.key_suggestion) {
                //     if *dict_map.dictionary.get(&tile.key_suggestion).unwrap() == tile.prefabs {}
                // } else {
                //     dict_map
                //         .dictionary
                //         .insert(tile.key_suggestion, tile.prefabs.clone());
                //     dict_map.grid[coord3_to_index(coord.z(1), grid_map.size)] =
                //         tile.key_suggestion.clone();
                // }
                let key = dictionary_reverse.get(&tile.prefabs).unwrap().clone();
                dict_map.dictionary.insert(key, tile.prefabs.clone());
                dict_map.grid[coord3_to_index(coord.z(1), grid_map.size)] = key;
            } else {
                panic!();
            }
        }
    }

    dict_map.adjust_key_length();
    dict_map
}
