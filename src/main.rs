use crate::grid::Grid;
use crate::solver::solve_grid;

pub mod grid;
pub mod solver;

fn main() {
    let mut grid = Grid::empty_9x9_grid();
    grid.set_row(1, "- - - - - - - - -");
    grid.set_row(2, "- - 9 8 - - - - 7");
    grid.set_row(3, "- 8 - - 6 - - 5 -");
    grid.set_row(4, "- 5 - - 4 - - 3 -");
    grid.set_row(5, "- - 7 9 - - - - 2");
    grid.set_row(6, "- - - - - - - - -");
    grid.set_row(7, "- - 2 7 - - - - 9");
    grid.set_row(8, "- 4 - - 5 - - 6 -");
    grid.set_row(9, "3 - - - - 6 2 - -");
    solve_grid(&mut grid);
}
