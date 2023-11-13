use grid::Grid;

use crate::dmmr::Umm;

pub struct Coord {
    x: usize,
    y: usize,
}

pub struct Size {
    x: usize,
    y: usize,
}

pub fn extract(umm_src: &Umm, coord: Coord, size: Size) -> Umm {
    let mut umm_dst = Umm {
        comment: umm_src.comment.clone(),
        grid: Grid::new(size.x, size.y),
    };

    for x in 0..size.x {
        for y in 0..size.y {
            if let Some(cell) = umm_src.grid.get(coord.x + x, coord.y + y) {
                *umm_dst.grid.get_mut(x, y).unwrap() = cell.clone();
            }
        }
    }

    umm_dst
}

pub fn insert(umm_src: &Umm, umm_dst: &mut Umm, coord: Coord) {
    let size = Size {
        x: umm_src.grid.size().0,
        y: umm_src.grid.size().1,
    };

    for x in 0..size.x {
        for y in 0..size.y {
            let cell_src = umm_src.grid.get(x, y).unwrap().clone();
            if let Some(cell_dst) = umm_dst.grid.get_mut(coord.x + x, coord.y + y) {
                *cell_dst = cell_src;
            }
        }
    }
}
