#![no_main]
#![no_std]

use core::convert::Infallible;
use panic_halt as _;

pub enum KeyStates {
    Pressed,
    Released
}

pub struct KeyState {
    state: KeyStates,
    prev_state: KeyStates,
    pressed_dur: i32,
}

impl KeyState {
    pub fn 
}