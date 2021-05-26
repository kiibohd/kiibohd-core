#![no_std]
#[allow(unused_imports)]
#[allow(unused_attributes)]
/// This crate is to handle scanning and strobing of the key matrix.
/// It also handles the debouncing of key input to ensure acurate keypresses are being read.
/// InputPin's, and OutputPin's are passed in through the "rows" and "cols" parameters in the Scan::new() function.
/// The maximum number of rows is 7, and the maximum number of columns is 20. This number may need adjusted through testing.

pub mod matrix;
pub use self::matrix::state::State;
pub use self::matrix::{Matrix, StateMatrix};


use core::convert::Infallible;
use atsam4_hal::{prelude::*, gpio::*};
use embedded_time::{duration::*, rate::*};
/*
/// Scan structure to handle matrix strobing and sensing
pub struct Scan {
    matrix: Matrix<(), ()>,
}

impl Scan {

    pub fn new(matrix: Matrix<(), ()>) -> Scan {
        Scan {
            matrix: matrix
        }
    }

    pub fn scan_change(&mut self, flr: usize, ceil: usize) {
        self.colflr = flr;
        self.colceil = ceil;
    }

    /*pub fn single_scan(&mut self) {
        let mut i = 0;
        let mut j = 0;
        while i <= self.matrix.colarray.colcnt  {
            if i >= self.colflr && i < self.colceil  {
                let _highret = self.matrix.colarray.cols[i].set_high().ok();
                while j <= 7 {
                    match self.matrix.rowarray.rows[j].is_high().ok().unwrap() {
                        true => {
                            StateMatrix::poll_update(&mut self.matrix.state_matrix, j, i, true);
                        }
                        false => {
                            StateMatrix::poll_update(&mut self.matrix.state_matrix, j, i, false);
                        }
                    }
                    j = j + 1;
                }
                let _lowret = self.matrix.colarray.cols[i].set_low().ok();
            }
            i = i + 1;
        }
        // TODO
        //send scan end event
    }*/

    pub fn single_scan(&mut self) {
        self.matrix.get()
    }
}*/