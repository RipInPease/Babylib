//Module for making 2d vectors less AIDS
pub mod vec_2d {
  struct Vec2d<T> {
    values: Vec<Vec<T>>
  }


  //Implementations for Vec2d
  impl<T> Vec2d<T> {

    //Creates a new 2d vector with given x and y dimensions and default value
    fn new(x: usize, y: usize, value: T) -> Self 
      where 
      T: Clone {

          Self{
            values: vec![vec![value.clone(); y]; x]
          }
      }
  } 
}
