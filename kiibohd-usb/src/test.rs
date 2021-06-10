// Copyright 2021 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![cfg(test)]

use crate::descriptor::{HidioReport, KeyboardNkroReport, MouseReport, SysCtrlConsumerCtrlReport};
use usbd_hid::descriptor::generator_prelude::*;

#[test]
fn test_hidio_descriptor() {
    let expected = &[
        0x06, 0x1C, 0xFF, // Usage Page (Vendor Defined 0xFF1C)
        0x0A, 0x00, 0x11, // Usage (0x1100)
        0xA1, 0x01, // Collection (Application)
        0x15, 0x00, //   Logical Minimum (0)
        0x26, 0xFF, 0x00, //   Logical Maximum (255)
        0x75, 0x08, //   Report Size (8)
        0x95, 0x40, //   Report Count (64)
        0x91,
        0x02, //   Output (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
        0x81, 0x02, //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
        0xC0, // End Collection
    ];
    //libc_print::libc_eprintln!("HIDIO: {:0X?}", HidioReport::desc());
    assert_eq!(HidioReport::desc(), expected);
}

#[test]
fn test_keyboard_nkro_descriptor() {
    let expected = &[
        0x05, 0x01, // Usage Page (Generic Desktop Ctrls)
        0x09, 0x06, // Usage (Keyboard)
        0xA1, 0x01, // Collection (Application)
        0x05, 0x08, //   Usage Page (LEDs)
        0x19, 0x01, //   Usage Minimum (Num Lock)
        0x29, 0x05, //   Usage Maximum (Kana)
        0x15, 0x00, //   Logical Minimum (0)
        0x25, 0x01, //   Logical Maximum (1)
        0x75, 0x01, //   Report Size (1)
        0x95, 0x05, //   Report Count (5)
        0x91,
        0x02, //   Output (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
        0x95, 0x03, //   Report Count (3)
        0x91,
        0x03, //   Output (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
        0x05, 0x07, //   Usage Page (Kbrd/Keypad)
        0x19, 0x01, //   Usage Minimum (0x01)
        0x29, 0xE7, //   Usage Maximum (0xE7)
        0x95, 0xE7, //   Report Count (231)
        0x81,
        0x00, //   Input (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
        0x95, 0x01, //   Report Count (1)
        0x81, 0x03, //   Input (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
        0xC0, // End Collection
    ];
    //libc_print::libc_println!("NKRO: {:02X?}", KeyboardNkroReport::desc());
    assert_eq!(KeyboardNkroReport::desc(), expected);
}

#[test]
fn test_sysctrlconsumerctrl_descriptor() {
    let expected = &[
        0x05, 0x0C, // Usage Page (Consumer)
        0x09, 0x01, // Usage (Consumer Control)
        0xA1, 0x01, // Collection (Application)
        0x05, 0x0C, //   Usage Page (Consumer)
        0x19, 0x00, //   Usage Minimum (Unassigned)
        0x2A, 0x9D, 0x02, //   Usage Maximum (0x029D)
        0x15, 0x00, //   Logical Minimum (0)
        0x27, 0xFF, 0xFF, 0x00, 0x00, //   Logical Maximum (65534)
        0x75, 0x10, //   Report Size (16)
        0x95, 0x01, //   Report Count (1)
        0x81,
        0x00, //   Input (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
        0x05, 0x01, //   Usage Page (Generic Desktop Ctrls)
        0x19, 0x81, //   Usage Minimum (Sys Power Down)
        0x29, 0xB7, //   Usage Maximum (Sys Display LCD Autoscale)
        0x15, 0x01, //   Logical Minimum (1)
        0x26, 0xFF, 0x00, //   Logical Maximum (255)
        0x75, 0x08, //   Report Size (8)
        0x81,
        0x00, //   Input (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
        0xC0, // End Collection
    ];
    //libc_print::libc_println!("SysCtrlConsumerCtrl: {:02X?}", SysCtrlConsumerCtrlReport::desc());
    assert_eq!(SysCtrlConsumerCtrlReport::desc(), expected);
}

#[test]
fn test_mouse_descriptor() {
    let expected = &[
        0x05, 0x01, // Usage Page (Generic Desktop Ctrls)
        0x09, 0x02, // Usage (Mouse)
        0xA1, 0x01, // Collection (Application)
        0x09, 0x01, //   Usage (Pointer)
        0xA1, 0x00, //   Collection (Physical)
        0x05, 0x09, //     Usage Page (Button)
        0x19, 0x01, //     Usage Minimum (0x01)
        0x29, 0x08, //     Usage Maximum (0x08)
        0x15, 0x00, //     Logical Minimum (0)
        0x25, 0x01, //     Logical Maximum (1)
        0x75, 0x01, //     Report Size (1)
        0x95, 0x08, //     Report Count (8)
        0x81,
        0x02, //     Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
        0x05, 0x01, //     Usage Page (Generic Desktop Ctrls)
        0x09, 0x30, //     Usage (X)
        0x17, 0x01, 0x80, 0xFF, 0xFF, //     Logical Minimum (-32768)
        0x26, 0xFF, 0x7F, //     Logical Maximum (32767)
        0x75, 0x10, //     Report Size (16)
        0x95, 0x01, //     Report Count (1)
        0x81,
        0x06, //     Input (Data,Var,Rel,No Wrap,Linear,Preferred State,No Null Position)
        0x09, 0x31, //     Usage (Y)
        0x81,
        0x06, //     Input (Data,Var,Rel,No Wrap,Linear,Preferred State,No Null Position)
        0x09, 0x38, //     Usage (Wheel)
        0x17, 0x81, 0xFF, 0xFF, 0xFF, //     Logical Minimum (-128)
        0x25, 0x7F, //     Logical Maximum (127)
        0x75, 0x08, //     Report Size (8)
        0x81,
        0x06, //     Input (Data,Var,Rel,No Wrap,Linear,Preferred State,No Null Position)
        0x05, 0x0C, //     Usage Page (Consumer)
        0x0A, 0x38, 0x02, //     Usage (AC Pan)
        0x81,
        0x06, //     Input (Data,Var,Rel,No Wrap,Linear,Preferred State,No Null Position)
        0xC0, //   End Collection
        0xC0, // End Collection
    ];
    //libc_print::libc_println!("Mouse: {:02X?}", MouseReport::desc());
    assert_eq!(MouseReport::desc(), expected);
}
