#![no_main]
#![no_std]

mod state;
pub use self::state::KeyState;

use core::convert::Infallible;
use atsam4_hal::{InputPin, OutputPin};
use panic_halt as _;
use embedded_hal::{prelude::*, digital::v2::*};

pub struct StateMatrix {
    keys: [[KeyState; 7]; 20],
}