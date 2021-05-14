// Copyright 2021 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![no_std]
#![feature(lang_items)]

// ----- Crates -----

// ----- Modules -----

pub mod he;
pub mod hidio;
pub mod keyscanning;

// ----- Embedded Functionality -----

#[cfg(feature = "lib")]
use core::panic::PanicInfo;

#[cfg(feature = "lib")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// ----- Host Functionality -----

#[cfg(feature = "lib")]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
