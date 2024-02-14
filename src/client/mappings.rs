use crate::Market;

use std::fmt;

impl Market {

}

impl fmt::Display for Market {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}-USD", self)
    }

}

/*
impl AsRef<str> for Market {

    fn as_ref(&self) -> &str {
        self.to_string()
    }
}
*/

impl Market {

    pub const fn atomic_resolution(&self) -> i8 {
        match *self {
            Market::BTC => -10,
            Market::ETH => -9,
            Market::SOL => -7,
            Market::AVAX => -7,
            Market::MATIC => -5,
            Market::NEAR => -6,
            Market::LINK => -6,
            Market::ATOM => -6,
            Market::TRX => -4,
            Market::UNI => -6,
            Market::ETC => -7,
            Market::APE => -6,
            Market::PEPE => 1,
            Market::SEI => -5,
            Market::LTC => -7,
            Market::CRV => -5,
            Market::ARB => -6,
            Market::SUI => -5,
            Market::AAVE => -7,
            Market::SHIB => 0,
            Market::DOGE => -4,
            Market::MKR => -9,
            Market::WLD => -6,
            Market::BLUR => -5,
            Market::ADA => -5,
            Market::APT => -6,
            Market::FIL => -6,
            Market::COMP => -7,
            Market::TIA => -7,
            Market::JUP => -5,
            Market::DOT => -6,
            Market::LDO => -6,
            Market::BNB => -8,
            Market::XRP => -5,
            Market::OP => -6,
            Market::BCH => -8,
            Market::XLM => -5,
        }
    }

    pub const fn tick_size(&self) -> f32 {
        match *self {
            Market::BTC => 1.0,
            Market::ETH => 0.1,
            Market::SOL => 0.01,
            Market::AVAX => 0.01,
            Market::MATIC => 0.0001,
            Market::NEAR => 0.001,
            Market::LINK => 0.001,
            Market::ATOM => 0.001,
            Market::TRX => 0.00001,
            Market::UNI => 0.001,
            Market::ETC => 0.01,
            Market::APE => 0.001,
            Market::PEPE => 0.0000000001,
            Market::SEI => 0.0001,
            Market::LTC => 0.01,
            Market::CRV => 0.0001,
            Market::ARB => 0.001,
            Market::SUI => 0.0001,
            Market::AAVE => 0.01,
            Market::SHIB => 0.000000001,
            Market::DOGE => 0.00001,
            Market::MKR => 1.0,
            Market::WLD => 0.001,
            Market::BLUR => 0.0001,
            Market::ADA => 0.0001,
            Market::APT => 0.001,
            Market::FIL => 0.001,
            Market::COMP => 0.01,
            Market::TIA => 0.01,
            Market::JUP => 0.0001,
            Market::DOT => 0.001,
            Market::LDO => 0.001,
            Market::BNB => 0.1,
            Market::XRP => 0.0001,
            Market::OP => 0.001,
            Market::BCH => 0.1,
            Market::XLM => 0.0001,
        }
    }

    pub const fn step_size(&self) -> f32 {
        match *self {
            Market::BTC => 0.0001,
            Market::ETH => 0.001,
            Market::SOL => 0.1,
            Market::AVAX => 0.1,
            Market::MATIC => 10.0,
            Market::NEAR => 1.0,
            Market::LINK => 1.0,
            Market::ATOM => 1.0,
            Market::TRX => 100.0,
            Market::UNI => 1.0,
            Market::ETC => 0.1,
            Market::APE => 1.0,
            Market::PEPE => 10000000.0,
            Market::SEI => 10.0,
            Market::LTC => 0.1,
            Market::CRV => 10.0,
            Market::ARB => 1.0,
            Market::SUI => 10.0,
            Market::AAVE => 0.1,
            Market::SHIB => 1000000.0,
            Market::DOGE => 100.0,
            Market::MKR => 0.001,
            Market::WLD => 1.0,
            Market::BLUR => 10.0,
            Market::ADA => 10.0,
            Market::APT => 1.0,
            Market::FIL => 1.0,
            Market::COMP => 0.1,
            Market::TIA => 0.1,
            Market::JUP => 10.0,
            Market::DOT => 1.0,
            Market::LDO => 1.0,
            Market::BNB => 0.01,
            Market::XRP => 10.0,
            Market::OP => 1.0,
            Market::BCH => 0.01,
            Market::XLM => 10.0,
        }
    }

    pub const fn subticks_per_tick(&self) -> u64 {
        match *self {
            Market::BTC => 100000,
            Market::ETH => 100000,
            Market::SOL => 1000000,
            Market::AVAX => 1000000,
            Market::MATIC => 1000000,
            Market::NEAR => 1000000,
            Market::LINK => 1000000,
            Market::ATOM => 1000000,
            Market::TRX => 1000000,
            Market::UNI => 1000000,
            Market::ETC => 1000000,
            Market::APE => 1000000,
            Market::PEPE => 1000000,
            Market::SEI => 1000000,
            Market::LTC => 1000000,
            Market::CRV => 1000000,
            Market::ARB => 1000000,
            Market::SUI => 1000000,
            Market::AAVE => 1000000,
            Market::SHIB => 1000000,
            Market::DOGE => 1000000,
            Market::MKR => 1000000,
            Market::WLD => 1000000,
            Market::BLUR => 1000000,
            Market::ADA => 1000000,
            Market::APT => 1000000,
            Market::FIL => 1000000,
            Market::COMP => 1000000,
            Market::TIA => 1000000,
            Market::JUP => 1000000,
            Market::DOT => 1000000,
            Market::LDO => 1000000,
            Market::BNB => 1000000,
            Market::XRP => 1000000,
            Market::OP => 1000000,
            Market::BCH => 1000000,
            Market::XLM => 1000000,
        }
    }

    pub fn vec() -> Vec<Market> {
        vec![
            Market::BTC,
            Market::ETH,
            Market::SOL,
            Market::AVAX,
            Market::MATIC,
            Market::NEAR,
            Market::LINK,
            Market::ATOM,
            Market::TRX,
            Market::UNI,
            Market::ETC,
            Market::APE,
            Market::PEPE,
            Market::SEI,
            Market::LTC,
            Market::CRV,
            Market::ARB,
            Market::SUI,
            Market::AAVE,
            Market::SHIB,
            Market::DOGE,
            Market::MKR,
            Market::WLD,
            Market::BLUR,
            Market::ADA,
            Market::APT,
            Market::FIL,
            Market::COMP,
            Market::TIA,
            Market::JUP,
            Market::DOT,
            Market::LDO,
            Market::BNB,
            Market::XRP,
            Market::OP,
            Market::BCH,
            Market::XLM,
        ]
    }
}
