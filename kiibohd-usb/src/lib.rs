// Copyright 2021 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![no_std]

mod descriptor;
mod test;

pub use crate::descriptor::{
    HidioReport, KeyboardNkroReport, MouseReport, SysCtrlConsumerCtrlReport,
};
use heapless::spsc::{Consumer, Producer};
use usbd_hid::descriptor::generator_prelude::*;
use usbd_hid::descriptor::KeyboardReport;
use usbd_hid::hid_class::{HIDClass, HidClassSettings, HidProtocol, HidSubClass};
pub use usbd_hid::hid_class::{HidCountryCode, HidProtocolMode, ProtocolModeConfig};
use usbd_hid::{UsbBus, UsbBusAllocator, UsbClass};

pub enum KeyState {
    /// Press the given USB HID Keyboard code
    Press(u8),
    /// Release the given USB HID Keyboard code
    Release(u8),
    /// Clear all currently pressed USB HID Keyboard codes
    Clear,
}

pub enum MouseState {
    /// Press the given mouse button (1->8)
    Press(u8),
    /// Release the given mouse button (1->8)
    Release(u8),
    /// Position update
    Position { x: i16, y: i16 },
    /// Vertical Wheel Increment
    VertWheel(i8),
    /// Horizontal Wheel Increment
    HorzWheel(i8),
    /// Clear all mouse state
    Clear,
}

pub enum CtrlState {
    /// Press the given USB HID System Ctrl code
    SystemCtrlPress(u8),
    /// Release the given USB HID System Ctrl code
    SystemCtrlRelease(u8),
    /// Press the given USB HID Consumer Ctrl code
    ConsumerCtrlPress(u16),
    /// Release the given USB HID Consumer Ctrl code
    ConsumerCtrlRelease(u16),
    /// Clear all the currently pressed consumer and system control HID codes
    Clear,
}

#[derive(Clone, Copy, Debug)]
pub struct HidioPacket {
    pub data: [u8; 64],
}

impl HidioPacket {
    fn new() -> Self {
        Self { data: [0; 64] }
    }
}

/// USB HID Combination Interface
///
/// Handles creation and management of multiple USB HID interfaces through SPSC queues.
/// Maintains state for you so you only need to send state changes and poll events.
///
/// Typical Usage
/// - Queue up changes using SPSC queues
/// - Call push to send the current states of all the queues
///   (queues are not processed until push() is called)
///
/// HID-IO
/// - Queue up changes, or receive changes using hidio_rx and hidio_tx spsc queues
/// - Call poll to process queues in both directions (or push, which will call poll for you)
///   Will attempt to push and pull as many packets as possible in case the USB device supports
///   larger buffers (e.g. double buffering)
pub struct HidInterface<
    'a,
    B: UsbBus,
    const KBD_SIZE: usize,
    const MOUSE_SIZE: usize,
    const CTRL_SIZE: usize,
    const HIDIO_RX_SIZE: usize,
    const HIDIO_TX_SIZE: usize,
> {
    kbd_6kro: HIDClass<'a, B>,
    kbd_6kro_report: KeyboardReport,
    kbd_nkro: HIDClass<'a, B>,
    kbd_nkro_report: KeyboardNkroReport,
    kbd_consumer: Consumer<'a, KeyState, KBD_SIZE>,
    ctrl: HIDClass<'a, B>,
    ctrl_consumer: Consumer<'a, CtrlState, CTRL_SIZE>,
    ctrl_report: SysCtrlConsumerCtrlReport,
    //mouse: HIDClass<'a, B>,
    mouse_consumer: Consumer<'a, MouseState, MOUSE_SIZE>,
    mouse_report: MouseReport,
    hidio: HIDClass<'a, B>,
    hidio_rx: Producer<'a, HidioPacket, HIDIO_RX_SIZE>,
    hidio_tx: Consumer<'a, HidioPacket, HIDIO_TX_SIZE>,
}

impl<
        B: UsbBus,
        const KBD_SIZE: usize,
        const MOUSE_SIZE: usize,
        const CTRL_SIZE: usize,
        const HIDIO_RX_SIZE: usize,
        const HIDIO_TX_SIZE: usize,
    > HidInterface<'_, B, KBD_SIZE, MOUSE_SIZE, CTRL_SIZE, HIDIO_RX_SIZE, HIDIO_TX_SIZE>
{
    pub fn new<'a>(
        alloc: &'a UsbBusAllocator<B>,
        locale: HidCountryCode,
        kbd_consumer: Consumer<'a, KeyState, KBD_SIZE>,
        mouse_consumer: Consumer<'a, MouseState, MOUSE_SIZE>,
        ctrl_consumer: Consumer<'a, CtrlState, CTRL_SIZE>,
        hidio_rx: Producer<'a, HidioPacket, HIDIO_RX_SIZE>,
        hidio_tx: Consumer<'a, HidioPacket, HIDIO_TX_SIZE>,
    ) -> HidInterface<'a, B, KBD_SIZE, MOUSE_SIZE, CTRL_SIZE, HIDIO_RX_SIZE, HIDIO_TX_SIZE> {
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
        /*
        let mouse =
            HIDClass::new_ep_in(alloc, MouseReport::desc(), 10, HidClassSettings::default());
        */
        let hidio = HIDClass::new(alloc, HidioReport::desc(), 10, HidClassSettings::default());

        HidInterface {
            kbd_6kro,
            kbd_6kro_report: KeyboardReport {
                modifier: 0,
                reserved: 0,
                leds: 0,
                keycodes: [0; 6],
            },
            kbd_nkro,
            kbd_nkro_report: KeyboardNkroReport {
                leds: 0,
                keybitmap: [0; 29],
            },
            kbd_consumer,
            ctrl,
            ctrl_consumer,
            ctrl_report: SysCtrlConsumerCtrlReport {
                consumer_ctrl: 0,
                system_ctrl: 0,
            },
            //mouse,
            mouse_consumer,
            mouse_report: MouseReport {
                buttons: 0,
                x: 0,
                y: 0,
                vert_wheel: 0,
                horz_wheel: 0,
            },
            hidio,
            hidio_rx,
            hidio_tx,
        }
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
    //pub fn interfaces(&mut self) -> [&'_ mut dyn UsbClass<B>; 5] {
    pub fn interfaces(&mut self) -> [&'_ mut dyn UsbClass<B>; 4] {
        [
            &mut self.kbd_6kro,
            &mut self.kbd_nkro,
            &mut self.ctrl,
            /*
            &mut self.mouse,
            */
            &mut self.hidio,
        ]
    }

    /// Modifies the nkro report bitmask
    fn nkro_bit(&mut self, key: u8, press: bool) {
        // NOTE: The indexing actually starts from 1 (not 0), so position 0 represents 1
        //       0 in USB HID represents no keys pressed, so it's meaningless in a bitmask
        //       Ignore any keys over 231/0xE7
        if key == 0 || key > 0xE7 {
            log::warn!("Invalid key for nkro_bit({}, {}), ignored.", key, press);
            return;
        }

        let key = key - 1;

        // Determine position
        let byte: usize = (key / 8).into();
        let bit: usize = (key % 8).into();

        // Set/Unset
        if press {
            self.kbd_nkro_report.keybitmap[byte] |= 1 << bit;
        } else {
            self.kbd_nkro_report.keybitmap[byte] &= !(1 << bit);
        }
    }

    fn update_kbd(&mut self) -> bool {
        let mut updated = false;

        // Empty kbd queue
        loop {
            match self.kbd_consumer.dequeue() {
                Some(state) => {
                    updated = true;
                    match state {
                        KeyState::Press(key) => {
                            // Ignore 0
                            // - 6KRO -
                            // Modifiers
                            if key & 0xE0 == 0xE0 {
                                self.kbd_6kro_report.modifier |= 1 << (key ^ 0xE0);
                                // Left shift 1 by key XOR 0xE0
                            }
                            // Keys
                            for pos in self.kbd_6kro_report.keycodes.iter_mut() {
                                // Check to see if key is already presed
                                if *pos == key {
                                    break;
                                }
                                // Set the key if we encounter a 0 (no key set)
                                if *pos == 0 {
                                    *pos = key;
                                    break;
                                }
                            }

                            // - NKRO -
                            self.nkro_bit(key, true);
                        }
                        KeyState::Release(key) => {
                            // - 6KRO -
                            // Modifiers
                            if key & 0xE0 == 0xE0 {
                                self.kbd_6kro_report.modifier |= 1 << (key ^ 0xE0);
                                // Left shift 1 by key XOR 0xE0
                            }
                            // Keys
                            if key != 0 {
                                // Check to see if key is pressed
                                if let Some(index) =
                                    self.kbd_6kro_report.keycodes.iter().position(|&k| k == key)
                                {
                                    // Rotate in all the keys
                                    // OSs will skip all the keys after the first 0 is found in
                                    // the array.
                                    self.kbd_6kro_report.keycodes[index..].rotate_left(1);
                                    // Clear the last index
                                    self.kbd_6kro_report.keycodes
                                        [self.kbd_6kro_report.keycodes.len() - 1] = 0;
                                }
                            }

                            // - NKRO -
                            self.nkro_bit(key, false);
                        }
                        KeyState::Clear => {
                            // - 6KRO -
                            self.kbd_6kro_report.modifier = 0;
                            self.kbd_6kro_report.keycodes = [0; 6];

                            // - NKRO -
                            self.kbd_nkro_report.keybitmap = [0; 29];
                        }
                    }
                }
                None => {
                    return updated;
                }
            }
        }
    }

    fn push_6kro_kbd(&mut self) {
        if let Err(val) = self.kbd_6kro.push_input(&self.kbd_6kro_report) {
            log::error!("6KRO Buffer Overflow: {:?}", val);
        }
    }

    fn push_nkro_kbd(&mut self) {
        if let Err(val) = self.kbd_nkro.push_input(&self.kbd_nkro_report) {
            log::error!("NKRO Buffer Overflow: {:?}", val);
        }
    }

    fn mouse_button_bit(&mut self, button: u8, press: bool) {
        // Ignore keys outside of 1 to 8
        if let 1..=8 = button {
            let button = button - 1;
            // Determine position
            let bit: usize = (button % 8).into();

            // Set/Unset
            if press {
                self.mouse_report.buttons |= 1 << bit;
            } else {
                self.mouse_report.buttons &= !(1 << bit);
            }
        }
    }

    fn push_mouse(&mut self) {
        let mut updated = false;

        // Empty mouse queue
        while let Some(state) = self.mouse_consumer.dequeue() {
            updated = true;
            match state {
                MouseState::Press(key) => {
                    self.mouse_button_bit(key, true);
                }
                MouseState::Release(key) => {
                    self.mouse_button_bit(key, false);
                }
                MouseState::Position { x, y } => {
                    self.mouse_report.x = x;
                    self.mouse_report.y = y;
                }
                MouseState::VertWheel(pos) => {
                    self.mouse_report.vert_wheel = pos;
                }
                MouseState::HorzWheel(pos) => {
                    self.mouse_report.horz_wheel = pos;
                }
                MouseState::Clear => {
                    self.mouse_report.buttons = 0;
                }
            }
        }

        // Push report
        if updated {
            /*
            if let Err(val) = self.mouse.push_input(&self.mouse_report) {
                log::error!("Mouse Buffer Overflow: {:?}", val);
            }
            */
        }

        // Clear relative fields
        self.mouse_report.x = 0;
        self.mouse_report.y = 0;
        self.mouse_report.vert_wheel = 0;
        self.mouse_report.horz_wheel = 0;
    }

    fn push_ctrl(&mut self) {
        let mut updated = false;

        // Empty ctrl queue
        while let Some(state) = self.ctrl_consumer.dequeue() {
            updated = true;
            match state {
                CtrlState::SystemCtrlPress(key) => {
                    self.ctrl_report.system_ctrl = key;
                }
                CtrlState::SystemCtrlRelease(_key) => {
                    self.ctrl_report.system_ctrl = 0;
                }
                CtrlState::ConsumerCtrlPress(key) => {
                    self.ctrl_report.consumer_ctrl = key;
                }
                CtrlState::ConsumerCtrlRelease(_key) => {
                    self.ctrl_report.consumer_ctrl = 0;
                }
                CtrlState::Clear => {
                    self.ctrl_report.consumer_ctrl = 0;
                    self.ctrl_report.system_ctrl = 0;
                }
            }
        }

        // Push report
        if updated {
            if let Err(val) = self.ctrl.push_input(&self.ctrl_report) {
                log::error!("Ctrl Buffer Overflow: {:?}", val);
            }
        }
    }

    /// Processes each of the spsc queues and pushes data over USB
    /// This is primarily for keyboard, mouse and ctrl interfaces.
    /// HID-IO is pushed more frequently depending on USB interrupts.
    pub fn push(&mut self) {
        // Update keyboard if necessary
        if self.update_kbd() {
            // Check protocol mode to decide nkro vs. 6kro (boot)
            match self.get_kbd_protocol_mode() {
                HidProtocolMode::Report => {
                    self.push_nkro_kbd();
                }
                HidProtocolMode::Boot => {
                    self.push_6kro_kbd();
                }
            }
        }

        // Push consumer and system control reports
        self.push_ctrl();

        // Push mouse reports
        self.push_mouse();

        // Push any pending hidio reports
        self.poll();
    }

    /// Poll the HID-IO interface
    pub fn poll(&mut self) {
        // Check for any incoming packets
        while self.hidio_rx.ready() {
            let mut packet = HidioPacket::new();
            match self.hidio.pull_raw_output(&mut packet.data) {
                Ok(_size) => {
                    self.hidio_rx.enqueue(packet).unwrap();
                }
                Err(_) => {
                    break;
                }
            }
        }

        // Push as many packets as possible
        while self.hidio_tx.ready() {
            // Don't dequeue yet, we might not be able to send
            let packet = self.hidio_tx.peek().unwrap();

            // Attempt to push
            match self.hidio.push_raw_input(&packet.data) {
                Ok(_size) => {
                    // Dequeue
                    self.hidio_tx.dequeue().unwrap();
                }
                Err(_) => {
                    break;
                }
            }
        }
    }
}
