// Copyright 2021 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![cfg(test)]

// ----- Crates -----

use super::*;
use flexi_logger::Logger;

// ----- Types -----

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

// ----- Enumerations -----

enum LogError {
    CouldNotStartLogger,
}

// ----- Functions -----

/// Lite logging setup
fn setup_logging_lite() -> Result<(), LogError> {
    match Logger::with_env_or_str("")
        .format(flexi_logger::colored_default_format)
        .format_for_files(flexi_logger::colored_detailed_format)
        .duplicate_to_stderr(flexi_logger::Duplicate::All)
        .start()
    {
        Err(_) => Err(LogError::CouldNotStartLogger),
        Ok(_) => Ok(()),
    }
}

// ----- Tests -----

#[test]
fn invalid_index() {
    setup_logging_lite().ok();

    // Allocate a single sensor
    let mut sensors = Sensors::<1>::new().unwrap();

    // Add data to an invalid location
    assert!(sensors
        .add_test::<1, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(1, 0)
        .is_err());

    // Retrieve data from an invalid location
    assert!(sensors.get_data(1).is_err());
}

#[test]
fn not_ready() {
    setup_logging_lite().ok();

    // Allocate a single sensor
    let sensors = Sensors::<1>::new().unwrap();

    // Retrieve before sending any data
    let state = sensors.get_data(0);
    match state.clone() {
        Err(SensorError::CalibrationError(data)) => match data.cal {
            CalibrationStatus::NotReady => {
                return;
            }
            _ => {}
        },
        _ => {}
    }
    assert!(false, "Unexpected state: {:?}", state);
}

#[test]
fn sensor_missing() {
    setup_logging_lite().ok();

    // Allocate a single sensor
    let mut sensors = Sensors::<1>::new().unwrap();

    // Add a sensor value of 0
    // (needs 2 samples to finish averaging)
    // Once averaging is complete, we'll get a result
    assert!(sensors
        .add_test::<2, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
            0,
            NO_SENSOR_THRESHOLD as u16 - 1
        )
        .is_ok());
    let state = sensors.add_test::<2, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
        0,
        NO_SENSOR_THRESHOLD as u16 - 1,
    );

    match state.clone() {
        Err(SensorError::CalibrationError(data)) => match data.cal {
            CalibrationStatus::SensorMissing => {
                return;
            }
            _ => {}
        },
        _ => {}
    }
    assert!(false, "Unexpected state: {:?}", state);
}

#[test]
fn sensor_broken() {
    setup_logging_lite().ok();

    // Allocate a single sensor
    let mut sensors = Sensors::<1>::new().unwrap();

    // Add max sensor value
    // (needs 2 samples to finish averaging)
    // Once averaging is complete, we'll get a result
    assert!(sensors
        .add_test::<2, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(0, 0xFFFF,)
        .is_ok());
    let state =
        sensors.add_test::<2, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(0, 0xFFFF);

    match state.clone() {
        Err(SensorError::CalibrationError(data)) => match data.cal {
            CalibrationStatus::SensorBroken => {
                return;
            }
            _ => {}
        },
        _ => {}
    }
    assert!(false, "Unexpected state: {:?}", state);
}

#[test]
fn magnet_missing() {
    setup_logging_lite().ok();

    // Allocate a single sensor
    let mut sensors = Sensors::<1>::new().unwrap();

    // Add max sensor value
    // (needs 2 samples to finish averaging)
    // Once averaging is complete, we'll get a result
    assert!(sensors
        .add_test::<2, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
            0,
            MIN_OK_THRESHOLD as u16 - 1
        )
        .is_ok());
    let state = sensors.add_test::<2, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(
        0,
        MIN_OK_THRESHOLD as u16 - 1,
    );

    match state.clone() {
        Err(SensorError::CalibrationError(data)) => match data.cal {
            CalibrationStatus::MagnetWrongPoleOrMissing => {
                return;
            }
            _ => {}
        },
        _ => {}
    }
    assert!(false, "Unexpected state: {:?}", state);
}

fn magnet_check_calibration<const U: usize>(sensors: &mut Sensors<U>) {
    // Add two values, larger MIN_OK_THRESHOLD
    let val = MIN_OK_THRESHOLD as u16 + 2;
    // (needs 2 samples to finish averaging)
    // Once averaging is complete, we'll get a result
    assert!(sensors
        .add_test::<2, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(0, val)
        .is_ok());
    let state =
        sensors.add_test::<2, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(0, val);

    let mut test = false;
    match state.clone() {
        Ok(rval) => {
            if let Some(rval) = rval {
                if rval.raw == val {
                    test = true;
                }
            }
        }
        _ => {}
    }
    assert!(test, "Unexpected state: {:?}", state);

    // Check calibration
    let mut test = false;
    let state = sensors.get_data(0);
    match state {
        Ok(val) => {
            if val.cal == CalibrationStatus::MagnetDetected {
                test = true;
            }
        }
        _ => {}
    }
    assert!(test, "Unexpected state: {:?}", state);
}

fn magnet_check_normal<const U: usize>(sensors: &mut Sensors<U>) {
    // Add two values, larger MIN_OK_THRESHOLD
    let val = MIN_OK_THRESHOLD as u16 + 2;
    // (needs 2 samples to finish averaging)
    // Once averaging is complete, we'll get a result
    assert!(sensors
        .add_test::<2, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(0, val)
        .is_ok());
    let state =
        sensors.add_test::<2, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(0, val);

    let mut test = false;
    match state.clone() {
        Ok(rval) => {
            if let Some(rval) = rval {
                if rval.raw == val {
                    test = true;
                }
            }
        }
        _ => {}
    }
    assert!(test, "Unexpected state: {:?}", state);

    // Check calibration
    let mut test = false;
    let state = sensors.get_data(0);
    match state {
        Ok(val) => {
            if val.cal == CalibrationStatus::MagnetDetected {
                test = true;
            }
        }
        _ => {}
    }
    assert!(test, "Unexpected state: {:?}", state);
}

fn magnet_calibrate<const U: usize>(sensors: &mut Sensors<U>) {
    // Calibrate sensor
    magnet_check_calibration::<U>(sensors);

    // Check again with normal operation
    magnet_check_normal::<U>(sensors);
}

#[test]
fn magnet_detected() {
    setup_logging_lite().ok();

    // Allocate a single sensor
    let mut sensors = Sensors::<1>::new().unwrap();

    // Two sets of samples that will put the sensor into normal mode (and check both MagnetDetected
    // states)
    magnet_calibrate::<1>(&mut sensors);
}

#[test]
fn sensor_min_adjust() {
    setup_logging_lite().ok();

    // Allocate a single sensor
    let mut sensors = Sensors::<1>::new().unwrap();

    // Baseline
    magnet_check_calibration::<1>(&mut sensors);
    magnet_check_normal::<1>(&mut sensors);

    // Send a lower value than the min calibration and make sure it was set
    let old_min = sensors.get_data(0).unwrap().stats.min;
    let val = old_min - 1;

    assert!(sensors
        .add_test::<2, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(0, val)
        .is_ok());
    let state =
        sensors.add_test::<2, MIN_OK_THRESHOLD, MAX_OK_THRESHOLD, NO_SENSOR_THRESHOLD>(0, val);
    let mut test = false;
    match state.clone() {
        Ok(rval) => {
            if let Some(rval) = rval {
                if rval.raw == val {
                    test = true;
                }
            }
        }
        _ => {}
    }
    assert!(test, "Unexpected state: {:?}", state);

    // Check min calibration
    let new_min = sensors.get_data(0).unwrap().stats.min;
    assert!(val == new_min);
}
