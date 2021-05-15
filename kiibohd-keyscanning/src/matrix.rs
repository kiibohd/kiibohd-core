#![no_main]
#![no_std]
#[allow(unused_imports)]

pub mod state;
pub use self::state::KeyState;
pub use self::state::State;

use core::convert::Infallible;
use panic_halt as _;
use embedded_hal::{prelude::*, digital::v2::*};

pub struct StateMatrix {
    keys: [[KeyState; 7]; 20],
}

impl StateMatrix {

    pub fn new() -> StateMatrix {
        StateMatrix {
            keys: [[KeyState::new(); 7]; 20],
        }
    }

    pub fn set_state(&self, r: usize, c: usize, state: State) -> bool {
        KeyState::set_state(&self.keys[r][c], state)
    }

}