use crate::sudoku_table::{SudokuTable, EMPTY_NUM};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone)]
struct ParseError;
impl std::error::Error for ParseError {}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ParseError: invalid syntax")
    }
}

#[derive(Debug, Clone)]
struct NoSolutionError;
impl std::error::Error for NoSolutionError {}
impl std::fmt::Display for NoSolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No solution")
    }
}

fn gen_candidates() -> Vec<u32> {
    let mut ret: Vec<u32> = (1..=9).collect();
    let mut rng = rand::thread_rng();
    use rand::seq::SliceRandom;
    ret.shuffle(&mut rng);
    return ret;
}

#[derive(Default)]
pub struct RecursiveSolver {
    m_table: SudokuTable,
    m_rows_indexes: Vec<usize>,
}

impl RecursiveSolver {
    pub fn solve_str(&mut self, s: &str) -> Result<SudokuTable> {
        let mut t = SudokuTable::default();
        if !t.try_parse(s) {
            return Err(Box::new(ParseError));
        }
        self.set_table(t);
        return self.solve();
    }

    pub fn set_table(&mut self, t: SudokuTable) {
        self.m_table = t;
        self.m_rows_indexes = self.m_table.get_sorted_rows_indexes();
    }

    pub fn solve(&mut self) -> Result<SudokuTable> {
        if !self.m_table.verify_all() {
            return Err(Box::new(NoSolutionError));
        }
        if self.solve_impl(self.m_rows_indexes[0] * 9) {
            return Ok(self.m_table.clone());
        } else {
            return Err(Box::new(NoSolutionError));
        }
    }

    fn solve_impl(&mut self, start_cell: usize) -> bool {
        let empty_cell = self
            .m_table
            .get_empty_cell(start_cell, &self.m_rows_indexes);
        if empty_cell.is_none() {
            return true;
        }
        let empty_cell = empty_cell.unwrap();
        for val in gen_candidates() {
            if self.m_table.try_set(empty_cell, val) {
                if self.solve_impl(empty_cell) {
                    return true;
                } else {
                    self.m_table[empty_cell] = EMPTY_NUM;
                }
            }
        }
        return false;
    }
}
