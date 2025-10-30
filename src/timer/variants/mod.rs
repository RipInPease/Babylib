use std::time::Duration;

pub enum Variant{
    Ton(Duration),
}


impl Variant{
    pub fn ton(t: Duration) -> Self {
        Self::Ton(t)
    }
}