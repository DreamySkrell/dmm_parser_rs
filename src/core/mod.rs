use std::collections::BTreeMap;

pub use dmmtools::dmm;

pub mod to_dict_map;
pub use to_dict_map::to_dict_map;
pub mod to_grid_map;
pub use to_grid_map::to_grid_map;

// pub mod from_grid_map;
// pub use from_grid_map::from_grid_map;

pub mod map_to_string;
pub use map_to_string::map_to_string;

/// Kinda analogous to `dmmtools::dmm::Map`, but instead of being structured like dmm maps are,
/// where they have a dictionary of keys-to-prefabs and a separate grid of keys,
/// this is only a direct coord-prefab grid.
/// It is not memory efficient, but it allows for much greater flexibility of manipulation.
#[derive(Clone, Debug)]
pub struct GridMap {
    ///
    pub size: dmm::Coord3,
    ///
    pub grid: BTreeMap<dmm::Coord2, crate::core::Tile>,
}

#[derive(Clone, Debug)]
pub struct Tile {
    ///
    pub key_suggestion: dmm::Key,
    ///
    pub prefabs: Vec<dmm::Prefab>,
}
