//Module for making 2d vectors less AIDS
pub mod vec_2d {
  #[derive(Debug, Clone)]
  pub struct Vec2d<T> {
    values: Vec<Vec<T>>
  }


  //Implementations for Vec2d
  impl<T> Vec2d<T> {

    //Creates a new 2d vector with given x and y dimensions and default value
    pub fn new(x: usize, y: usize, value: T) -> Self 
      where 
      T: Clone {

          Self{
            values: vec![vec![value.clone(); y]; x]
          }
      }
  }





///////////////////////////////////
/////// Trait implentations ///////
///////////////////////////////////

  use std::ops::Index;
  use std::ops::IndexMut;


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
}



