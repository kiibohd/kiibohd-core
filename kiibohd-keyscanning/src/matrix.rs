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
            keys: [[KeyState::new(10 as i32); 7]; 20],
        }
    }

    pub fn poll_update(&mut self, r: usize, c: usize, high: bool) -> bool {
        let change = KeyState::poll_update(&mut self.keys[r][c], high);
        false
    }

    pub fn get_state(&self, r: usize, c: usize) -> State {
        KeyState::get_state(&self.keys[r][c])
    }

}