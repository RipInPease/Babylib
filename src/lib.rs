pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub mod rng {
  pub fn rand (min: u32, max: u32) -> u32 {
    use std::time::SystemTime;
    let sys_time = SystemTime::now();
    max
  }
}
