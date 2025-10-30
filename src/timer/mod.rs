use std::time::SystemTime;

/// Contains all different types of timers
/// e.g. TON, TOF and much more (at some point)
///
pub mod variants;


use variants::*;




pub struct Timer {
    variant: Variant,
    start_time: SystemTime,
}