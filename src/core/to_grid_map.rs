use super::Tile;
use dmmtools::dmm::{self, Coord3};
use std::ops::Index;

fn tuple_to_coord3(xyz: (usize, usize, usize)) -> Coord3 {
    Coord3::new(xyz.0 as i32, xyz.1 as i32, xyz.2 as i32)
}

pub fn to_grid_map(dict_map: &dmm::Map) -> crate::GridMap {
    let mut grid_map = crate::GridMap {
        grid: Default::default(),
        size: tuple_to_coord3(dict_map.dim_xyz()),
    };

    for x in 1..255 {
        for y in 1..255 {
            let coord = dmm::Coord2::new(x, y);
            let key = dict_map.index(coord.z(1));
            let prefabs = dict_map.dictionary[key].clone();
            let tile = Tile {
                key_suggestion: *key,
                prefabs,
            };
            grid_map.grid.insert(coord, tile);
        }
    }

    grid_map
}
