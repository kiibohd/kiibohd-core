// Copyright 2021 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![no_std]

use usbd_hid::descriptor::generator_prelude::*;
use usbd_hid::descriptor::KeyboardReport;
use usbd_hid::hid_class::{HIDClass, HidClassSettings, HidProtocol, HidSubClass};
pub use usbd_hid::hid_class::{HidCountryCode, HidProtocolMode, ProtocolModeConfig};
use usbd_hid::{UsbBus, UsbBusAllocator, UsbClass};

pub struct HidInterface<'a, B: UsbBus> {
    kbd_6kro: HIDClass<'a, B>,
    kbd_nkro: HIDClass<'a, B>,
    ctrl: HIDClass<'a, B>,
    mouse: HIDClass<'a, B>,
    hidio: HIDClass<'a, B>,
}

impl<B: UsbBus> HidInterface<'_, B> {
    pub fn new(alloc: &UsbBusAllocator<B>, locale: HidCountryCode) -> HidInterface<'_, B> {
        let kbd_6kro = HIDClass::new_ep_in(
            alloc,
            KeyboardReport::desc(),
            10,
            HidClassSettings {
                subclass: HidSubClass::Boot,
                protocol: HidProtocol::Keyboard,
                config: ProtocolModeConfig::DefaultBehavior,
                locale,
            },
        );
        let kbd_nkro = HIDClass::new_ep_in(
            alloc,
            KeyboardNkroReport::desc(),
            10,
            HidClassSettings {
                subclass: HidSubClass::NoSubClass,
                protocol: HidProtocol::Keyboard,
                config: ProtocolModeConfig::DefaultBehavior,
                locale,
            },
        );
        let ctrl = HIDClass::new_ep_in(
            alloc,
            SysCtrlConsumerCtrlReport::desc(),
            10,
            HidClassSettings::default(),
        );
        let mouse =
            HIDClass::new_ep_in(alloc, MouseReport::desc(), 10, HidClassSettings::default());
        let hidio = HIDClass::new(alloc, HidioReport::desc(), 10, HidClassSettings::default());
        HidInterface {
            kbd_6kro,
            kbd_nkro,
            ctrl,
            mouse,
            hidio,
        }
    }

    pub fn push(&self) {
        self.kbd_6kro.push_raw_input(&[0, 0, 4, 0, 0, 0, 0, 0]);
    }

    /// Dynamically update the keyboard protocol mode (and behavior)
    /// Used to force NKRO or 6KRO regardless of what the host configures
    pub fn set_kbd_protocol_mode(&mut self, mode: HidProtocolMode, config: ProtocolModeConfig) {
        log::trace!(
            "HidInterface::set_kbd_protocol_mode({:?}, {:?})",
            mode,
            config
        );
        self.kbd_6kro.set_protocol_mode(mode, config).ok();
        self.kbd_nkro.set_protocol_mode(mode, config).ok();
    }

    /// Retrieves the current protocol mode
    /// Uses the 6kro keyboard (both HID Classes should return the same value)
    pub fn get_kbd_protocol_mode(&self) -> HidProtocolMode {
        let mode = self.kbd_6kro.get_protocol_mode().unwrap();
        log::trace!("HidInterface::get_kbd_protocol_mode() -> {:?}", mode);
        mode
    }

    /// Used to pass all of the interfaces to usb_dev.poll()
    pub fn interfaces(&mut self) -> [&'_ mut dyn UsbClass<B>; 5] {
        [
            &mut self.kbd_6kro,
            &mut self.kbd_nkro,
            &mut self.mouse,
            &mut self.ctrl,
            &mut self.hidio,
        ]
    }

    /// Poll each HID interface
    /// TODO Needed?
    pub fn poll(&self) {}
}

/// NKRO Keyboard - HID Bitmap
///
/// This is a simplified NKRO descriptor as comparied to kiibohd/controller.
/// It uses 1 extra byte in each packet, but easier to understand and parse.
///
/// NOTES:
/// Supports all keys defined by the spec.
/// 0 represents "no keys pressed" so it is excluded.
/// Supports all keys defined by the spec, except 1-3 which define error events
///  and 0 which is "no keys pressed"
/// See https://usb.org/sites/default/files/hut1_22.pdf Chapter 10
///
/// Special bits:
/// 0x00 - Reserved (represents no keys pressed, not useful in a bitmap)
/// 0x01 - ErrorRollOver
/// 0x02 - POSTFail
/// 0x03 - ErrorUndefined
/// 0xA5..0xAF - Reserved
/// 0xDE..0xDF - Reserved
/// 0xE8..0xFFFF - Not specified (Reserved in protocol)
///
/// Compatibility Notes:
///  - Using a second endpoint for a boot mode device helps with compatibility
///  - DO NOT use Padding in the descriptor for bitfields
///    (Mac OSX silently fails... Windows/Linux work correctly)
///  - DO NOT use Report IDs (to split the keyboard report), Windows 8.1 will not update
///    keyboard correctly (modifiers disappear)
///    (all other OSs, including OSX work fine...)
///    (you can use them *iff* you only have 1 per collection)
///  - Mac OSX and Windows 8.1 are extremely picky about padding
#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = KEYBOARD) = {
        // LED Report
        (usage_page = LEDS, usage_min = 0x01, usage_max = 0x05) = {
            #[packed_bits 5] #[item_settings data,variable,absolute] leds=output;
        };

        // 1-231 (29 bytes/231 bits)
        (usage_page = KEYBOARD, usage_min = 0x01, usage_max = 0xE7) = {
            #[packed_bits 231] #[item_settings data,array,absolute] keybitmap=input;
        };
    }
)]
#[allow(dead_code)]
pub struct KeyboardNkroReport {
    pub leds: u8,
    pub keybitmap: [u8; 29],
}

/// System Control and Consumer Control
#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = CONSUMER, usage = CONSUMER_CONTROL) = {
        // Consumer Control Collection - Media Keys (16 bits)
        //
        // NOTES:
        // Not bothering with NKRO for this table. If there's a need, I can implement it. -HaaTa
        // Using a 1KRO scheme
        (usage_page = CONSUMER, usage_min = 0x00, usage_max = 0x29D) = {
            #[item_settings data,array,absolute,not_null] consumer_ctrl=input;
        };

        // System Control Collection (8 bits)
        //
        // NOTES:
        // Not bothering with NKRO for this table. If there's need, I can implement it. -HaaTa
        // Using a 1KRO scheme
        // XXX (HaaTa):
        //  Logical Minimum must start from 1 (not 0!) to resolve MS Windows issues
        //  Usage Minimum must start from 129 (0x81) to fix macOS scrollbar issues
        (usage_page = GENERIC_DESKTOP, usage_min = 0x81, usage_max = 0xB7, logical_min = 1) = {
            #[item_settings data,array,absolute,not_null] system_ctrl=input;
        };
    }
)]
#[allow(dead_code)]
pub struct SysCtrlConsumerCtrlReport {
    pub consumer_ctrl: u16,
    pub system_ctrl: u8,
}

/// Mouse Interface
/// MouseReport describes a report and its companion descriptor than can be used
/// to send mouse movements and button presses to a host.
#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = MOUSE) = {
        (collection = PHYSICAL, usage = POINTER) = {
            (usage_page = BUTTON, usage_min = BUTTON_1, usage_max = BUTTON_8) = {
                #[packed_bits 8] #[item_settings data,variable,absolute] buttons=input;
            };
            (usage_page = GENERIC_DESKTOP,) = {
                (usage = X,) = {
                    #[item_settings data,variable,relative] x=input;
                };
                (usage = Y,) = {
                    #[item_settings data,variable,relative] y=input;
                };
                (usage = WHEEL,) = {
                    #[item_settings data,variable,relative] vert_wheel=input;
                };
            };
            (usage_page = CONSUMER, usage = AC_PAN,) = {
                #[item_settings data,variable,relative] horz_wheel=input;
            };
        };
    }
)]
#[allow(dead_code)]
pub struct MouseReport {
    pub buttons: u8,
    pub x: i16,
    pub y: i16,
    pub vert_wheel: i8, // Scroll down (negative) or up (positive) this many units
    pub horz_wheel: i8, // Scroll left (negative) or right (positive) this many units
}

/// HID-IO Interface
#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = 0xFF1C, usage = 0x1100) = {
        rx=output;
        tx=input;
    }
)]
struct HidioReport {
    rx: [u8; 64],
    tx: [u8; 32], // TODO This won't work until it can be 64 bytes
}
