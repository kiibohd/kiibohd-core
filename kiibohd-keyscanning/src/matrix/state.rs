#![no_main]
#![no_std]

use core::convert::Infallible;
use panic_halt as _;

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

    pub fn new(state: State) -> KeyState {
        KeyState {
            state: State,
            prev_state: State::Released,
            pressed_dur: 0,
        }
    }

    pub fn get_state(&self) -> State {
        self.state
    }
    
    pub fn set_state(&self, state: State) -> bool {
        self.prev_state = self.state;
        self.state = State;
        if self.prev_state.get_state() == self.state.get_state() {
            self.pressed_dur++;
            false
        } else {
            true
        }
    }
}