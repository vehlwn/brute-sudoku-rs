mod recursive_solver;
mod sudoku_table;

use recursive_solver::RecursiveSolver;
use sudoku_table::{Format, SudokuTable};

const EXAMPLE1: &str = "
    . . .  9 . .  . 3 .
    3 . 6  . 2 .  . 4 .
    2 . 4  . . 3  1 . 6

    . 7 .  . 5 1  . 8 .
    . 3 1  . 6 .  . 5 7
    5 . 9  . . .  6 . .

    4 1 .  . . 2  . 7 8
    7 6 3  . . 5  4 . .
    9 2 8  . . 4  . . 1
    ";
const EXAMPLE2: &str = "
    . . .  . . .  . . .
    . . .  . . 3  . 8 5
    . . 1  2 . .  . . .

    . . .  5 . 7  . . .
    . . 4  . . .  1 . .
    . 9 .  . . .  . . .

    5 . .  . . .  . 7 3
    . . 2  . 1 .  . . .
    . . .  . 4 .  . . 9
    ";
fn main() {
    let mut t: SudokuTable = Default::default();
    assert!(t.try_parse(EXAMPLE2));
    println!("{}", t.to_string(Format::Indent));
    let mut s = RecursiveSolver::default();
    s.set_table(t);
    let result = s.solve().unwrap();
    println!("{}", result.to_string(Format::Indent));
}
