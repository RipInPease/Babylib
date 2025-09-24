
use std::slice::Iter;


#[derive(Debug, Clone)]
pub struct Vec2d<T> {
    values: Vec<Vec<T>>
}


///////////////////////////////////
////// Methods and function ///////
///////////////////////////////////
impl<T> Vec2d<T> {

    ///Iterates over all columns
    
    pub fn iter(&self) -> Iter<'_, Vec<T>> {
    self.values.iter()
    }


    ///Creates a new 2d vector with given column and row dimensions and default value

    pub fn new(col: usize, row: usize, value: T) -> Self 
    where 
    T: Clone {

        Self{
            values: vec![vec![value.clone(); row]; col]
        }
    }

    

    ///Removes last value and returns a reference it

    pub fn pop(&mut self) -> Option<Vec<T>> {
    self.values.pop()
    }



    /// Appends an element to the back of a collection.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` _bytes_.

    pub fn push(&mut self, value: Vec<T>) {
    self.values.push(value);
    }
}





///////////////////////////////////
/////// Trait implentations ///////
///////////////////////////////////
use std::ops::Index;
use std::ops::IndexMut;
mod iterators;


impl<T> Index<usize> for Vec2d<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
    &self.values[index]
    }
}

impl<T> IndexMut<usize> for Vec2d<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.values[index]
    }
}
  