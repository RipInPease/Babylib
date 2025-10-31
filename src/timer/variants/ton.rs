use std::time::{Duration, SystemTime};
use crate::timer::Timer;
/// Timer on. 
/// Output high when elapsed time is greater or equal to set time. 
/// 
#[derive(Debug, Clone)]
pub struct Ton {
    start_time  : SystemTime,
    set_time    : Duration,
    enabled     : bool,
}

impl Ton {

    /// Creates a new disabled instance of timer
    /// 
    pub fn new(set_time: Duration) -> Self {
        Self{
            start_time: SystemTime::now(),
            set_time,
            enabled: false,
        }
    }


    /// Creates a new enabled instance of timer
    /// 
    pub fn new_enabled(set_time: Duration) -> Self {
        Self{
            start_time: SystemTime::now(),
            set_time,
            enabled: true,
        }
    }


    /// Change the set time
    /// 
    pub fn set_time(&mut self, new_time: Duration) {
        self.set_time = new_time;
    }
}

impl Timer for Ton {
    fn q(&self) -> bool {
        if !self.enabled {return false}
        
        if SystemTime::now().duration_since(self.start_time).unwrap() >= self.set_time {
            true
        } else {
            false
        }
    }

    fn set_status(&mut self, status: bool) {
        if !status {
            self.enabled = status;
            return;
        }

        if !self.enabled {
            self.start_time = SystemTime::now();
        } 
        self.enabled = status;
    }

    fn et(&self) -> Option<Duration> {
        if !self.enabled {return None}

        let et = SystemTime::now().duration_since(self.start_time).unwrap();
        Some(et)
    }

    fn time_remain(&self) -> Option<Duration> {
        if self.q() || !self.enabled {
            return None
        }

        Some(self.set_time - self.et()?)
    }
}