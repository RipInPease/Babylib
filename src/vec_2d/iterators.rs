///Module for iterators of the struct Vec2d

use std::slice::Iter;

#[derive(Debug, Clone)]
pub struct IntoIter<T> {
    values: Vec<Vec<T>>,
    current: usize,
    max: usize,
}


#[derive(Debug, Clone)]
pub struct IterByRow<'a, T: 'a> {
    values: &'a Vec<Vec<T>>,
    curr_row: usize,
    curr_col: usize,
    max_row: usize,
    max_cols: usize,
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


impl<'a, T: 'a > Iterator for IterByRow<'a, T> {
    type Item = Option<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_row >= self.max_row || self.curr_col >= self.max_cols {
            return None
        }

        let ret = &self.values[self.curr_col].get(self.curr_row);

        if self.curr_col == self.max_cols-1 {
            self.curr_col = 0;
            self.curr_row += 1;
        } else {
            self.curr_col += 1;
        }

        Some(*ret)
    }
}


impl<T> super::Vec2d<T> {

    ///Iterates over all columns

    pub fn iter(&self) -> Iter<'_, Vec<T>> {
        self.values.iter()
    }


    ///Consumes the 2d vector and returns an iterator over the columns
    
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


    ///Iterates over the 2d vector row by row instead of column by column.
    ///Will first iterate over all values in the first row, then the next and so on
    
    pub fn iter_by_row(&self) -> IterByRow<'_, T> {
        //println!("cols = {}", self.values.len(), );
        IterByRow {
            values: &self.values, 
            curr_row: 0, 
            curr_col: 0, 
            max_row: self.values.iter().map(|x| x.len()).max().unwrap_or(0), 
            max_cols: self.values.len()
        }
    }
}


