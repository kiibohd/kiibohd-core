// Copyright 2021 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![no_std]

pub use log;

#[cfg(feature = "semihosting")]
pub use cortex_m_semihosting;

#[cfg(feature = "rtt")]
pub use rtt_target;

/// Logger Structure
///
/// Example Usage
/// ```
/// use kiibohd_rtt_log::{log, Logger, rtt_target, setup_rtt};
/// use rtt_target::{rtt_init_default, set_print_channel};
///
/// static LOGGER: Logger = Logger::new(log::LevelFilter::Info);
///
/// fn main() {
///     // Setup RTT logging
///     let channels = rtt_init_default!();
///     set_print_channel(channels.up.0);
///     log::set_logger(&LOGGER).unwrap();
///
///     // Example usage
///     log::trace!("Trace message");
///     log::debug!("Debug message");
///     log::info!("Info message");
///     log::warn!("Warn message");
///     log::error!("Error message");
///
///     // Read downchannel
///     let mut buf = [0u8; 16];
///     downchannel.read(&mut buf[..]);
///     log::trace!("{}", buf);
/// }
/// ```
pub struct Logger {
    level_filter: log::LevelFilter,
}

impl Logger {
    pub const fn new(level_filter: log::LevelFilter) -> Self {
        Self { level_filter }
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.level_filter.ge(&metadata.level())
    }

    // Handle entry prefixes
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let color = match record.level() {
                log::Level::Error => "1;5;31",
                log::Level::Warn => "1;33",
                log::Level::Info => "1;32",
                log::Level::Debug => "1;35",
                log::Level::Trace => "1;90",
            };
            let dwt = unsafe { &*cortex_m::peripheral::DWT::ptr() };
            #[cfg(feature = "rtt")]
            rtt_target::rprintln!(
                "{:10}:\x1b[{}m{:5}\x1b[0m:{}",
                dwt.get_cycle_count(),
                color,
                record.level(),
                record.args()
            );
            #[cfg(feature = "semihosting")]
            cortex_m_semihosting::hprintln!(
                "{:10}:\x1b[{}m{:5}\x1b[0m:{}",
                dwt.get_cycle_count(),
                color,
                record.level(),
                record.args()
            )
            .ok();
            /* TODO (HaaTa) Add itm support
            #[cfg(feature = "itm")]
            {
                let itm = unsafe { &mut *cortex_m::peripheral::ITM::ptr() };
                let stim = &mut itm.stim[0];
                cortex_m::iprintln!(stim,
                    "{}:\x1b[{}m{}\x1b[0m - {}",
                    dwt.cyccnt.read(),
                    color,
                    record.level(),
                    record.args()
                );
            }
            */
        }
    }

    fn flush(&self) {}
}
