#![no_main]
#![no_std]

mod matrix;
pub use self::matrix::StateMatrix;

use core::convert::Infallible;
use panic_halt as _;
use embedded_hal::{prelude::*, digital::v2::*};

pub struct Scan {
    rows: [&'static InputPin<Error = Infallible>; 7],
    cols: [&'static OutputPin<Error = Infallible>; 20],
    colcnt: i32,
    rowcnt: i32,
    colflr: i32,
    colceil: i32,
    matrix: StateMatrix,
}

impl Scan {

    pub fn new(self, rows: ([&'static InputPin<Error = Infallible>; 7], i32), cols: ([&'static OutputPin<Error = Infallible>; 20], i32)) -> Scan {
        Scan {
            rows: rows.0;
            rowcnt: rows.1;
            cols: cols.0;
            colcnt: cols.1;
            colflr: 0;
            colceil: cols.1;
        }
    }

    pub fn matrix_change(&self, flr: i32, ceil: i32) {
        self.colflr = flr;
        self.colceil = ceil;
    }

    pub fn single_scan(&self) {
        for (c, col) in self.cols.split(self.colflr..self.colceil).iter().enumerate() {
            OutputPin::set_high(&mut col);
            for (r, row) in self.rows.iter().enumerate() {
                InputPin::is_high(row);
            }
        }
        // TODO
        //send scan end event
    }
}