mod sudoku_cell;
mod sudoku_grid;
mod sudoku_values;

use std::fs::File;
use std::io;
use sudoku_grid::SudokuGrid;
use std::path::Path;

fn main() -> io::Result<()> {
    println!("{0}", "Please enter the puzzle's path");

    loop {
        match read_grid() {
            Ok(grid) => {
                match grid.solve_grid() {
                    Ok(g) => {
                        println!("{:?}", g);
                        break;
                    }
                    Err(e) => {
                        println!("{:?}", e);
                        break;
                    }
                }
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    Ok(())
}

fn read_grid() -> Result<SudokuGrid, String> {
    let mut input = String::new();
    let input_result = io::stdin().read_line(&mut input);

    if let Err(_) = input_result {
        return Err("Failed to read input".to_string());
    }
    let path = Path::new(input.trim());

    let file = File::open(path);
    if let Err(_) = file {
        return Err("Invalid file".to_string());
    }

    let final_file = file.unwrap();

    return SudokuGrid::parse_grid(final_file);
}