use std::time::{Duration, Instant};
use crate::timer::Timer;

/// Off timer. When the enable signal goes low, the output is high for set duration
/// 
pub struct Tof {
    start_time  : Instant,
    set_time    : Duration,
    enabled     : bool,
    gone_high   : bool,
    gone_low    : bool,
}


impl Tof {

    /// Creates a new disabled instance of timer
    /// 
    pub fn new(set_time: Duration) -> Self {
        Self{
            start_time: Instant::now(),
            set_time,
            enabled: false,
            gone_high: false,
            gone_low: false
        }
    }


    /// Creates a new enabled instance of timer
    /// 
    pub fn new_enabled(set_time: Duration) -> Self {
        Self{
            start_time: Instant::now(),
            set_time,
            enabled: true,
            gone_high: false,
            gone_low: false
        }
    }


    /// Change the set time
    /// 
    pub fn set_time(&mut self, new_time: Duration) {
        self.set_time = new_time;
    }
}


impl Timer for Tof {
    fn q(&self) -> bool {
        if !self.gone_low {return false}

        if self.start_time.elapsed() <= self.set_time {
            true
        } else {
            false
        }
    }

    fn set_status(&mut self, status: bool) {
        let prev_state = self.enabled;
        self.enabled = status;

        if status {
            self.gone_high = true;
        } else if self.gone_high {
            self.gone_low = true;
        }
        
        if !prev_state && prev_state {
            self.start_time = Instant::now();
        }
    }

    fn et(&self) -> Option<Duration> {
        todo!()
    }
}
