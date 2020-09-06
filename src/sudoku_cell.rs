use super::sudoku_values::SudokuValues;
use std::fmt;

#[derive(Clone)]
pub struct SudokuCell {
    pub cur_val: Option<usize>,
    pub row: usize,
    pub col: usize,
    pub subgrid: usize,
    pub possible_vals: SudokuValues,
}

impl SudokuCell {
    pub fn new(r: usize, c: usize, s: usize) -> Self {
        SudokuCell {
            cur_val: None,
            row: r,
            col: c,
            subgrid: s,
            possible_vals: SudokuValues { values: [true; 9] },
        }
    }
}

impl fmt::Debug for SudokuCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return if let Some(v) = self.cur_val {
            write!(
                f,
                "Row: {} Column: {} Subgrid: {} Value: {}",
                self.row + 1,
                self.col + 1,
                self.subgrid + 1,
                v
            )
        } else {
            write!(
                f,
                "Row: {} Column: {} Subgrid: {} Possible Values: {}",
                self.row + 1,
                self.col + 1,
                self.subgrid + 1,
                self.possible_vals
            )
        }
    }
}
