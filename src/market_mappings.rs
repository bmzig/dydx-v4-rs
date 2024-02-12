use crate::Market;

use std::fmt;

impl Market {
    pub fn ticker(&self) -> String {
        self.to_string()
    }

    pub const fn id(&self) -> u32 {
        match *self {
            Market::ETH => 1,
            _ => 0,
        }
    }
}

impl fmt::Display for Market {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}-USD", self)
    }
}
