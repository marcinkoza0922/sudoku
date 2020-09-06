use core::ops::Index;
use core::ops::IndexMut;
use std::fmt;

///Purpose of this struct
#[derive(Clone)]
pub struct SudokuValues {
    pub values: [bool; 9],
}

impl SudokuValues {
    pub fn new(init_val: bool) -> Self {
        SudokuValues {
            values: [init_val; 9],
        }
    }
}

impl Index<usize> for SudokuValues {
    type Output = bool;

    fn index(&self, index: usize) -> &bool {
        &self.values[index]
    }
}

impl IndexMut<usize> for SudokuValues {
    fn index_mut(&mut self, index: usize) -> &mut bool {
        &mut self.values[index]
    }
}

impl fmt::Debug for SudokuValues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pos_val_string = String::new();
        pos_val_string.push('[');
        for i in 0..9 {
            if self.values[i] {
                pos_val_string.push_str(&(i + 1).to_string());
                pos_val_string.push_str(", ");
            }
        }
        pos_val_string.push(']');

        return write!(f, "Cell Data: {}", pos_val_string);
    }
}

impl fmt::Display for SudokuValues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pos_val_string = String::new();
        pos_val_string.push('[');
        for i in 0..9 {
            if self.values[i] {
                pos_val_string.push_str(&(i + 1).to_string());
                pos_val_string.push_str(", ");
            }
        }
        pos_val_string.push(']');

        return write!(f, "Cell Data: {}", pos_val_string);
    }
}
