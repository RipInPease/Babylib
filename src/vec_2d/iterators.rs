/// Module for iterators of the struct Vec2d

use std::slice::Iter;
use std::num::NonZero;


/// A consuming iterator that returns a vector of all columns `vec2d[column][row]`

#[derive(Debug, Clone)]
pub struct IntoIter<T> {
    values: Vec<Vec<T>>,
    current: usize,
    max: usize,
}


impl<T: Clone> Iterator for IntoIter<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        
        let index = self.current;

        if index <= self.max {
            Some(self.values[index-1].clone())
        } else {
            None
        }
    }
}




/// An iterator that iterates over all values in each row `vec2d[column][row]`
/// before continuing to the next row

#[derive(Debug, Clone)]
pub struct IterByRow<'a, T: 'a> {
    values: &'a Vec<Vec<T>>,
    curr_row: usize,
    curr_col: usize,
    max_row: usize,
    max_cols: usize,
}


impl<'a, T: 'a > Iterator for IterByRow<'a, T> {
    type Item = Option<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_row >= self.max_row {
            return None
        }

        let ret = &self.values[self.curr_col].get(self.curr_row);

        self.curr_col += 1;
        if self.curr_col >= self.max_cols {
            self.curr_col = 0;
            self.curr_row += 1;
        }

        Some(*ret)
    }
}




/// An iterator that iterates over all values in each column `vec2d[column][row]`
/// before continuing to the next row

#[derive(Debug, Clone)]
pub struct IterByCol<'a, T: 'a> {
    values: &'a Vec<Vec<T>>,
    curr_row: usize,
    curr_col: usize,
    max_row: usize,
    max_cols: usize,
}


impl<'a, T: 'a> Iterator for IterByCol<'a, T> {
    type Item = Option<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_col >= self.max_cols {
            return None
        }

        let ret = self.values[self.curr_col].get(self.curr_row);

        self.curr_row += 1;
        if self.curr_row >= self.max_row {
            self.curr_col += 1;
            self.curr_row = 0;
        }

        Some(ret)
    }
}




/// Iterator over all the adjacent coordinates given a range
/// 
/// Given range `1` it will give an iterator of a 3x3 grid
/// because it goes `1` in each direction from the start cell
/// 
/// Start from the upper left corner and goes right and then down
/// to the lower right corner:

pub struct Neighbors<'a, T: 'a> {
    values: &'a Vec<Vec<T>>,
    range: i32,
    start_col: i32,
    start_row: i32,
    col_offset: i32,
    row_offset: i32,
}


impl<'a, T: 'a> Iterator for Neighbors<'a, T> {
    type Item = Option<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row_offset > self.range {
            return None
        }

        let col_to_get = self.start_col + self.col_offset;
        let row_to_get = self.start_row + self.row_offset;

        let ret = {
            if col_to_get < 0 || row_to_get < 0 {
                None
            } else {
                if let Some(col) = self.values.get(col_to_get as usize) {
                    col.get(row_to_get as usize)
                } else {
                    None
                }
            }
        };


        self.col_offset += 1;
        if self.col_offset > self.range {
            self.row_offset += 1;
            self.col_offset = -self.range;
        }

        Some(ret)
    }
}




/// An iterator over overlapping subslices of length `size`.

#[derive(Debug, Clone)]
pub struct Windows<'a, T: 'a> {
    slice: &'a [T],
    size: NonZero<usize>,
}


impl<'a, T: 'a> Iterator for Windows<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.size.get() > self.slice.len() {
            None
        } else {
            let ret = Some(&self.slice[..self.size.get()]);
            self.slice = &self.slice[1..];
            ret
        }
    }
}




impl<T> super::Vec2d<T> {

    /// Iterates over all columns and return the entire columns as a vector

    pub fn iter(&self) -> Iter<'_, Vec<T>> {
        self.values.iter()
    }


    /// Consumes the 2d vector and returns an iterator over the columns
    /// returning the entire column as a vector
    
    pub fn into_iter(self) -> IntoIter<T> {
        let max = self.values.len();
        let current = 0;
        let values = self.values;

        IntoIter {
            values: values,
            current: current,
            max: max
        }
    }


    /// Iterates over the 2d vector row by row instead of column by column.
    /// Will first iterate over all values in the first row, then the next and so on
    /// Returns and `Option<T>` of the matrix coordinate, None if if it empty
    
    pub fn iter_by_row(&self) -> IterByRow<'_, T> {
        IterByRow {
            values: &self.values, 
            curr_row: 0, 
            curr_col: 0, 
            max_row: self.values.iter().map(|x| x.len()).max().unwrap_or(0), 
            max_cols: self.values.len()
        }
    }




    /// Iterates over the all rows in a column `2dvec[column][row]`
    /// before continuing to next column
    
    pub fn iter_by_col(&self) -> IterByCol<'_, T> {
        IterByCol {
            values: &self.values, 
            curr_row: 0, 
            curr_col: 0, 
            max_row: self.values.iter().map(|x| x.len()).max().unwrap_or(0), 
            max_cols: self.values.len()
        }
    }




    /// An iterator that iterates over all adjecent cells in the grid
    /// given a `range`
    /// Given a `range` of _1_, will iterate over a 3x3 area. It goes
    /// _1_ in each direction from `start_pos`

    pub fn neighbors(&self, range: usize, start_pos: (usize, usize)) -> Neighbors<'_, T> {
        Neighbors {
            values: &self.values,
            range: range as i32, 
            start_col: start_pos.0 as i32,
            start_row: start_pos.1 as i32,
            col_offset: -(range as i32),
            row_offset: -(range as i32)
        }
    } 



    /// Iterates over all columns, giving you x rows at the same time
    
    pub fn windows(&self, size: usize) -> Windows<'_, Vec<T>> {
        let size = NonZero::new(size).expect("window size must be non-zero");
        Windows {
            size: size,
            slice: &self.values
        }
    }
}


