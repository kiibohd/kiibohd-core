#![no_main]
#![no_std]
#[allow(unused_imports)]
#[allow(unused_attributes)]

mod matrix;
pub use self::matrix::StateMatrix;
pub use self::matrix::state::State;

use core::convert::Infallible;
use panic_halt as _;
#[allow(unused_imports)]
use embedded_hal::{prelude::*, digital::v2::*};

pub struct Scan {
    rows: [&'static mut dyn InputPin<Error = Infallible>; 7],
    cols: [&'static mut dyn OutputPin<Error = Infallible>; 20],
    colcnt: usize,
    rowcnt: usize,
    colflr: usize,
    colceil: usize,
    matrix: StateMatrix,
}

impl Scan {

    pub fn new(self, rows: ([&'static mut dyn InputPin<Error = Infallible>; 7], usize), cols: ([&'static mut dyn OutputPin<Error = Infallible>; 20], usize)) -> Scan {
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

    pub fn scan_change(&mut self, flr: usize, ceil: usize) {
        self.colflr = flr;
        self.colceil = ceil;
    }

    pub fn single_scan(&mut self) {
        let mut i = 0;
        let mut j = 0;
        while i <= self.colcnt  {
            if i >= self.colflr && i < self.colceil  {
                OutputPin::set_high(self.cols[i]);
                while j <= 7 {
                    match InputPin::is_high(self.rows[j]).unwrap() {
                        true => {
                            StateMatrix::poll_update(&mut self.matrix, j, i, true);
                        }
                        false => {
                            StateMatrix::poll_update(&mut self.matrix, j, i, false);
                        }
                    }
                    j = j + 1;
                }
                OutputPin::set_low(self.cols[i]);
            }
            i = i + 1;
        }
        // TODO
        //send scan end event
    }
}