#![no_main]
#![no_std]
#[allow(unused_imports)]

use core::convert::Infallible;
use panic_halt as _;

#[derive(PartialEq)]
pub enum State {
    Pressed,
    Released
}

pub struct KeyState {
    state: State,
    prev_state: State,
    pressed_dur: i32,
}

impl KeyState {

    pub fn new() -> KeyState {
        KeyState {
            state: State::Released,
            prev_state: State::Released,
            pressed_dur: 0,
        }
    }

    pub fn get_state(&self) -> State {
        self.state
    }
    
    pub fn set_state(&self, state: State) -> bool {
        self.prev_state = self.state;
        self.state = state;
        if self.prev_state == self.state {
            self.pressed_dur = self.pressed_dur + 1;
            return false;
        } else {
            return true;
        }
    }
}