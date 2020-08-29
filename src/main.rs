mod sudoku_cell;
mod sudoku_grid;
mod sudoku_values;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use sudoku_grid::SudokuGrid;

fn main() -> io::Result<()> {
    let mut sudoku_grid = SudokuGrid::new();
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let file = File::open(input.trim_end())?;
    println!("{:?}", input);
    let mut row_num: usize = 0;
    {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(li) => {
                    match sudoku_grid.parse_line(&li, row_num) {
                        Ok(_) => row_num += 1,
                        Err(s) => println!("{:?}", s),
                    };
                }
                Err(err) => println!("{:?}", err),
            }
        }
    }
    sudoku_grid.solve_grid(false, 0);

    Ok(())
}
