use dmmtools::dmm;
use itertools::Itertools;

#[test]
fn sanity() {
    let paths = std::fs::read_dir("D:/Git/dmm_parser_rs/src/test").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        println!("path: {}", path.display());

        let map = dmmtools::dmm::Map::from_file(&path).unwrap();
        let map_str_parsed = crate::core::map_to_string(&map).unwrap();
        let map_str_original = std::fs::read_to_string(path).unwrap();

        for (i, diff) in diff::lines(&map_str_original, &map_str_parsed)
            .iter()
            .enumerate()
        {
            match diff {
                diff::Result::Left(l) => println!("{} diff - : {}", i, l),
                diff::Result::Both(l, r) => {
                    assert_eq!(l, r);
                }
                diff::Result::Right(r) => println!("{} diff + : {}", i, r),
            }
        }
        if map_str_original != map_str_parsed {
            assert!(false);
        }
    }
}

#[test]
fn grid_check() {
    let path = std::path::Path::new("D:/Git/dmm_parser_rs/src/test/_tiny_test_map.dmm");
    println!("path: {}", path.display());

    let dict_map_original = dmmtools::dmm::Map::from_file(&path).unwrap();
    let grid_map = crate::core::to_grid_map(&dict_map_original);
    assert!(grid_map.grid[&dmm::Coord2::new(2, 1)]
        .prefabs
        .iter()
        .any(|p| p.path == "/obj/random/firstaid"));
    assert!(grid_map.grid[&dmm::Coord2::new(1, 2)]
        .prefabs
        .iter()
        .any(|p| p.path == "/obj/random/finances"));
    assert!(grid_map.grid[&dmm::Coord2::new(14, 15)]
        .prefabs
        .iter()
        .any(|p| p.path == "/obj/random/handgun"));
    assert!(grid_map.grid[&dmm::Coord2::new(15, 14)]
        .prefabs
        .iter()
        .any(|p| p.path == "/obj/random/handgun"));
}

#[test]
fn to_grid_and_back() {
    let paths = std::fs::read_dir("D:/Git/dmm_parser_rs/src/test").unwrap();
    for path in paths.map(|r| r.unwrap().path()).sorted() {
        println!("path: {}", path.display());

        let dict_map_original = dmmtools::dmm::Map::from_file(&path).unwrap();
        let grid_map = crate::core::to_grid_map(&dict_map_original);
        let dict_map_again = crate::core::to_dict_map(&grid_map);
        let map_str_original = crate::core::map_to_string(&dict_map_original).unwrap();
        let map_str_from_grid = crate::core::map_to_string(&dict_map_again).unwrap();

        dict_map_again
            .to_file(
                &std::path::Path::new("D:/Git/dmm_parser_rs/src/test-out")
                    .join(path.file_name().unwrap()),
            )
            .unwrap();

        for (i, diff) in diff::lines(&map_str_original, &map_str_from_grid)
            .iter()
            .enumerate()
        {
            match diff {
                diff::Result::Left(l) => println!("{} diff - : {}", i, l),
                diff::Result::Both(l, r) => {
                    assert_eq!(l, r);
                }
                diff::Result::Right(r) => println!("{} diff + : {}", i, r),
            }
        }
        if map_str_original != map_str_from_grid {
            assert!(false);
        }
    }
}
