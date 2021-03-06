// Copyright 2021 Zion Koyl
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![no_std]
#[allow(unused_imports)]
#[allow(unused_attributes)]
/// This crate is to handle scanning and strobing of the key matrix.
/// It also handles the debouncing of key input to ensure acurate keypresses are being read.
/// InputPin's, and OutputPin's are passed in through the "rows" and "cols" parameters in the Scan::new() function.
/// The maximum number of rows is 7, and the maximum number of columns is 20. This number may need adjusted through testing.
pub mod state;
pub use self::state::{KeyState, State, StateReturn};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use embedded_time::duration::*;
use generic_array::{ArrayLength, GenericArray};
use keyberon::matrix::HeterogenousArray;

pub struct Matrix<C, R> {
    // The matrix of inputs, and outputs, and the state of each key
    pub cols: C,
    pub rows: R,
    pub state_matrix: StateMatrix,
}

impl<C, R> Matrix<C, R> {
    pub fn new<E>(cols: C, rows: R, scan_period: Microseconds) -> Result<Self, E>
    where
        for<'a> &'a mut C: IntoIterator<Item = &'a mut dyn OutputPin<Error = E>>,
    {
        let state_matrix = StateMatrix::new(
            5_u32.milliseconds(),
            500_u32.milliseconds(),
            700_u32.milliseconds(),
            scan_period,
        ); // (debounce-duration, held-duration, idle-duration, scan-period)
        let mut res = Self {
            cols,
            rows,
            state_matrix,
        };
        res.clear()?;
        Ok(res)
    }

    pub fn clear<'a, E: 'a>(&'a mut self) -> Result<(), E>
    where
        &'a mut C: IntoIterator<Item = &'a mut dyn OutputPin<Error = E>>,
    {
        for c in self.cols.into_iter() {
            c.set_low().ok().unwrap();
        }
        Ok(())
    }

    /// This is the main matrix scanning function.
    /// The function iterates over the array of columns, sets them high, then iterates over the
    /// array of rows and reads their state.
    /// For each key that's state has changed since last scan the "callback" function is executed.
    /// The idea for the callback is that you can use your own handling of state changes
    /// independent of the scanning module.
    pub fn get<'a, E: 'a>(&'a mut self, callback: fn(StateReturn, usize, bool)) -> Result<(), E>
    where
        &'a mut C: IntoIterator<Item = &'a mut dyn OutputPin<Error = E>>,
        C: HeterogenousArray,
        C::Len: ArrayLength<GenericArray<bool, R::Len>>,
        C::Len: heapless::ArrayLength<GenericArray<bool, R::Len>>,
        &'a R: IntoIterator<Item = &'a dyn InputPin<Error = E>>,
        R: HeterogenousArray,
        R::Len: ArrayLength<bool>,
        R::Len: heapless::ArrayLength<bool>,
    {
        let rows = &self.rows;
        let state_matrix = &mut self.state_matrix;
        for (i, c) in self.cols.into_iter().enumerate() {
            c.set_high().ok().unwrap();
            for (j, r) in rows.into_iter().enumerate() {
                let on = r.is_high().ok().unwrap();
                let state: StateReturn = state_matrix.poll_update(j, i, on);
                callback(state, state_matrix.get_scancode(j, i), on);
            }
            c.set_low().ok().unwrap();
        }

        Ok(())
    }
}

/// The matrix to keep all the key states and handle state updating
pub struct StateMatrix {
    keys: [[KeyState; 7]; 20],
}

impl StateMatrix {
    pub fn new(
        bounce_limit: Milliseconds,
        held_limit: Milliseconds,
        idle_limit: Milliseconds,
        scan_period: Microseconds,
    ) -> StateMatrix {
        StateMatrix {
            // Create a two dimensional array of key states with a debounce delay of 5ms, a hold time of 5ms, and an idle limit of 500ms
            keys: [[KeyState::new(bounce_limit, held_limit, idle_limit, scan_period); 7]; 20],
        }
    }

    // Update the individual KeyStates in the array\
    //TODO Do something with the returned StateReturn
    pub fn poll_update(&mut self, r: usize, c: usize, high: bool) -> StateReturn {
        KeyState::poll_update(&mut self.keys[r][c], high)
    }

    // Get the individual state of a specific key
    pub fn get_state(&self, r: usize, c: usize) -> State {
        KeyState::get_state(&self.keys[r][c])
    }

    pub fn get_scancode(&self, r: usize, c: usize) -> usize {
        c + (19 * r)
    }
}
