#[allow(unused_imports)]

use core::convert::Infallible;
use panic_halt as _;

#[derive(PartialEq, Copy, Clone)]
pub enum State {
    Pressed,
    Released,
    Idle,
}

#[derive(Copy, Clone)]
pub struct KeyState {
    state: State,
    prev_state: State,
    pressed_dur: i32,
    release_dur: i32,
    bounce_limit: i32,
    idle_time: i32,
}

impl KeyState {

    pub fn new(bl: i32) -> KeyState {
        KeyState {
            state: State::Released,
            prev_state: State::Released,
            pressed_dur: 0,
            release_dur: 0,
            bounce_limit: bl,
            idle_time: 1000,
        }
    }

    pub fn get_state(&self) -> State {
        self.state
    }
    
    pub fn poll_update(&mut self, high: bool) -> (bool, State) {
        self.prev_state = self.state;
        if self.prev_state == State::Idle && high == false {
            self.state = State::Idle;
        } else {
            self.state = if high { State::Pressed } else { State::Released };
        }

        if self.prev_state == self.state && self.state == State::Pressed {
            self.pressed_dur = self.pressed_dur + 1;
            return (KeyState::bounce_check(self), State::Pressed);
        } else if self.prev_state != self.state && self.state == State::Pressed {
            self.pressed_dur = self.pressed_dur +1;
            return (false, State::Pressed);
        } else if self.prev_state == self.state && self.state == State::Released {
            if self.release_dur <= self.idle_time {
                self.state = State::Idle;
                return (true, State::Idle);
            } else {
                self.release_dur = self.release_dur + 1;
                return (false, State::Released);
            }
        } else if self.prev_state != self.state && self.state == State::Released {
            self.release_dur = self.release_dur + 1;
            return (false, self.prev_state);
        } else {
            (false, self.state)
        }
    }

    pub fn get_bounce(&self) -> i32 {
        self.pressed_dur
    }

    fn bounce_check(&self) -> bool {
        if self.pressed_dur >= self.bounce_limit {true}
        else {false}
    }

}