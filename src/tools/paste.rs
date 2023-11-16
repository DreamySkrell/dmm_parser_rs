// pub fn insert(umm_src: &Umm, umm_dst: &mut Umm, coord: Coord) {
//     let size = Size {
//         x: umm_src.grid.size().0,
//         y: umm_src.grid.size().1,
//     };

//     for x in 0..size.x {
//         for y in 0..size.y {
//             let cell_src = umm_src.grid.get(x, y).unwrap().clone();
//             if let Some(cell_dst) = umm_dst.grid.get_mut(coord.x + x, coord.y + y) {
//                 *cell_dst = cell_src;
//             }
//         }
//     }
// }
