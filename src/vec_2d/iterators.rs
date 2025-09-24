use std::iter::Iterator;

#[derive(Debug, Clone)]
pub struct IntoIter<T> {
    pub values: Vec<Vec<T>>,
    pub current: usize,
    pub max: usize,
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