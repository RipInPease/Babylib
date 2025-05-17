//pub mod rng {
//  pub fn rand (min: u32, max: u32) -> u32 {
//    use std::time::SystemTime;
//    let sys_time = SystemTime::now();
//    max
//  }
//}

pub mod Vec2d {
  pub fn new<T> (x: usize, y: usize, value: T) -> Vec<Vec<T>>
  where 
    T: Clone {
      vec![vec![value; y]; x]
    }

}
