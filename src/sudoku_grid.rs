use super::sudoku_cell::SudokuCell;
use super::sudoku_values::SudokuValues;
use std::fmt;
use std::fs::File;
use std::io::BufRead;

#[derive(Clone)]
pub struct SudokuGrid {
    cells: Vec<SudokuCell>,
    rows: Vec<SudokuValues>,
    cols: Vec<SudokuValues>,
    subgrids: Vec<SudokuValues>,
}

impl SudokuGrid {
    pub fn new() -> Self {
        let mut grid = SudokuGrid {
            cells: Vec::with_capacity(81),
            rows: Vec::with_capacity(9),
            cols: Vec::with_capacity(9),
            subgrids: Vec::with_capacity(9),
        };
        for _i in 0..9 {
            grid.rows.push(SudokuValues::new(false));
            grid.cols.push(SudokuValues::new(false));
            grid.subgrids.push(SudokuValues::new(false));
        }

        return grid;
    }

    pub fn solve_grid(mut self) -> Result<Self, String>{
        return if self.solve_grid_helper(false, 0) {
            Ok(self)
        } else {
            Err("Unable to solve this puzzle".to_string())
        }
    }

    /// This is supposed to be the 'main' function for filling out the rest of the grid
    /// There are basically three approaches we use for solving the grid.
    ///
    /// The first is to check each cell's list of possible values, and compare it against the
    /// existing values already entered in that cell's row, column, or subgrid.
    ///
    /// The second is more involved. It takes a cell, and compares it against other unsolved cells
    /// within the same row, column, or subgrid. If a given cell has a possible value that is
    /// impossible in any other cells, then that is the only possible value for that cell
    ///
    /// The third is a brute force approach only used when the first two approaches fail to make any
    /// progress. We clone the grid, find an unsolved cell, set its value to one of its
    /// possibilities, then call 'solve_grid_helper' on that cloned grid. If that value is incorrect,
    /// then 'solve_grid_helper' will return false

    pub fn solve_grid_helper(&mut self, is_guess: bool, clone_num: usize) -> bool {
        let mut complete: bool = false;
        let mut iterations: usize = 0;

        //
        let mut some_change: bool = true;

        while !complete && iterations <= 100 {
            complete = true;
            while some_change {
                some_change = false;
                for i in 0..81 {
                    if let None = self.cells[i].cur_val {
                        let row = self.cells[i].row;
                        let col = self.cells[i].col;
                        let subgrid = self.cells[i].subgrid;

                        match SudokuGrid::calc_possibilities_in_cell(
                            &mut self.cells[i],
                            &mut self.rows[row],
                            &mut self.cols[col],
                            &mut self.subgrids[subgrid],
                        ) {
                            Possibilities::One => {
                                // println!(
                                //     "Found a value! {:?}, clone# {}",
                                //     self.cells[i], clone_num
                                // );
                                some_change = true;
                            }
                            Possibilities::Many => {
                                //println!("Cell# {:?} clone# {}", i, clone_num);
                                complete = false;
                            }
                            Possibilities::None => {
                                // println!("Failure! {:?}, clone# {}", self.cells[i], clone_num);
                                return false;
                            }
                        };

                        //println!("{:?}", self.cells[i]);
                    }
                }
            }

            for i in 0..81 {
                if let None = self.cells[i].cur_val {
                    match self.compare_cell_against_other_cells(i) {
                        Some(v) => {
                            let cur_row = self.cells[i].row;
                            let cur_col = self.cells[i].col;
                            let cur_sub = self.cells[i].subgrid;

                            self.cells[i].cur_val = Some(v);
                            self.rows[cur_row][v - 1] = true;
                            self.cols[cur_col][v - 1] = true;
                            self.subgrids[cur_sub][v - 1] = true;

                            some_change = true;
                            // println!("Found a value! {:?}, clone# {}", self.cells[i], clone_num);

                            break;
                        }
                        _ => {
                            complete = false;
                            continue;
                        }
                    }
                }
            }

            //if the deductive approaches fail to make any progress,
            //we need to brute force this and just try guessing things
            if !some_change && !complete {
                //we don't want to override our current grid just yet.
                let mut clone_grid = Box::new(self.clone());
                match clone_grid.get_next_unsolved_cell_index() {
                    Some(s) => {
                        for i in 0..9 {
                            if clone_grid.cells[s].possible_vals[i] {
                                let cur_row = clone_grid.cells[s].row;
                                let cur_col = clone_grid.cells[s].col;
                                let cur_sub = clone_grid.cells[s].subgrid;

                                clone_grid.rows[cur_row][i] = true;
                                clone_grid.cols[cur_col][i] = true;
                                clone_grid.subgrids[cur_sub][i] = true;

                                clone_grid.cells[s].cur_val = Some(i + 1);
                                // println!(
                                //     "Guessing {} for {:?}, clone#: {}\n{:?}",
                                //     (i + 1),
                                //     clone_grid.cells[s],
                                //     clone_num,
                                //     clone_grid
                                // );

                                if clone_grid.solve_grid_helper(true, clone_num + 1) {
                                    self.copy_over_grid(&clone_grid);
                                    break;
                                } else {
                                    // println!(
                                    //     "Guess {} for {:?} was not correct",
                                    //     i + 1,
                                    //     clone_grid.cells[s],
                                    // );

                                    self.cells[s].possible_vals[i] = false;
                                    clone_grid.copy_over_grid(&self);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            iterations += 1;
        }
        return if complete {
            if !is_guess {
                println!("Solved!");
                println!("{:?}", self);
            }
            true
        } else {
            if !is_guess {
                println!("I can't solve this!");
                println!("{:?}", self);
            }
            false
        };
    }

    ///Every sudoku cell, if it does not already contain a value,
    ///will have a list of potential values, represented by
    ///an array of booleans. On the grid itself, we have arrays of arrays
    ///holding what values are already present for each row, column, or subgrid.
    ///We pass in the cell's list of 'potential' values, then look at the
    ///values present in row/column/subgrid. If a value is present in any of those
    ///groupings, that value is no longer possible within the cell.
    ///Returns true if a cell value is solved
    fn calc_possibilities_in_cell(
        cell: &mut SudokuCell,
        row: &mut SudokuValues,
        col: &mut SudokuValues,
        subgrid: &mut SudokuValues,
    ) -> Possibilities {
        for i in 0..9 {
            //A value can only be possible for the cell if it already exists as a possible value,
            //and it's not present in the cell's row, column or subgrid
            cell.possible_vals[i] = cell.possible_vals[i] && !row[i];
            cell.possible_vals[i] = cell.possible_vals[i] && !col[i];
            cell.possible_vals[i] = cell.possible_vals[i] && !subgrid[i];
        }
        let mut count = 0;
        let mut new_val_index = 0;

        //Check how values are possible for the cell.
        for i in 0..9 {
            if cell.possible_vals[i] {
                count += 1;
                new_val_index = i;
            }
            //If there is more than one, it means this cell's value is still uncertain
            if count > 1 {
                return Possibilities::Many;
            }
        }

        //If we are returning this, it probably means one of the values of another cell is wrong.
        if count == 0 {
            return Possibilities::None;
        }

        //If there is only possibility, set the cell's value to that, and mark it on the cell's row,
        // column, and subgrid
        cell.cur_val = Some(new_val_index + 1);
        row[new_val_index] = true;
        col[new_val_index] = true;
        subgrid[new_val_index] = true;
        return Possibilities::One;
    }

    ///calc_possibilities_in_cell can solve many puzzles on its own,
    ///but in some cases many cells will be left with multiple possibilities.
    ///In that case, we want look at each cell, and then compare it to
    ///every other cell in that cell's row/column/subgrid.
    ///If we find a possibility in a cell that is not possible in any other
    ///cell in the row/column/subgrid, then we return that value
    fn compare_cell_against_other_cells(&self, cell_index: usize) -> Option<usize> {
        //The cell we are checking
        let main_cell = &self.cells[cell_index];
        // println!("{:?}", main_cell);
        let cur_row = main_cell.row;
        let cur_col = main_cell.col;
        let cur_sub = main_cell.subgrid;

        //A counter for each potential value we discover in the other cells.
        let mut val_counts = [0; 9];

        for j in 0..81 {
            if cell_index == j {
                continue;
            }

            //The cell we are comparing against
            let cmp_cell = &self.cells[j];

            if cmp_cell.row == cur_row {
                for k in 0..9 {
                    if cmp_cell.possible_vals[k] {
                        val_counts[k] += 1;
                    }
                }
            } else if cmp_cell.col == cur_col {
                for k in 0..9 {
                    if cmp_cell.possible_vals[k] {
                        val_counts[k] += 1;
                    }
                }
            } else if cmp_cell.subgrid == cur_sub {
                for k in 0..9 {
                    if cmp_cell.possible_vals[k] {
                        val_counts[k] += 1;
                    }
                }
            } else {
                continue;
            }
        }

        //Check val_counts. If a value is listed as a possible value for the cell, and it isn't
        //present as a possible value in any cell within the same row/col/subgrid,
        //
        for v in 0..9 {
            if main_cell.possible_vals[v] {
                if val_counts[v] == 0 {
                    // The 'real' value is always the index + 1
                    // A hash map might have been clearer here, but since a sudoku grid has fixed
                    // dimensions, we can get away with something more flexible and efficient
                    return Some(v + 1);
                }
            }
        }
        return None;
    }

    ///Find the index of the next unsolved cell in the grid.
    /// If we return 'None', it basically means the puzzle is solved
    fn get_next_unsolved_cell_index(&self) -> Option<usize> {
        let mut index = 0;
        while index < 81 {
            if let None = self.cells[index].cur_val {
                return Some(index);
            }
            index += 1;
        }
        return None;
    }

    pub fn parse_grid(file: File) -> Result<SudokuGrid, String> {
        let reader = std::io::BufReader::new(file);
        let mut sudoku_grid = SudokuGrid::new();
        let mut row_num: usize = 0;

        for line in reader.lines() {
            match line {
                Ok(li) if row_num < 9 => {
                    match sudoku_grid.parse_line(&li, row_num) {
                        Ok(_) => row_num += 1,
                        Err(s) => return Err(s)
                    };
                }
                Ok(_) => {
                    return Err("Too many lines".to_string());
                }
                Err(err) => return Err(err.to_string())
            }
        }
        return Ok(sudoku_grid);
    }

    fn parse_line(&mut self, line: &str, row_num: usize) -> Result<(), String> {
        let chars = line.split(',').collect::<Vec<&str>>();
        if chars.len() < 9 {
            return Err("All lines must have 9 characters".to_string());
        }

        for col_index in 0..9 {
            let new_cell: SudokuCell;
            let subgrid_index = SudokuGrid::get_subgrid(row_num, col_index as usize);

            if let Ok(v) = chars[col_index].parse::<usize>() {
                if v < 1 || v > 9 {
                    return Err("Values must be numbers between 1 and 9.".to_string());
                }

                let value = Some(v);
                new_cell = SudokuCell {
                    cur_val: value,
                    row: row_num,
                    col: col_index as usize,
                    subgrid: subgrid_index,
                    possible_vals: SudokuValues { values: [false; 9] },
                };
                //If we have a valid value, then we can indicate that the current
                //row/col/subgrid contain this value
                self.rows[row_num][v - 1] = true;
                self.cols[col_index][v - 1] = true;
                self.subgrids[subgrid_index][v - 1] = true;
            } else {
                //if value is not a number, we'll treat it as blank
                new_cell = SudokuCell::new(row_num, col_index, subgrid_index);
            }
            self.cells.push(new_cell);
        }
        return Ok(());
    }

    fn get_subgrid(row: usize, col: usize) -> usize {
        return if row < 3 {
            if col < 3 {
                0
            } else if col < 6 {
                1
            } else {
                2
            }
        } else if row < 6 {
            if col < 3 {
                3
            } else if col < 6 {
                4
            } else {
                5
            }
        } else {
            if col < 3 {
                6
            } else if col < 6 {
                7
            } else {
                8
            }
        };
    }

    fn copy_over_grid(&mut self, grid: &SudokuGrid) {
        for i in 0..81 {
            self.cells[i].cur_val = grid.cells[i].cur_val;
            for j in 0..9 {
                self.cells[i].possible_vals[j] = grid.cells[i].possible_vals[j];
            }
        }
        for i in 0..9 {
            for j in 0..9 {
                self.rows[i][j] = grid.rows[i][j];
                self.cols[i][j] = grid.cols[i][j];
                self.subgrids[i][j] = grid.subgrids[i][j];
            }
        }
    }
}

impl fmt::Debug for SudokuGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        let mut row_counter = 0;
        for i in 0..81 {
            match self.cells[i].cur_val {
                Some(x) => output.push_str(&x.to_string()),
                _ => output.push('*'),
            }
            output.push(',');
            row_counter += 1;
            if row_counter == 9 {
                row_counter = 0;
                output.push('\n');
            }
        }
        return write!(f, "{}", output);
    }
}

enum Possibilities {
    One,
    Many,
    None,
}
