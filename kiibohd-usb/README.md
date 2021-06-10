# kiibohd-usb

Combination USB HID interface from the kiibohd project.
Instanciates the following USB HID interfaces:
* Boot mode keyboard (supports auto-switching through SET_PROTOCOL and manual switching)
* NKRO mode keyboard
* Consumer Ctrl and System Ctrl
* Mouse
* HID-IO

## Usage

```rust
let (mut kbd_producer, mut kbd_consumer) = KBD_QUEUE.split();
let (mut mouse_producer, mut mouse_consumer) = MOUSE_QUEUE.split();
let (mut ctrl_producer, mut ctrl_consumer) = CTRL_QUEUE.split();
let (mut hidio_rx_producer, mut hidio_rx_consumer) = HIDIO_RX_QUEUE.split();
let (mut hidio_tx_producer, mut hidio_tx_consumer) = HIDIO_TX_QUEUE.split();
let usb_hid = HidInterface::new(
		usb_bus, /* UsbBusAllocator */
		HidCountryCode::NotSupported,
		kbd_consumer,
		mouse_consumer,
		ctrl_consumer,
		hidio_rx_producer,
		hidio_tx_consumer,
);

usb_hid.poll(); // Poll HID-IO
usb_hid.push(); // Push hid reports and poll HID-IO
```

See docs for more details.


## WIP

- Mouse interface not enabled yet (still some issues during allocation on atsam4s)
