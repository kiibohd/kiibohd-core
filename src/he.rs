// Copyright 2021 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// ----- Modules -----

// ----- Crates -----

pub use kiibohd_hall_effect::{
    CalibrationStatus, SenseAnalysis, SenseData, SenseStats, SensorError, Sensors,
};

// ----- Types -----

const NUM_SCAN_CODES: usize = 115;
const SENSE_ACCUMULATION: usize = 2;

// --- NOTE ---
// These thresholds were calculated on a Keystone v1.00 TKL pcb

// Calibration Mode Thresholds
const MIN_OK_THRESHOLD: usize = 1350;
// U1350 - b10101000110 - Switch not pressed (not 100% guaranteed, but the minimum range we can work withA
// Some sensors will have default values up to 1470 without any magnet and that is within the specs
// of the datasheet.

const MAX_OK_THRESHOLD: usize = 2500;
// U2500 - b100111000100 - Switch fully pressed

const NO_SENSOR_THRESHOLD: usize = 1000;
// Likely invalid ADC level from non-existent sensor (or very low magnet)

// ----- Globals -----

static mut INTF: Option<Sensors<NUM_SCAN_CODES>> = None;

// ----- External C Callbacks -----

extern "C" {}

// ----- External C Interface -----

#[repr(C)]
#[derive(PartialEq)]
pub enum HeStatus {
    Success,
    AnalysisReady,
    ErrorInvalidIndex,
    ErrorInvalidReading,
    ErrorMagnetWrongPoleOrMissing,
    ErrorNotInitialized,
    ErrorSensorBroken,
    ErrorSensorMissing,
    ErrorSensorNotReady,
    ErrorUnknown,
}

/// Initialize the hall effect interface
/// Must be called first, before any other he_ commands.
#[no_mangle]
pub extern "C" fn he_init() -> HeStatus {
    unsafe {
        INTF = Some(match Sensors::<NUM_SCAN_CODES>::new() {
            Ok(intf) => intf,
            Err(_) => {
                return HeStatus::ErrorUnknown;
            }
        });
    }
    HeStatus::Success
}

/// Processes an ADC event for the given index (scan code location)
/// This should be the raw value from the ADC
/// Once enough values have been accumulated, data analysis is done
/// automatically
/// Uses normal mode
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn he_scan_event(
    index: u16,
    val: u16,
    analysis: *mut SenseAnalysis,
) -> HeStatus {
    // Retrieve interface
    let intf = match INTF.as_mut() {
        Some(intf) => intf,
        None => {
            return HeStatus::ErrorNotInitialized;
        }
    };

    match intf.add::<SENSE_ACCUMULATION>(index as usize, val) {
        Ok(data) => {
            if let Some(data) = data {
                *analysis = data.clone();
                HeStatus::AnalysisReady
            } else {
                HeStatus::Success
            }
        }
        Err(err) => match err {
            SensorError::CalibrationError(data) => match data.cal {
                CalibrationStatus::NotReady => HeStatus::ErrorSensorNotReady,
                _ => HeStatus::ErrorUnknown,
            },
            SensorError::InvalidSensor(_) => HeStatus::ErrorInvalidIndex,
            _ => HeStatus::ErrorUnknown,
        },
    }
}

/// Processes an ADC event for the given index (scan code location)
/// This should be the raw value from the ADC
/// Once enough values have been accumulated, data analysis is done
/// automatically
/// Uses test mode
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn he_test_event(
    index: u16,
    val: u16,
    analysis: *mut SenseAnalysis,
) -> HeStatus {
    // Retrieve interface
    let intf = match INTF.as_mut() {
        Some(intf) => intf,
        None => {
            return HeStatus::ErrorNotInitialized;
        }
    };

    match intf
        .add_test::<SENSE_ACCUMULATION, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
            index as usize,
            val,
        ) {
        Ok(data) => {
            if let Some(data) = data {
                *analysis = data.clone();
                HeStatus::AnalysisReady
            } else {
                HeStatus::Success
            }
        }
        Err(err) => match err {
            SensorError::CalibrationError(data) => match data.cal {
                CalibrationStatus::MagnetWrongPoleOrMissing => {
                    HeStatus::ErrorMagnetWrongPoleOrMissing
                }
                CalibrationStatus::NotReady => HeStatus::ErrorSensorNotReady,
                CalibrationStatus::SensorBroken => HeStatus::ErrorSensorBroken,
                CalibrationStatus::SensorMissing => HeStatus::ErrorSensorMissing,
                _ => HeStatus::ErrorUnknown,
            },
            SensorError::InvalidSensor(_) => HeStatus::ErrorInvalidIndex,
            _ => HeStatus::ErrorUnknown,
        },
    }
}
/// Retrieve calibration status
#[no_mangle]
pub extern "C" fn he_calibration(index: u16) -> CalibrationStatus {
    unsafe {
        // Retrieve interface
        let intf = match INTF.as_mut() {
            Some(intf) => intf,
            None => {
                return CalibrationStatus::NotReady;
            }
        };

        match intf.get_data(index as usize) {
            Ok(results) => results.cal.clone(),
            Err(_) => CalibrationStatus::InvalidIndex,
        }
    }
}

/// Retrieve analysis data
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn he_analysis(index: u16, analysis: *mut SenseAnalysis) -> HeStatus {
    // Retrieve interface
    let intf = match INTF.as_mut() {
        Some(intf) => intf,
        None => {
            return HeStatus::ErrorNotInitialized;
        }
    };

    match intf.get_data(index as usize) {
        Ok(results) => {
            *analysis = results.analysis.clone();
            HeStatus::Success
        }
        Err(_) => HeStatus::ErrorInvalidIndex,
    }
}

/// Retrieve stats data
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn he_stats(index: u16, stats: *mut SenseStats) -> HeStatus {
    // Retrieve interface
    let intf = match INTF.as_mut() {
        Some(intf) => intf,
        None => {
            return HeStatus::ErrorNotInitialized;
        }
    };

    match intf.get_data(index as usize) {
        Ok(results) => {
            *stats = results.stats.clone();
            HeStatus::Success
        }
        Err(_) => HeStatus::ErrorInvalidIndex,
    }
}
