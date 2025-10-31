use std::time::Duration;

/// Contains all different types of timers
/// e.g. TON, TOF and much more (at some point)
///
mod variants;
pub use variants::*;


/// Defines how a timer works
///
pub trait Timer {

    /// Get the status of the output
    /// 
    fn q(&self) -> bool;


    /// Get the elapsed time.
    /// Returns None if the timer has not been started
    /// 
    fn et(&self) -> Option<Duration> {
        unimplemented!("et() not implemented for this timer")
    }


    /// Gives the time until output of timer is goes high.
    /// Returns None if timer is not enabled or the output is already high
    /// 
    fn time_remain(&self) -> Option<Duration> {
        unimplemented!("time_remain() not implemented for this timer")
    }


    /// Set the start signal of the timer to true
    /// 
    fn start(&mut self) {
        self.set_status(true);
    }


    /// Set the start signal of the timer to false, resets the elapsed time
    /// 
    fn stop(&mut self) {
        self.set_status(false);
    }


    /// Sets the status of the input. On reset, elapsed time is reset
    /// 
    fn set_status(&mut self, status: bool);
}