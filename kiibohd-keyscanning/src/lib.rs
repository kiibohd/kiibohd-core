#![no_main]
#![no_std]
#[allow(unused_imports)]

mod matrix;
pub use self::matrix::StateMatrix;
pub use self::matrix::state::State;

use core::convert::Infallible;
use panic_halt as _;
#[allow(unused_imports)]
use embedded_hal::{prelude::*, digital::v2::*};

pub struct Scan {
    rows: [&'static dyn InputPin<Error = Infallible>; 7],
    cols: [&'static dyn OutputPin<Error = Infallible>; 20],
    colcnt: i32,
    rowcnt: i32,
    colflr: i32,
    colceil: i32,
    matrix: StateMatrix,
}

impl Scan {

    pub fn new(self, rows: ([&'static dyn InputPin<Error = Infallible>; 7], i32), cols: ([&'static dyn OutputPin<Error = Infallible>; 20], i32)) -> Scan {
        Scan {
            rows: rows.0,
            rowcnt: rows.1,
            cols: cols.0,
            colcnt: cols.1,
            colflr: 0,
            colceil: cols.1,
            matrix: StateMatrix::new()
        }
    }

    pub fn matrix_change(&self, flr: i32, ceil: i32) {
        self.colflr = flr;
        self.colceil = ceil;
    }

    pub fn single_scan(&self) {
        for (c, col) in self.cols.iter().enumerate() {
            if (c as i32) >= self.colflr && (c as i32) < self.colceil  {
                OutputPin::set_high(&mut col);
                for (r, row) in self.rows.iter().enumerate() {
                    match InputPin::is_high(row).unwrap() {
                        true => {
                            StateMatrix::set_state(&self.matrix, r, c, State::Pressed);
                        }
                    }
                }
                OutputPin::set_low(&mut col);
            }
        }
        // TODO
        //send scan end event
    }
}