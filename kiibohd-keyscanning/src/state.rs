// Copyright 2021 Jacob Alexander, Zion Koyl
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[allow(unused_imports)]
use core::convert::Infallible;
use embedded_time::{duration::*, rate::*};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum State {
    Pressed,
    Bouncing,
    Released,
    Idle,
    Held,
}

#[derive(PartialEq, Copy, Clone)]
pub struct StateReturn {
    pub state_change: bool,
    pub ending_state: State,
}

/// The KeyState handles all of the decision making and state changes based on a high signal from the GPIO, or a low signal
#[derive(Copy, Clone)]
pub struct KeyState {
    state: State,
    prev_state: State,
    pressed_dur: Milliseconds,
    bouncing_dur: Milliseconds,
    released_dur: Milliseconds,
    held_dur: Milliseconds,
    bounce_limit: Milliseconds,
    idle_time: Milliseconds,
    scan_period: Microseconds,
    held_limit: Milliseconds,
    idle_limit: Milliseconds,
}

impl KeyState {
    pub fn new(
        bounce_limit: Milliseconds,
        held_limit: Milliseconds,
        idle_limit: Milliseconds,
        scan_period: Microseconds,
    ) -> KeyState {
        KeyState {
            state: State::Released,             // Current key state
            prev_state: State::Released,        // Previous key state
            pressed_dur: 0_u32.milliseconds(), // Duration(ms) the key has been pressed(after debouncing)
            bouncing_dur: 0_u32.milliseconds(), // Duration(ms) the key has been debouncing
            released_dur: 0_u32.milliseconds(), // Duration(ms) the key has been released
            held_dur: 0_u32.milliseconds(),    // Duration(ms) the key has been held
            bounce_limit: bounce_limit, // Duration(ms) that the key has to be high to be considered debounced
            idle_time: 0_u32.milliseconds(), // Duration(ms) the key has been idle
            scan_period: scan_period,   // the period of time(microseconds) that the scan takes
            held_limit: held_limit, // Duration(ms) that the key needs to be pressed to be considered held
            idle_limit: idle_limit, // Duration(ms) that the key needs to be released to be considered idle
        }
    }

    pub fn get_state(&self) -> State {
        self.state
    }

    /// The result of the last scan gets sent here and thie function determines the actual state change of the key.
    /// This function returns (state change, ending state)
    pub fn poll_update(&mut self, high: bool) -> StateReturn {
        let zero: Milliseconds = 0_u32.milliseconds();
        let scan_period: Milliseconds = self.scan_period.into();
        let mut state_change: bool = false;
        self.prev_state = self.state;
        if high == false {
            // if the GPIO reads the input pin low
            match self.prev_state {
                State::Pressed => {
                    // if the previous state was pressed and the input is read as low
                    self.state = State::Released;
                    self.pressed_dur = zero;
                    state_change = true;
                }
                State::Bouncing => {
                    // the previous state was bouncing and the output was read as low
                    self.state = State::Released;
                    self.bouncing_dur = zero;
                    state_change = true;
                }
                State::Released => {
                    // if the previous state was released and the input is still low
                    if self.released_dur >= self.idle_limit {
                        // The key was Released and is now considered idle
                        self.state = State::Idle;
                        self.released_dur = zero;
                        state_change = true;
                    } else {
                        // The key was released, but is not yet idle
                        self.state = State::Released;
                        self.released_dur = self.released_dur + scan_period;
                    }
                }
                State::Idle => {
                    // if the previous state was idle and the input is still read as low
                    self.state = State::Idle;
                    self.idle_time = self.idle_time + scan_period;
                }
                State::Held => {
                    self.state = State::Released;
                    self.held_dur = zero;
                    state_change = true;
                }
            }
        } else if high == true {
            // if the GPIO reads the input pin high
            match self.prev_state {
                State::Pressed => {
                    if self.pressed_dur >= self.held_limit {
                        // if the key was pressed
                        self.state = State::Held;
                        state_change = true;
                    } else {
                        self.state = State::Pressed;
                        self.pressed_dur = self.pressed_dur + scan_period;
                    }
                }
                State::Bouncing => {
                    if self.bouncing_dur >= self.bounce_limit {
                        // The key has been pressed longer than the debounce limit and is officialyl pressed
                        self.state = State::Pressed;
                        self.bouncing_dur = zero;
                        state_change = true;
                    } else {
                        // The key is still in the debounce phase
                        self.state = State::Bouncing;
                        self.bouncing_dur = self.bouncing_dur + scan_period;
                    }
                }
                State::Released => {
                    // The key was released, but now the GPIO reads high
                    self.state = State::Bouncing;
                    self.released_dur = zero;
                    state_change = true;
                }
                State::Idle => {
                    // The key was idle, but is now bouncing
                    self.state = State::Bouncing;
                    self.idle_time = zero;
                    state_change = true;
                }
                State::Held => {
                    // The key was held, and is still held
                    self.state = State::Held;
                    self.held_dur = self.held_dur + scan_period;
                }
            }
        }

        return StateReturn {
            state_change: state_change,
            ending_state: self.state,
        };
    }
}
