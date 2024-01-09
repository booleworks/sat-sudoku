use std::time::Instant;

use logicng::{
    datastructures::Model,
    formulas::{FormulaFactory, ToFormula, Variable},
    propositions::Proposition,
    solver::{
        functions::enumerate_models,
        minisat::{sat::Tristate, MiniSat, MiniSatConfig},
    },
};

use crate::grid::Grid;

pub fn solve_grid(grid: &mut Grid) {
    println!("Input grid:");
    println!("{}", grid);
    let f = FormulaFactory::new();

    // translate the grid into Boolean formulas
    let start = Instant::now();
    let translation = translate_to_sat_encoding(&f, grid);
    let duration = start.elapsed();
    println!("Computed SAT encoding in {:?}", duration);

    // construct a new SAT solver and fill it with the translated constraints
    let mut solver = MiniSat::new_with_config(MiniSatConfig::default().proof_generation(true));
    solver.add_propositions(&translation.iter().collect::<Vec<_>>(), &f);

    // solve the formulas
    let start = Instant::now();
    let sat = solver.sat();
    let duration = start.elapsed();
    println!("Computed solution in {:?}", duration);

    if sat == Tristate::False {
        // non solvable Sudoku
        println!("Sudoku grid has no solution!");
        for prop in solver.unsat_core(&f).propositions {
            println!("  {}", prop.description.unwrap());
        }
    } else {
        // solvable Sudoku
        handle_sat_sudoku(&f, &mut solver, grid);
    }
}

fn handle_sat_sudoku(f: &FormulaFactory, solver: &mut MiniSat, grid: &mut Grid) {
    let model = solver.model(None).unwrap();
    fill_grid_with_solution(f, &model, grid);
    let start = Instant::now();
    let all_models = enumerate_models(solver);
    let duration = start.elapsed();
    if all_models.len() == 1 {
        // Sudoku has exactly one solution
        println!("\nSudoku has exactly one solution -> valid Sudoku!");
        println!("\nSolved grid:");
        println!("{}", grid);
    } else {
        // More than one solution -> invalid Sudoku
        println!(
            "Sudoku has {} solutions (computed in {:?}) -> invalid Sudoku!",
            all_models.len(),
            duration
        );
    }
}

fn translate_to_sat_encoding(f: &FormulaFactory, grid: &Grid) -> Vec<Proposition> {
    let mut props = Vec::new();
    one_number_per_row(f, &mut props, grid);
    one_number_per_col(f, &mut props, grid);
    one_number_per_cell(f, &mut props, grid);
    one_number_per_box(f, &mut props, grid);
    fill_given_digits(f, &mut props, grid);
    println!("Generated encoding with {} constraints.", props.len());
    props
}

fn one_number_per_row(f: &FormulaFactory, props: &mut Vec<Proposition>, grid: &Grid) {
    for row in grid.rows() {
        for value in grid.possible_values() {
            let vars: Vec<_> = row
                .cells()
                .into_iter()
                .map(|(r, c)| var_for_cell(f, r, c, value))
                .collect();
            let proposition = Proposition::new(f.exo(vars))
                .description(&format!("Exactly one {value} in row {}", row.row));
            props.push(proposition);
        }
    }
}

fn one_number_per_col(f: &FormulaFactory, props: &mut Vec<Proposition>, grid: &Grid) {
    for col in grid.cols() {
        for value in grid.possible_values() {
            let vars: Vec<_> = col
                .cells()
                .into_iter()
                .map(|(r, c)| var_for_cell(f, r, c, value))
                .collect();
            let proposition = Proposition::new(f.exo(vars))
                .description(&format!("Exactly one {value} in column {}", col.col));
            props.push(proposition);
        }
    }
}

fn one_number_per_cell(f: &FormulaFactory, props: &mut Vec<Proposition>, grid: &Grid) {
    for row in grid.rows() {
        for (row_index, col_index) in row.cells() {
            let vars: Vec<_> = grid
                .possible_values()
                .into_iter()
                .map(|v| var_for_cell(f, row_index, col_index, v))
                .collect();
            let proposition = Proposition::new(f.exo(vars)).description(&format!(
                "Exactly one value in row {row_index} column {col_index}",
            ));
            props.push(proposition);
        }
    }
}

fn one_number_per_box(f: &FormulaFactory, props: &mut Vec<Proposition>, grid: &Grid) {
    for (num, grid_box) in grid.boxes().into_iter().enumerate() {
        for value in grid.possible_values() {
            let vars: Vec<_> = grid_box
                .cells()
                .into_iter()
                .map(|(r, c)| var_for_cell(f, r, c, value))
                .collect();
            let proposition = Proposition::new(f.exo(vars))
                .description(&format!("Exactly one {value} in box {}", num + 1));
            props.push(proposition);
        }
    }
}

fn fill_given_digits(f: &FormulaFactory, props: &mut Vec<Proposition>, grid: &Grid) {
    for (row, col, value) in grid.get_filled_cells() {
        let proposition = Proposition::new(var_for_cell(f, row, col, value).to_formula(f))
            .description(&format!("Given {value} in row {row} column {col}"));
        props.push(proposition);
    }
}

fn var_for_cell(f: &FormulaFactory, row: u8, col: u8, value: u8) -> Variable {
    f.var(&format!("v_{row}_{col}_{value}"))
}

fn fill_grid_with_solution(f: &FormulaFactory, model: &Model, grid: &mut Grid) {
    for pos in model.pos() {
        let var_name = pos.name(f);
        let decoded: Vec<u8> = var_name
            .split('_')
            .skip(1)
            .map(|t| t.parse().unwrap())
            .collect();
        grid.set_value(decoded[0], decoded[1], decoded[2]);
    }
}
