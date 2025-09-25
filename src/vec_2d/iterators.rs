use std::slice::Iter;

#[derive(Debug, Clone)]
pub struct IntoIter<T> {
    values: Vec<Vec<T>>,
    current: usize,
    max: usize,
}


#[derive(Debug, Clone)]
pub struct IterByRow<'a, T: 'a> {
    pub values: &'a Vec<Vec<T>>,
    pub curr_row: usize,
    pub curr_col: usize,
    pub max_rows: usize,
    pub max_cols: usize,
    pub curr_emptys: usize,
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
}