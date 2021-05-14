// Copyright 2021 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// ----- Crates -----

// This forces rust to import all the symbols from kiibohd-hid-io
// and include them into libkiibohd_core.a
#[allow(unused_imports)]
pub use kiibohd_keyscanning::*;