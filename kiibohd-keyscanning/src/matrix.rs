#[allow(unused_imports)]

pub mod state;
pub use self::state::KeyState;
pub use self::state::State;

use core::convert::Infallible;
use panic_halt as _;
use embedded_hal::{prelude::*, digital::v2::*};
use embedded_time::{duration::*, rate::*};

pub struct RowArray { // An array of the input pin's for each row, and a count of how full the array is
    pub rows: [&'static mut dyn InputPin<Error = Infallible>; 7],
    pub rowcnt: usize,
}

impl RowArray {
    pub fn new(rows: [&'static mut dyn InputPin<Error = Infallible>; 7], rowcnt: usize) -> RowArray {
        RowArray {
            rows: rows,
            rowcnt: rowcnt,
        }
    }
}

pub struct ColArray { // An array of the output pin's for each column, and a count of how full the array is
    pub cols: [&'static mut dyn OutputPin<Error = Infallible>; 20],
    pub colcnt: usize,
}

impl ColArray {
    pub fn new(cols: [&'static mut dyn OutputPin<Error = Infallible>; 20], colcnt: usize) -> ColArray{
        ColArray {
            cols: cols,
            colcnt: colcnt,
        }
    }
}

pub struct Matrix { // The matrix of inputs, and outputs, and the state of each key
    pub rowarray: RowArray,
    pub colarray: ColArray,
    pub state_matrix: StateMatrix,
}

impl Matrix {
    pub fn new(rows: RowArray, cols: ColArray, scan_period: Microseconds) -> Matrix {
        Matrix {
            rowarray: rows,
            colarray: cols,
            state_matrix: StateMatrix::new(scan_period),
        }
    }
}

/// The matrix to keep all the key states and handle state updating
pub struct StateMatrix {
    keys: [[KeyState; 7]; 20],
}

impl StateMatrix {

    pub fn new(scan_period: Microseconds) -> StateMatrix {
        StateMatrix { // Create a two dimensional array of key states with a debounce delay of 5ms, a hold time of 5ms, and an idle limit of 500ms
            keys: [[KeyState::new(5_u32.milliseconds(), 5_u32.milliseconds(), 500_u32.milliseconds(), scan_period); 7]; 20],
        }
    }

    pub fn poll_update(&mut self, r: usize, c: usize, high: bool) -> bool {
        let _change = KeyState::poll_update(&mut self.keys[r][c], high);
        false
    }

    pub fn get_state(&self, r: usize, c: usize) -> State {
        KeyState::get_state(&self.keys[r][c])
    }

}