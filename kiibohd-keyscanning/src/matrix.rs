#[allow(unused_imports)]

pub mod state;
pub use self::state::KeyState;
pub use self::state::State;
pub use self::state::StateReturn;

use core::convert::Infallible;
use atsam4_hal::{prelude::*, gpio::*, InputPin, OutputPin};
use embedded_time::{duration::*, rate::*};
use heapless::Vec;
use keyberon::matrix::{HeterogenousArray};
use generic_array::{ArrayLength, GenericArray};

/*
pub struct RowArray { // An array of the input pin's for each row, and a count of how full the array is
    pub rows: [&'static dyn Input<PullDown>; 6],
    pub rowcnt: usize,
}

impl RowArray {
    pub fn new(rows: [&'static dyn Input<PullDown>; 6], rowcnt: usize) -> RowArray {
        RowArray {
            rows: rows,
            rowcnt: rowcnt,
        }
    }
}

pub struct ColArray { // An array of the output pin's for each column, and a count of how full the array is
    pub cols: [&'static mut dyn Output<PushPull>; 17],
    pub colcnt: usize,
}

impl ColArray {
    pub fn new(cols: [&'static mut dyn Output<PushPull>; 17], colcnt: usize) -> ColArray{
        ColArray {
            cols: cols,
            colcnt: colcnt,
        }
    }
}*/

pub struct Matrix<C, R> { // The matrix of inputs, and outputs, and the state of each key
    pub rows: R,
    pub cols: C,
    pub state_matrix: StateMatrix,
}

impl<C, R> Matrix<C, R> {
    pub fn new<E>(cols: C, rows: R, scan_period: Microseconds) -> Result<Self, E>
    where
        for<'a> &'a mut C: IntoIterator<Item = &'a mut dyn OutputPin<Error = E>>,
    {
        let state_matrix = StateMatrix::new(5_u32.milliseconds(), 500_u32.milliseconds(), 700_u32.milliseconds(), scan_period);
        let mut res = Self { cols, rows, state_matrix };
        res.clear()?;
        Ok(res)
    }
    pub fn clear<'a, E: 'a>(&'a mut self) -> Result<(), E>
    where
        &'a mut C: IntoIterator<Item = &'a mut dyn OutputPin<Error = E>>,
    {
        for c in self.cols.into_iter() {
            c.set_low()?;
        }
        Ok(())
    }
    pub fn get<'a, E: 'a>(&'a mut self) -> Result<(), E>
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
        let _stuff = self.cols
            .into_iter()
            .enumerate()
            .map(|(i, c)| {
                match c.set_high() {
                    Ok(_) => {}
                    Err(_e) => {}
                }
                rows
                    .into_iter()
                    .enumerate()
                    .map(|(j, r)| {
                        match r.is_high() {
                            Ok(t) => {state_matrix.poll_update(j, i, t);}
                            Err(_e) => {}
                        }
                    });
                match c.set_low() {
                    Ok(_) => {}
                    Err(_e) => {}
                }
            });
        Ok(())
    }
}

/// The matrix to keep all the key states and handle state updating
pub struct StateMatrix {
    keys: [[KeyState; 7]; 20],
}

impl StateMatrix {

    pub fn new(bounce_limit: Milliseconds, held_limit: Milliseconds, idle_limit: Milliseconds, scan_period: Microseconds) -> StateMatrix {
        StateMatrix { // Create a two dimensional array of key states with a debounce delay of 5ms, a hold time of 5ms, and an idle limit of 500ms
            keys: [[KeyState::new(bounce_limit, held_limit, idle_limit, scan_period); 7]; 20],
        }
    }

    pub fn poll_update(&mut self, r: usize, c: usize, high: bool) -> bool {
        let _change = KeyState::poll_update(&mut self.keys[r][c], high);
        false
    }

    /*pub fn events<'a, U>(&'a mut self, new: T) -> impl Iterator<Item = Event> + 'a
    where
        &'a T: IntoIterator<Item = U>,
        U: IntoIterator<Item = &'a bool>,
        U::IntoIter: 'a,
    {



        if self.poll_update()

        if self.update(new) {
            Left(
                self.new
                    .into_iter()
                    .zip(self.cur.into_iter())
                    .enumerate()
                    .flat_map(move |(i, (o, n))| {
                        o.into_iter().zip(n.into_iter()).enumerate().filter_map(
                            move |(j, bools)| match bools {
                                (false, true) => {
                                    Some(Event::Press(i.try_into().unwrap(), j.try_into().unwrap()))
                                }
                                (true, false) => Some(Event::Release(
                                    i.try_into().unwrap(),
                                    j.try_into().unwrap(),
                                )),
                                _ => None,
                            },
                        )
                    }),
            )
        } else {
            Right(core::iter::empty())
        }*/

    pub fn get_state(&self, r: usize, c: usize) -> State {
        KeyState::get_state(&self.keys[r][c])
    }

}

/*
impl<C, R> Matrix<C, R> {
    pub fn new<E>(cols: C, rows: R) -> Result<Self, E>
    where
        for<'a> &'a mut C: IntoIterator<Item = &'a mut dyn OutputPin<Error = E>>,
    {
        let mut res = Self { cols, rows };
        res.clear()?;
        Ok(res)
    }
    pub fn clear<'a, E: 'a>(&'a mut self) -> Result<(), E>
    where
        &'a mut C: IntoIterator<Item = &'a mut dyn OutputPin<Error = E>>,
    {
        for c in self.cols.into_iter() {
            c.set_high()?;
        }
        Ok(())
    }
    pub fn get<'a, E: 'a>(&'a mut self) -> Result<PressedKeys<R::Len, C::Len>, E>
    where
        &'a mut R: IntoIterator<Item = &'a mut dyn OutputPin<Error = E>>,
        R: HeterogenousArray,
        R::Len: ArrayLength<GenericArray<bool, C::Len>>,
        R::Len: heapless::ArrayLength<GenericArray<bool, C::Len>>,
        &'a C: IntoIterator<Item = &'a dyn InputPin<Error = E>>,
        C: HeterogenousArray,
        C::Len: ArrayLength<bool>,
        C::Len: heapless::ArrayLength<bool>,
    {
        let cols = &self.cols;
        self.rows
            .into_iter()
            .map(|r| {
                r.set_low()?;
                let col = cols
                    .into_iter()
                    .map(|c| c.is_low())
                    .collect::<Result<Vec<_, C::Len>, E>>()?
                    .into_iter()
                    .collect();
                r.set_high()?;
                Ok(col)
            })
            .collect::<Result<Vec<_, R::Len>, E>>()
            .map(|res| PressedKeys(res.into_iter().collect()))
    }
}*/


#[derive(Default, PartialEq, Eq)]
pub struct PressedKeys<U, V>(pub GenericArray<GenericArray<bool, V>, U>)
where
    V: ArrayLength<bool>,
    U: ArrayLength<GenericArray<bool, V>>;

impl<U, V> PressedKeys<U, V>
where
    V: ArrayLength<bool>,
    U: ArrayLength<GenericArray<bool, V>>,
{
    pub fn iter_pressed(&self) -> impl Iterator<Item = (usize, usize)> + Clone + '_ {
        self.0.iter().enumerate().flat_map(|(i, r)| {
            r.iter()
                .enumerate()
                .filter_map(move |(j, &b)| if b { Some((i, j)) } else { None })
        })
    }
}

impl<'a, U, V> IntoIterator for &'a PressedKeys<U, V>
where
    V: ArrayLength<bool>,
    U: ArrayLength<GenericArray<bool, V>>,
    U: ArrayLength<&'a GenericArray<bool, V>>,
{
    type IntoIter = core::slice::Iter<'a, GenericArray<bool, V>>;
    type Item = &'a GenericArray<bool, V>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}