use std::time::Duration;

pub enum Variant{
    Ton{
        t: Duration
    },
    Tof{
        t: Duration
    }
}


impl Variant{
    /// Creates the TON variant with given set_time
    /// 
    pub fn ton(t: Duration) -> Self {
        Self::Ton{t}
    }

    
    /// Creates the TOF variant with given set_time
    /// 
    pub fn tof(t: Duration) -> Self {
        Self::Tof{t}
    }
}